mod iso;
mod store;
mod univ;

use crate::univ::primitive::*;
use crate::univ::{RefUniversalType, UniversalType};

fn main() {
    let mut r = (USIZE_ISO.lock().unwrap().inj)(5);
    let opt1 = (F32_ISO.lock().unwrap().prj)(&mut r);
    let opt2 = (USIZE_ISO.lock().unwrap().prj)(&mut r);
    let mut r = (BOOL_ISO.lock().unwrap().inj)(true);
    let opt3 = (BOOL_ISO.lock().unwrap().prj)(&mut r);
    let opt4 = (USIZE_ISO.lock().unwrap().prj)(&mut r);

    println!(
        "opt1: {:?}, opt2: {:?}, opt3: {:?}, opt4: {:?}",
        opt1, opt2, opt3, opt4
    );

    println!("{}", show(&mut r));
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
