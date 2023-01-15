mod iso;
mod store;
mod univ;

use crate::iso::Iso;
use crate::univ::{RefUniversalType, UniversalType};

fn main() {
    let Iso {
        inj: int_inj,
        prj: int_prj,
    } = <RefUniversalType as UniversalType>::embed::<usize>();
    let Iso {
        inj: _,
        prj: float_prj,
    } = <RefUniversalType as UniversalType>::embed::<f32>();
    let Iso {
        inj: bool_inj,
        prj: bool_prj,
    } = <RefUniversalType as UniversalType>::embed::<bool>();
    let mut r = (int_inj)(5);
    let opt1 = (float_prj)(&mut r);
    let opt2 = (int_prj)(&mut r);
    let mut r = (bool_inj)(true);
    let opt3 = (bool_prj)(&mut r);
    let opt4 = (int_prj)(&mut r);

    println!(
        "opt1: {:?}, opt2: {:?}, opt3: {:?}, opt4: {:?}",
        opt1, opt2, opt3, opt4
    );

    // Broken!
    println!("{}", show::<RefUniversalType>(&mut r));
}

fn show<A: UniversalType>(t: &mut A::T) -> String {
    let Iso {
        inj: _,
        prj: int_prj,
    } = <A as UniversalType>::embed::<usize>();
    let Iso {
        inj: _,
        prj: float_prj,
    } = <A as UniversalType>::embed::<f32>();
    let Iso {
        inj: _,
        prj: bool_prj,
    } = <A as UniversalType>::embed::<bool>();

    match ((int_prj)(t), (float_prj)(t), (bool_prj)(t)) {
        (Some(int), _, _) => format!("int: {}", int),
        (_, Some(float), _) => format!("float: {}", float),
        (_, _, Some(boolean)) => format!("bool: {}", boolean),
        _ => "Unknown type!".to_string(),
    }
}
