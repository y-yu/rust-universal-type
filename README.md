Universal Type in Rust
===============================================

[![CI](https://github.com/y-yu/rust-universal-type/actions/workflows/ci.yml/badge.svg)](https://github.com/y-yu/rust-universal-type/actions/workflows/ci.yml)

```rust
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
}
```

```
opt1: None, opt2: Some(5), opt3: Some(true), opt4: None
```

## References

- [UniversalType](http://mlton.org/UniversalType) (be careful, this link is HTTP, not HTTPS)
