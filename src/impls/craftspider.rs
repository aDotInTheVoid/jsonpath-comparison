impl<'q> crate::JsonPath<'q> for craftspider::JsonPath {
    fn compile(query: &'q str) -> Self {
        Self::compile(query).unwrap()
    }

    fn find<'a>(&self, value: &'a serde_json::Value) -> Vec<&'a serde_json::Value> {
        self.find(value)
    }

    const NAME: &'static str = "craftspider";
}
