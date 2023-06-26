impl<'q> crate::JsonPath<'q> for freestrings::Compiled {
    fn compile(query: &'q str) -> Self {
        Self::compile(query).unwrap()
    }

    fn find<'a>(&self, value: &'a serde_json::Value) -> Vec<&'a serde_json::Value> {
        self.select(value).unwrap()
    }

    const NAME: &'static str = "freestrings";
}
