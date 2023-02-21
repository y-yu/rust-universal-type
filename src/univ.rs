use crate::iso::Iso;
use crate::store::RefStore;
use std::sync::{Arc, Mutex};

pub trait UniversalType {
    type T;

    fn embed<A: 'static + Copy + Send>() -> Iso<A, Self::T>;
}

pub struct RefUniversalType();

impl UniversalType for RefUniversalType {
    type T = RefStore;

    fn embed<A: 'static + Copy + Send>() -> Iso<A, Self::T> {
        let rc0: Arc<Mutex<Option<A>>> = Arc::new(Mutex::new(None));
        //let rc1 = Rc::clone(&rc0);
        let rc2 = Arc::clone(&rc0);

        Iso::<A, Self::T>::new(
            Box::new(move |a: A| -> RefStore {
                let rc1 = Arc::clone(&rc0);
                let rc3 = Arc::clone(&rc0);
                RefStore {
                    clear: Box::new(move || {
                        let mut d = rc3.lock().unwrap();
                        *d = None;
                        // rc3.lock().unwrap().replace(None);
                        ()
                    }),
                    store: Box::new(move || {
                        let mut d = rc1.lock().unwrap();
                        *d = Some(a);
                        //rc1.replace(Some(a));
                        ()
                    }),
                }
            }),
            Box::new(move |t: &mut Self::T| -> Option<A> {
                let RefStore { store, clear } = t;

                (store)();
                let d = rc2.lock().unwrap();
                let result = *d;
                std::mem::drop(d);
                (clear)();
                result
            }),
        )
    }
}

pub mod primitive {
    use super::*;
    use crate::iso::Iso;
    use crate::store::RefStore;
    use once_cell::sync::Lazy;
    use std::sync::Mutex;

    pub static USIZE_ISO: Lazy<Mutex<Iso<usize, RefStore>>> = Lazy::new(|| {
        let iso = <RefUniversalType as UniversalType>::embed::<usize>();
        Mutex::new(iso)
    });

    pub static BOOL_ISO: Lazy<Mutex<Iso<bool, RefStore>>> = Lazy::new(|| {
        let iso = <RefUniversalType as UniversalType>::embed::<bool>();
        Mutex::new(iso)
    });

    pub static F32_ISO: Lazy<Mutex<Iso<f32, RefStore>>> = Lazy::new(|| {
        let iso = <RefUniversalType as UniversalType>::embed::<f32>();
        Mutex::new(iso)
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitive::*;

    #[test]
    fn test_prj_inj_adhoc() {
        let mut r1 = (USIZE_ISO.lock().unwrap().inj)(5);
        let opt1 = (F32_ISO.lock().unwrap().prj)(&mut r1);
        let opt2 = (USIZE_ISO.lock().unwrap().prj)(&mut r1);
        let mut r2 = (BOOL_ISO.lock().unwrap().inj)(true);
        let opt3 = (BOOL_ISO.lock().unwrap().prj)(&mut r2);
        let opt4 = (USIZE_ISO.lock().unwrap().prj)(&mut r2);

        assert_eq!(opt1, None);
        assert_eq!(opt2, Some(5));
        assert_eq!(opt3, Some(true));
        assert_eq!(opt4, None);

        assert_eq!(show(&mut r1), "int: 5");
        assert_eq!(show(&mut r2), "bool: true");
    }

    fn show(t: &mut <RefUniversalType as UniversalType>::T) -> String {
        match (
            (USIZE_ISO.lock().unwrap().prj)(t),
            (F32_ISO.lock().unwrap().prj)(t),
            (BOOL_ISO.lock().unwrap().prj)(t),
        ) {
            (Some(int), _, _) => format!("int: {}", int),
            (_, Some(float), _) => format!("float: {}", float),
            (_, _, Some(boolean)) => format!("bool: {}", boolean),
            _ => "Unknown type!".to_string(),
        }
    }
}
