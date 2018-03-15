#[macro_use]
extern crate from_repr_enum_derive;

#[repr(u8)]
#[derive(FromReprEnum,Debug, PartialEq)]
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
