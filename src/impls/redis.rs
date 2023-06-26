impl<'q> crate::JsonPath<'q> for redis::json_path::Query<'q> {
    fn compile(query: &'q str) -> Self {
        redis::compile(query).unwrap()
    }

    fn find<'a>(&self, value: &'a serde_json::Value) -> Vec<&'a serde_json::Value> {
        let pc =
            redis::json_path::PathCalculator::<redis::json_path::DummyTrackerGenerator>::create(
                self,
            );
        pc.calc(value)
    }

    const NAME: &'static str = "redis";
}
