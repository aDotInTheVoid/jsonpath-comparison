fn main() {
    println!("Hello, world!");
}

trait JsonPath<'q> {
    fn compile(query: &'q str) -> Self;
    fn find<'a>(&self, value: &'a serde_json::Value) -> Vec<&'a serde_json::Value>;

    const NAME: &'static str;
}

pub struct Bezok<'q>(&'q str);
pub type Craftspider = craftspider::JsonPath;
pub type Freestrings = freestrings::Compiled;
pub type Greyblake = greyblake::Selector;
pub type Hiltontj = hiltontj::JsonPath;
pub type Redis<'q> = redis::json_path::Query<'q>;
pub struct Zhxiaogg(Vec<zhxiaogg::tokenizer::Token>);

mod impls {
    mod besok;
    mod craftspider;
    mod freestrings;
    mod greyblake;
    mod hiltontj;
    mod redis;
    mod zhxiaogg;
}

#[cfg(test)]
mod tests;
