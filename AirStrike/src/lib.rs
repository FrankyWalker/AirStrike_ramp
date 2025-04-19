use rust_on_rails::prelude::*;
use std::sync::Arc;
use prelude::App;

const DEFAULT_WIDTH: u32 = 850;
const DEFAULT_HEIGHT: u32 = 1300;

pub struct MyApp {
    window_size: (u32, u32),
    image_keys: Option<(ImageKey, ImageKey)>, // Store your image keys
    font: Option<FontKey>,                    // Store your font key
}

impl App for MyApp {
    async fn new(ctx: &mut Context) -> Self {
        let window_size = (DEFAULT_WIDTH, DEFAULT_HEIGHT);
        println!("Creating new MyApp with window size: {:?}", window_size);

        let image1 = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/fly.png")).unwrap().into());
        let image2 = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/explosion.png")).unwrap().into());

        let font = ctx.add_font(include_bytes!("../assets/fonts/outfit_bold.ttf").to_vec());

        MyApp {
            window_size,
            image_keys: Some((image1, image2)),
            font: Some(font),
        }
    }

    async fn draw(&mut self, ctx: &mut Context) {
        println!("Starting draw method");

        // Clear the background with black color
        println!("Clearing background with color: 000000");
        ctx.clear("000000");
        //ADDED LINE
        ctx.draw(CanvasItem::Shape(
            Area((100, 700), None),
            Shape::Line(2, (200, 150)),
            "FF00FF", // Magenta
            255
        ));

        println!("Finished draw method");
    }

    async fn on_click(&mut self, ctx: &mut Context) {
        println!("on_click event triggered");
        // Handle click events if needed
    }

    async fn on_move(&mut self, ctx: &mut Context) {
        // Handle mouse movement if needed
    }

    async fn on_press(&mut self, ctx: &mut Context, t: String) {
        println!("on_press event triggered with key: {}", t);
        // Handle key press events if needed
    }
}

create_entry_points!(MyApp);
