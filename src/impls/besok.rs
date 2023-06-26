use std::str::FromStr;

impl<'q> crate::JsonPath<'q> for crate::Bezok<'q> {
    fn compile(query: &'q str) -> Self {
        Self(query)
    }

    fn find<'a>(&self, value: &'a serde_json::Value) -> Vec<&'a serde_json::Value> {
        let q = besok::JsonPathInst::from_str(self.0).unwrap();
        let f = besok::JsonPathFinder::new(Box::new(value.clone()), Box::new(q));
        f.find_slice()
            .into_iter()
            .map(|v| &*Box::leak(Box::new(v.to_data())))
            .collect()
    }

    const NAME: &'static str = "besok";
}
