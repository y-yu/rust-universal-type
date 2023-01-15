pub struct RefStore {
    pub(crate) clear: Box<dyn Fn() -> ()>,
    pub(crate) store: Box<dyn Fn() -> ()>,
}
