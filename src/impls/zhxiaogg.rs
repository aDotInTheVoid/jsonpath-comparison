impl<'q> crate::JsonPath<'q> for crate::Zhxiaogg {
    fn compile(query: &'q str) -> Self {
        let tk = zhxiaogg::tokenizer::Tokenizer::new();
        crate::Zhxiaogg(tk.tokenize(query).unwrap())
    }

    fn find<'a>(&self, value: &'a serde_json::Value) -> Vec<&'a serde_json::Value> {
        let mut eval = zhxiaogg::eval::Eval::new();
        let v = eval.eval(value, &self.0).unwrap();

        // TODO: This is bad
        vec![Box::leak(Box::new(v))]
    }

    const NAME: &'static str = "zhxiaogg";
}
