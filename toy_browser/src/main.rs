#![allow(dead_code)]

mod css;
mod dom;
mod html;

fn main() {
    let html = r#"
    <html>
        <body>
            <h1>Title</h1>
            <div id="main" class="test">
                <p>Hello <em>world</em>!</p>
            </div>
        </body>
    </html>
    "#
    .to_string();
    let root_node = html::parse(html);
    println!("{:?}", root_node);
}
