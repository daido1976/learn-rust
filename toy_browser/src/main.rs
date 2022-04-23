#![allow(dead_code)]

mod dom;
mod html;

fn main() {
    let t = dom::text("Hello".to_string());
    println!("{:?}", t.node_type);
}
