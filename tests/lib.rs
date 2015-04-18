#![feature(custom_attribute, plugin)]
#![plugin(heckle)]

#[heckle]
fn boolean() -> bool {
    false
}

#[heckle]
fn if_expr() -> u32 {
    if (true) {
        666
    } else {
        42
    }
}

#[test]
fn it_inverts_literal_booleans() {
    assert!(boolean());
}

#[test]
fn it_inverts_if_expr_condition() {
    assert!(if_expr() == 42);
}
