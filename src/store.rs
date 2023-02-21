pub struct RefStore {
    pub(crate) clear: Box<dyn FnMut() -> ()>,
    pub(crate) store: Box<dyn FnMut() -> ()>,
}
