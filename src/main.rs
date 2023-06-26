fn main() {
    println!("Hello, world!");
}

trait JsonPath<'q> {
    fn compile(query: &'q str) -> Self;
    fn find<'a>(&self, value: &'a serde_json::Value) -> Vec<&'a serde_json::Value>;

    const NAME: &'static str;
}

pub struct Zhxiaogg(Vec<zhxiaogg::tokenizer::Token>);

pub struct Bezok<'q>(&'q str);

mod impls {
    mod besok;
    mod craftspider;
    mod greyblake;
    mod redis;
    mod zhxiaogg;
}

#[cfg(test)]
mod tests;
