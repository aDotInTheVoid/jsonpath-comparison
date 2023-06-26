use serde_json::Value;

use crate::JsonPath;

mod smoke;

fn check<'a, T: JsonPath<'a>>(query: &'a str, value: Value, expected: Vec<&Value>) {
    let q = T::compile(query);
    assert_eq!(q.find(&value), expected);
}
