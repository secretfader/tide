#![feature(async_await, await_macro)]

use askama::Template;
use tide::{
    EndpointResult,
    Context,
};

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate<'a> {
    name: &'a str
}

async fn index(_: Context<()>) -> EndpointResult {
}

fn main() {
    let mut app = App::new();

    app.at("/").get(index);

    app.serve("127.0.0.1:8000").unwrap();
}
