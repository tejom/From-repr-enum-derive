#[macro_use]
extern crate from_repr_enum_derive;

#[repr(u8)]
#[derive(FromReprEnum, Debug, PartialEq)]
enum Foo {
    X = 1,
    Y = 2,
    Unknown = 255,
}

#[test]
fn test_from() {
    let x = Foo::from(1);
    assert_eq!(Foo::X, x);

    let y = Foo::from(2);
    assert_eq!(Foo::Y, y);

    let u = Foo::from(99);
    assert_eq!(Foo::Unknown, u);
}

#[repr(u8)]
#[derive(FromReprEnum, Debug, PartialEq)]
#[ReprEnumDefault = "NotFound"]
enum Bar {
    X = 1,
    Y = 2,
    NotFound = 255,
}

#[test]
fn test_from_with_default() {
    let x = Bar::from(1);
    assert_eq!(Bar::X, x);

    let y = Bar::from(2);
    assert_eq!(Bar::Y, y);

    let u = Bar::from(99);
    assert_eq!(Bar::NotFound, u);
}

#[repr(u64)]
#[derive(FromReprEnum, Debug, PartialEq)]
enum Foo64 {
    X = 1,
    Y = 5,
    Unknown = 255,
}

#[test]
fn test_from_64() {
    let x = Foo64::from(1);
    assert_eq!(Foo64::X, x);

    let y = Foo64::from(5);
    assert_eq!(Foo64::Y, y);

    let u = Foo64::from(99);
    assert_eq!(Foo64::Unknown, u);
}
