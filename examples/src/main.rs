#![feature(custom_attribute, plugin)]
#![plugin(heckle)]

#[heckle]
fn greet(name: &str) -> String {
    if name == "Bob" {
        "Hi, Bob!".to_string()
    } else {
        "Hello!".to_string()
    }
}

fn main() {
    println!("{}", greet("Alice"));
}

#[test]
fn test_greet_bob() {
    assert_eq!("Hi, Bob!", greet("Bob"));
}

#[test]
fn test_greet_anyone_else() {
    assert_eq!("Hello!", greet("Bill"));
}
