impl<'q> crate::JsonPath<'q> for redis_json_path::json_path::Query<'q> {
    fn compile(query: &'q str) -> Self {
        redis_json_path::compile(query).unwrap()
    }

    fn find<'a>(&self, value: &'a serde_json::Value) -> Vec<&'a serde_json::Value> {
        let pc = redis_json_path::json_path::PathCalculator::<
            redis_json_path::json_path::DummyTrackerGenerator,
        >::create(self);
        pc.calc(value)
    }

    const NAME: &'static str = "redis-json";
}
