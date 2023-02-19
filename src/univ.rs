use crate::iso::Iso;
use crate::store::RefStore;
use std::cell::RefCell;
use std::sync::Mutex;
use std::sync::Arc;
use std::mem::drop;

pub trait UniversalType {
    type T;

    fn embed<A: 'static + Copy + Send>() -> Iso<A, Self::T>;
}

pub struct RefUniversalType();

impl UniversalType for RefUniversalType {
    type T = RefStore;

    fn embed<A: 'static + Copy + Send>() -> Iso<A, Self::T> {
        println!("aaaaaa");
        let mutex0: Arc<Mutex<Option<A>>> = Arc::new(Mutex::new(None));
        let rc2: Arc<Mutex<Option<A>>> = Arc::clone(&mutex0);

        println!("bbb");

        println!("ccc");
        println!("iddd");

        Iso::<A, Self::T>::new(
            Box::new(move |a: A| -> RefStore {
                let rc0: Arc<Mutex<Option<A>>> = Arc::clone(&mutex0);
                let rc1: Arc<Mutex<Option<A>>> = Arc::clone(&mutex0);
                RefStore {
                    clear: Box::new(move || {
                        let mut data = rc0.lock().unwrap();
                        *data = None;
                        drop(data);
                        ()
                    }),
                    store: Box::new(move || {
                        let mut data = rc1.lock().unwrap();
                        *data = Some(a);
                        drop(data);
                        ()
                    }),
                }
            }),
            Box::new(move |t: &mut Self::T| -> Option<A> {
                let RefStore { store, clear } = t;

                (store)();
                let guard = rc2.lock().unwrap();
                (clear)();
                let result = *guard;
                drop(guard);
                result
            }),
        )
    }
}

pub mod primitives {

    use super::*;
    use crate::iso::Iso;
    use std::sync::RwLock;
    use once_cell::sync::Lazy;

    pub static USIZE_ISO: Lazy<Iso<usize, RefStore>> = Lazy::new(|| {
        <RefUniversalType as UniversalType>::embed::<usize>()
    });
    pub static FLOAT_ISO: Lazy<Iso<f32, RefStore>> = Lazy::new(|| {
        <RefUniversalType as UniversalType>::embed::<f32>()
    });
    pub static BOOL_ISO: Lazy<Iso<bool, RefStore>> =  Lazy::new(|| {
        <RefUniversalType as UniversalType>::embed::<bool>()
    });
}