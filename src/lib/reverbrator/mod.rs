pub use super::*;

#[derive(Debug, Clone)]
struct Context<'a> {
    opt: ContextInitOptions<'a>,
    coef: CoefVec,
}

#[derive(Debug, Clone)]
struct ContextInitOptions<'a> {
    hrtf: std::sync::Arc<&'a HRTFData>,
}
#[derive(Debug, Clone)]
pub struct Reverbrator {}

#[derive(Debug, Clone)]
pub struct Filter {}

#[derive(Debug, Clone)]
pub struct CoefVec {
    vec: Vec<Float>,
    range_x: (u32, u32),
    range_y: (u32, u32),
}

impl CoefVec {
    fn new(opt: &ContextInitOptions) -> Self {
        Self {
            vec: vec![],
            range_x: opt.hrtf.range_x,
            range_y: opt.hrtf.range_y,
        }
    }
    pub fn empty() -> Self {
        Self {
            vec: vec![],
            range_x: (0, 0),
            range_y: (0, 0),
        }
    }
}
impl std::ops::AddAssign for CoefVec {
    fn add_assign(&mut self, other: Self) {
        if self.vec.len() != other.vec.len() {
            panic!("vector size must equal to other")
        }
        if self.range_x == (0, 0) {
            self.vec = other.vec.clone();
            self.range_x = other.range_x.clone();
            self.range_y = other.range_y.clone();
        } else {
            for (i, v) in self.vec.iter_mut().enumerate() {
                *v += other.vec[i];
            }
        }
    }
}
impl<'a> ThreadContext<'a, ContextInitOptions<'a>> for Context<'a> {
    fn new(opt: &ContextInitOptions<'a>) -> Self {
        Context {
            opt: opt.clone(),
            coef: CoefVec::new(opt),
        }
    }
}
impl std::ops::AddAssign for Filter {
    fn add_assign(&mut self, _other: Self) {}
}
impl Reverbrator {
    fn on_receive(_ctx: std::cell::RefMut<Context>, _event: ReceiveEvent) -> () {}
    fn merge_contexts(contexts: Vec<Context>) -> CoefVec {
        let mut c = CoefVec::empty();
        for ctx in contexts.into_iter() {
            c += ctx.coef;
        }
        c
    }
    pub fn simulate<'a>(hrtf: &'a HRTFData, options: Options) -> CoefVec {
        let ctx_opt: ContextInitOptions<'a> = ContextInitOptions {
            hrtf: std::sync::Arc::new(hrtf),
        };
        let contexts: Vec<Context> = multi_simulate(5, Self::on_receive, options, ctx_opt);

        let result = Self::merge_contexts(contexts);
        result
    }
}
