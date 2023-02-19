pub trait Inject<A, T> {
    fn inject(&self, a: A) -> T;
}

pub trait Project<A, T> {
    fn project(&self, t: &mut T) -> Option<A>;
}

pub struct Iso<A, T> {
    pub inj: Box<dyn Fn(A) -> T + Send + 'static>,
    pub prj: Box<dyn Fn(&mut T) -> Option<A> + Send + 'static>,
}

unsafe impl<A, T> Sync for Iso<A, T> {}

impl<A, T> Iso<A, T> {
    pub fn new(
        inj: Box<dyn Fn(A) -> T + Send + 'static>,
        prj: Box<dyn Fn(&mut T) -> Option<A> + Send + 'static>
    ) -> Iso<A, T> {
        Iso { inj, prj }
    }
}

impl<A, T> Project<A, T> for Iso<A, T> {
    fn project(&self, t: &mut T) -> Option<A> {
        (self.prj)(t)
    }
}

impl<A, T> Inject<A, T> for Iso<A, T> {
    fn inject(&self, a: A) -> T {
        (self.inj)(a)
    }
}