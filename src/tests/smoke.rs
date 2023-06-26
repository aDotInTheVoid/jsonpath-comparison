use crate::JsonPath;
use serde_json::json;

fn smoke<'a, T: JsonPath<'a>>() {
    let q = T::compile("$.a.b");
    assert_eq!(
        q.find(&json! ({
            "a": {
                "b": 1
            }
        })),
        vec![&json!(1)]
    )
}

#[test]
fn redis() {
    smoke::<redis_json_path::json_path::Query>();
}
