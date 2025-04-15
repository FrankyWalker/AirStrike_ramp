use rand::Rng;
use rust_on_rails::prelude::*;
use std::sync::Arc;
use prelude::App;


const DEFAULT_WIDTH: u32 = 850;
const DEFAULT_HEIGHT: u32 = 1300;

pub struct MyApp {
    window_size: (u32, u32),
}

impl App for MyApp {
    async fn new(ctx: &mut Context) -> Self {
        let window_size = (DEFAULT_WIDTH, DEFAULT_HEIGHT);

        MyApp {
            window_size,
        }
    }

    async fn draw(&mut self, ctx: &mut Context) {
        ctx.clear("000000");
    }

    async fn on_click(&mut self, ctx: &mut Context) {
        // Empty implementation
    }

    async fn on_move(&mut self, _ctx: &mut Context) {}

    async fn on_press(&mut self, _ctx: &mut Context, _t: String) {}
}

create_entry_points!(MyApp);