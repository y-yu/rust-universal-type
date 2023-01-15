pub struct Iso<A, T> {
    pub inj: Box<dyn FnOnce(A) -> T>,
    pub prj: Box<dyn Fn(&mut T) -> Option<A>>,
}

impl<A, T> Iso<A, T> {
    pub fn new(inj: Box<dyn FnOnce(A) -> T>, prj: Box<dyn Fn(&mut T) -> Option<A>>) -> Iso<A, T> {
        Iso { inj, prj }
    }
}
