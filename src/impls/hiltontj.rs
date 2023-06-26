impl<'q> crate::JsonPath<'q> for hiltontj::JsonPath {
    fn compile(query: &'q str) -> Self {
        Self::parse(query).unwrap()
    }

    fn find<'a>(&self, value: &'a serde_json::Value) -> Vec<&'a serde_json::Value> {
        self.query(value).all()
    }

    const NAME: &'static str = "hiltontj";
}
