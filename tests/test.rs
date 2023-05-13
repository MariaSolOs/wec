use pretty_assertions::assert_eq;
use wec::*;

#[test]
fn test_bec() {
    let v = bec![1, 2, 3];
    assert_eq!(v, vec![Box::new(1), Box::new(2), Box::new(3)]);
}

#[test]
fn test_vinto() {
    let v: Vec<String> = vinto!["foo", String::from("bar")];
    assert_eq!(v, vec![String::from("foo"), String::from("bar")]);
}

#[test]
fn test_annotated_empty() {
    let _: Vec<bool> = bec![];
    let _: Vec<bool> = vinto![];
}
