impl<'q> crate::JsonPath<'q> for greyblake::Selector {
    fn compile(query: &'q str) -> Self {
        Self::new(query).unwrap()
    }

    fn find<'a>(&self, value: &'a serde_json::Value) -> Vec<&'a serde_json::Value> {
        self.find(value).collect()
    }

    const NAME: &'static str = "greyblake";
}
