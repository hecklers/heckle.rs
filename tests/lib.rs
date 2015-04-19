#![feature(custom_attribute, plugin)]
#![plugin(heckle)]

#[heckle]
fn boolean() -> bool {
    false
}

#[test]
fn it_works() {
    assert!(boolean());
}
