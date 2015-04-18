#![feature(plugin)]

#[plugin(heckle)]
fn boolean() -> bool {
    false
}

#[test]
fn it_works() {
    assert!(boolean());
}
