use crate::JsonPath;
use serde_json::json;

fn smoke<'a, T: JsonPath<'a>>() {
    super::check::<T>("$.a.b", json!({"a": {"b": 1}}), vec![&json!(1)]);
    // check::<T>("$.a.*.b", json!({"a": [{"b": 1}, {"b": 2}]}), vec![]);
}

#[test]
fn redis() {
    smoke::<redis::json_path::Query>();
}

#[test]
fn zhxiaogg() {
    smoke::<crate::Zhxiaogg>();
}

#[test]
fn greyblake() {
    smoke::<greyblake::Selector>();
}

#[test]
fn craftspider() {
    smoke::<craftspider::JsonPath>();
}

#[test]
fn bezok() {
    smoke::<crate::Bezok>();
}

#[test]
fn freestrings() {
    smoke::<freestrings::Compiled>();
}

#[test]
fn hiltontj() {
    smoke::<hiltontj::JsonPath>();
}
