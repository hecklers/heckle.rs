#![feature(custom_attribute, plugin)]
#![plugin(heckle)]

#[heckle]
pub mod heckled_primitives {
    pub fn boolean() -> bool {
        false
    }

    pub fn if_expr() -> u32 {
        if (true) {
            666
        } else {
            42
        }
    }

    pub fn string() -> &'static str {
        "foo"
    }

}

use self::heckled_primitives::*;

#[test]
fn test_boolean_mutation() {
    assert!(boolean());
}

#[test]
fn test_if_expr_condition_mutation() {
    assert_eq!(42, if_expr());
}

#[test]
fn test_string_mutation() {
    assert!(string() != "foo");
}
