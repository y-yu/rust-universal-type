mod iso;
mod store;
mod univ;

use crate::iso::Iso;
use crate::univ::{RefUniversalType, UniversalType};
use crate::univ::primitives::*;
use crate::iso::Inject;

fn main() {
    let r = USIZE_ISO.inject(5);
    let opt1 = (FLOAT_ISO.prj)(&mut r);
    let opt2 = (USIZE_ISO.prj)(&mut r);
    let r = (BOOL_ISO.inj)(true);
    let opt3 = (BOOL_ISO.prj)(&mut r);
    let opt4 = (USIZE_ISO.prj)(&mut r);

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
