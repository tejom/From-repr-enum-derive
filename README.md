# from-repr-enum-derive

A crate to derive from for an Enum with a repr defined

Example:

```

#[macro_use]
extern crate from_repr_enum_derive;


#[repr(u8)]
#[derive(FromReprEnum,Debug)]
enum Foo {
    X = 1,
    Y = 2,
    Unknown = 255,
}

fn main() {
    let z = Foo::from(1);
    println!("{:?}",z );
    assert_eq!(Foo::X, z);
}

```
