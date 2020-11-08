pub use super::*;
pub use lib::hrtf_data::*;
pub use model::*;
use std::cell::{RefCell, RefMut};
use std::collections::VecDeque;
use std::sync::Arc;
extern crate crossbeam;

#[derive(Debug, Clone)]
pub struct Options {
    room: Room,
    speaker: Speaker,
    receivers: Vec<Receiver>,
    max_dist: Float,
    min_intensity: Float,
}
#[derive(Debug, Clone)]
pub struct RayNodes<Context, Callback: 'static>
where
    Callback: Fn(RefMut<Context>, ReceiveEvent),
{
    list: RefCell<VecDeque<RayWithAttributes>>,
    options: Options,
    context: RefCell<Context>,
    on_receive: std::sync::Arc<Callback>,
}
#[derive(Debug, Clone)]
pub struct ReceiveEvent {
    pub dist: Float,
    pub intensity: Float,
    pub vec: Vector,
}
#[derive(Debug, Clone)]
pub struct AdjustResult {
    pub data: u32,
    //data_ratio:Float,
    pub intensity: Float,
    pub delay: Float,
}
pub trait ThreadContext<'a, T> {
    fn new(opt: &T) -> Self;
}
impl Options {
    pub fn default() -> Self {
        Options {
            room: Room::new(),
            max_dist: 1e2,
            min_intensity: 1e-1,
            speaker: Speaker::new(),
            receivers: vec![],
        }
    }
}

pub fn multi_simulate<
    'a,
    T: Send + Sync,
    Context: ThreadContext<'a, T> + Send + Sync,
    Callback: 'static + Send + Sync,
>(
    max_threads: u32,
    on_receive: Callback,
    options: Options,
    ctx_opt: T,
) -> Vec<Context>
where
    Callback: Fn(RefMut<Context>, ReceiveEvent),
{
    if max_threads == 0 {
        panic!("thread max number must not be 0.");
    }
    let result = crossbeam::scope(|scope| {
        let mut threads = vec![];
        let callback = std::sync::Arc::new(on_receive);
        let mut contexts: Vec<Context> = vec![];
        for i in 0..max_threads {
            let opt = options.clone();
            let cb = callback.clone();
            let context = Context::new(&ctx_opt);
            let handle = scope.spawn(move |_| {
                let node = RayNodes::new(context, cb, opt);
                node.first_by(i, max_threads);
                node.simulate();
                node.context.into_inner()
            });
            threads.push(handle);
        }
        for thread in threads {
            let result = thread.join();
            if let Ok(context) = result {
                contexts.push(context);
            } else {
                panic!("at least 1 thread finished abnormally");
            }
        }
        contexts
    });
    if let Ok(contexts) = result {
        contexts
    } else {
        panic!("finished erroneously")
    }
}

