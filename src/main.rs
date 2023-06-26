fn main() {
    println!("Hello, world!");
}

trait JsonPath<'q> {
    fn compile(query: &'q str) -> Self;
    fn find<'a>(&self, value: &'a serde_json::Value) -> Vec<&'a serde_json::Value>;

    const NAME: &'static str;
}

mod impls {
    mod redis;
}

#[cfg(test)]
mod tests;
