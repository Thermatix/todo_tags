use std::env;

mod gprc;
mod app;

fn main() {
    let app = app::new(env::args().collect());
}