impl<Context, Callback: 'static + Send + Sync> RayNodes<Context, Callback>
where
    Callback: Fn(RefMut<Context>, ReceiveEvent),
{
    pub fn new(
        context: Context,
        on_receive: Arc<Callback>,
        options: Options,
    ) -> RayNodes<Context, Callback> {
        RayNodes {
            list: RefCell::new(VecDeque::new()),
            options,
            context: RefCell::new(context),
            on_receive,
        }
    }
    pub fn receive_from(
        &self,
        pos: Vector,
        norm: Vector,
        phi_max: Float,
        intensity: Float,
        prev_dist: Float,
    ) {
        let cos_phi_max = phi_max.cos();

        for receiver in self.options.receivers.iter() {
            let d = receiver.pos - pos;
            let dist = d.norm2().sqrt();
            let d_normed = d / dist;
            let alpha = Vector::dot(d_normed, norm);
            if alpha <= 0.0 {
                continue;
            }
            let beta = (phi_max + receiver.r / dist).cos();
            if alpha >= beta {
                // (Pi*r^2) / (2*PI*dist*dist*(1-cos()))
                // r^2 / (2*dist^2*(1-cos()))
                let intensity_ratio =
                    receiver.r * receiver.r / (2.0 * dist * dist * (1.0 - cos_phi_max));
                (self.on_receive)(
                    self.context.borrow_mut(),
                    ReceiveEvent {
                        dist: prev_dist + dist,
                        intensity: intensity * intensity_ratio,
                        vec: d_normed,
                    },
                );
            }
        }
    }
    pub fn first_by(&self, num: u32, max_num: u32) {
        let speaker = self.options.speaker.clone();
        let total = speaker.theta_resolution * speaker.phi_resolution;
        let min = total * num / max_num;
        let max = total * (num + 1) / max_num;
        let norm = speaker.direction;
        let c_x = Self::orthogonal(norm);
        let c_y = Vector::cross(norm, c_x);

        self.receive_from(speaker.pos, norm, speaker.phi_max, 1.0, 0.0);

        for k in min..max {
            let i = k % speaker.theta_resolution;
            let j = k / speaker.theta_resolution;
            let theta: Float =
                (i as Float) * 2.0 * std::f64::consts::PI / (speaker.theta_resolution as Float);
            let phi: Float = (j as Float) * speaker.phi_max / (speaker.phi_resolution as Float);

            let c = c_x * phi.cos() + c_y * phi.sin();
            let t = (1.0 / theta.cos() - 1.0).sqrt();
            let scat_elem_vec = norm + c * t;
            let new_ray = RayWithAttributes {
                ray: Ray {
                    norm: scat_elem_vec,
                    o: speaker.pos.clone(),
                },
                dist: 0.0,
                intensity: 1.0 / (total as Float),
            };
            self.add_ray(new_ray);
        }
    }
    pub fn simulate(&self) -> bool {
        while !self.list.borrow().is_empty() {
            self.update();
        }
        true
    }
    pub fn add_ray(&self, ray: RayWithAttributes) {
        self.list.borrow_mut().push_front(ray);
    }
    pub fn on_refrect(_ray: &mut RayWithAttributes, _poly: &Polygon) -> bool {
        true
    }
    pub fn orthogonal(v: Vector) -> Vector {
        if v.x != 0.0 {
            let res = Vector::new((v.y + v.z) / v.x, 1.0, 1.0);
            return res / res.norm2().sqrt();
        } else if v.y != 0.0 {
            let res = Vector::new(1.0, (v.y + v.z) / v.y, 1.0);
            return res / res.norm2().sqrt();
        } else if v.z != 0.0 {
            let res = Vector::new(1.0, 0.0, 0.0);
            return res;
        } else {
            panic!("origin vector must not be zero vector");
        }
    }
    pub fn add_scatters(&self, ray: &RayWithAttributes, poly: &Polygon, intensity: Float) -> bool {
        let norm = poly.norm();
        let c_x = Self::orthogonal(norm);
        let c_y = Vector::cross(norm, c_x);
        let theta_resolution = 10;
        let phi_resolution = 10;

        let intensity_base =
            intensity * (1.0 - poly.mat.absorption_ratio) * (poly.mat.scattering_ratio);
        let intensity = intensity_base / ((theta_resolution * phi_resolution) as Float);
        if intensity < self.options.min_intensity {
            return false;
        }
        self.receive_from(
            ray.ray.o,
            norm,
            1.0 * std::f64::consts::PI,
            intensity_base,
            ray.dist,
        );
        for i in 0..theta_resolution {
            let theta: Float =
                (i as Float) * 2.0 * std::f64::consts::PI / (theta_resolution as Float);
            for j in 0..phi_resolution {
                let phi: Float =
                    (j as Float) * 1.0 * std::f64::consts::PI / (phi_resolution as Float);

                let c = c_x * phi.cos() + c_y * phi.sin();
                let t = (1.0 / theta.cos() - 1.0).sqrt();
                let scat_elem_vec = norm + c * t;
                let new_ray = RayWithAttributes {
                    ray: Ray {
                        norm: scat_elem_vec,
                        o: ray.ray.o,
                    },
                    dist: ray.dist,
                    intensity: intensity,
                };
                self.add_ray(new_ray);
            }
        }
        true
    }
    pub fn update(&self) -> bool {
        if self.list.borrow().len() == 0 {
            return false;
        }
        let intersection = {
            let list = self.list.borrow();
            let first = list.front().unwrap();

            self.options.room.find_intersection(&first.ray)
        };
        if let Some(intr) = intersection {
            let (poly, pos) = intr;
            let mut first = self.list.borrow().front().unwrap().clone();
            let intensity = first.intensity.clone();
            let ref_vec = Room::refrect(&first.ray, &poly);
            first.ray.norm = ref_vec;
            first.dist = first.dist + (pos - first.ray.o).norm2().sqrt();
            first.ray.o = pos;
            first.intensity = first.intensity
                * (1.0 - poly.mat.absorption_ratio)
                * (1.0 - poly.mat.scattering_ratio);

            self.receive_from(
                first.ray.o,
                first.ray.norm,
                0.0, //std::f64::consts::PI / 100.0,
                first.intensity,
                first.dist,
            );
            if !Self::on_refrect(&mut first, &poly) {
                self.list.borrow_mut().pop_front();
                return false;
            }
            if first.dist > self.options.max_dist || first.intensity < self.options.min_intensity {
                self.list.borrow_mut().pop_front();
                return false;
            }
            let cloned = first.clone();
            {
                self.list.borrow_mut()[0] = first;
            }
            self.add_scatters(&cloned, &poly, intensity);
            return true;
        }
        self.list.borrow_mut().pop_front();
        false
    }
}
