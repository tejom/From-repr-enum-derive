# from-repr-enum-derive

A crate to derive from for an Enum with a repr defined

Currently a `#[repr()]` is needed for the derive to work

Example:

```rust

#[macro_use]
extern crate from_repr_enum_derive;


#[repr(u8)]
#[derive(FromReprEnum,Debug,PartialEq)]
enum Foo {
    X = 1,
    Y = 2,
    Unknown = 255,
}

fn main() {
    let z = Foo::from(1);
    assert_eq!(Foo::X, z);
}

```

The match block that is created needs a default enum variant. The crate has a default of `Unknown`

## Custom default variant
You can define your own with another attribute `#[ReprEnumDefault = ""]`

```rust
#[repr(u8)]
#[derive(FromReprEnum, Debug, PartialEq)]
#[ReprEnumDefault = "NotFound"]
enum Bar {
    X = 1,
    Y = 2,
    NotFound = 255,
}

fn main() {
    let x = Bar::from(1);
    assert_eq!(Bar::X, x);

    let u = Bar::from(99);
    assert_eq!(Bar::NotFound, u);
}
```

## Notes

The match block generated tries to use the Enum name with a wildcard. If the variants aren't found by the compiler you might need to add `use path::to::Enum::*`
