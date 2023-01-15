use crate::iso::Iso;
use crate::store::RefStore;
use std::cell::RefCell;
use std::rc::Rc;

pub trait UniversalType {
    type T;

    fn embed<A: 'static + Copy>() -> Iso<A, Self::T>;
}

pub struct RefUniversalType();

impl UniversalType for RefUniversalType {
    type T = RefStore;

    fn embed<A: 'static + Copy>() -> Iso<A, Self::T> {
        let rc0: Rc<RefCell<Option<A>>> = Rc::new(RefCell::new(None));
        let rc1 = Rc::clone(&rc0);
        let rc2 = Rc::clone(&rc0);

        Iso::<A, Self::T>::new(
            Box::new(move |a: A| -> RefStore {
                RefStore {
                    clear: Box::new(move || {
                        rc0.replace(None);
                        ()
                    }),
                    store: Box::new(move || {
                        rc1.replace(Some(a));
                        ()
                    }),
                }
            }),
            Box::new(move |t: &mut Self::T| -> Option<A> {
                let RefStore { store, clear } = t;

                (store)();
                let result = rc2.take();
                (clear)();
                result
            }),
        )
    }
}
