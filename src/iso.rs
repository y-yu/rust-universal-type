pub struct Iso<A, T> {
    pub inj: Box<dyn FnMut(A) -> T + 'static + Send>,
    pub prj: Box<dyn Fn(&mut T) -> Option<A> + 'static + Send>,
}

impl<A, T> Iso<A, T> {
    pub fn new(
        inj: Box<dyn FnMut(A) -> T + 'static + Send>,
        prj: Box<dyn Fn(&mut T) -> Option<A> + 'static + Send>,
    ) -> Iso<A, T> {
        Iso { inj, prj }
    }
}
