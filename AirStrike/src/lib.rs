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

        // Draw shapes
        // Example 1: Red rounded rectangle
        println!("Drawing rounded_rect1 at (100, 100) with size (200, 150) and corner radius 25");
        ctx.draw(CanvasItem::Shape(
            Area((100, 100), None),
            Shape::RoundedRectangle(2, (200, 150), 25),
            "FF0000", // Red
            255
        ));

        // Example 2: Green rounded rectangle
        println!("Drawing rounded_rect2 at (350, 100) with size (200, 150) and corner radius 25");
        ctx.draw(CanvasItem::Shape(
            Area((350, 100), None),
            Shape::RoundedRectangle(2, (200, 150), 25),
            "00FF00", // Green
            255
        ));

        // Add images if available
        if let Some((image1, image2)) = self.image_keys {
            // Image 1 with rectangular shape
            ctx.draw(CanvasItem::Image(
                Area((100, 300), None),
                Shape::Rectangle(0, (150, 150)),
                image1,
            ));

            // Image 2 with rounded rectangle shape
            ctx.draw(CanvasItem::Image(
                Area((350, 300), None),
                Shape::RoundedRectangle(0, (150, 150), 20),
                image2,
            ));
        }

        // Add text in different sizes if font is available
        if let Some(font) = self.font {
            // Large title text
            ctx.draw(CanvasItem::Text(
                Area((100, 500), None),
                Text::new(
                    "Large Title Text",
                    "FFFFFF", // White
                    255,
                    Some(400),
                    36, // Large font size
                    42, // Line height
                    font
                )
            ));

            // Medium subtitle text
            ctx.draw(CanvasItem::Text(
                Area((100, 550), None),
                Text::new(
                    "Medium Subtitle Text",
                    "CCCCCC", // Light gray
                    255,
                    Some(400),
                    24, // Medium font size
                    30, // Line height
                    font
                )
            ));

            // Small body text
            ctx.draw(CanvasItem::Text(
                Area((100, 600), None),
                Text::new(
                    "This is smaller body text that can be used for descriptions or other content that doesn't need to be as prominent as titles or subtitles.",
                    "AAAAAA", // Lighter gray
                    255,
                    Some(600), // Wider text area
                    16, // Small font size
                    22, // Line height
                    font
                )
            ));

            // Right-aligned status text
            ctx.draw(CanvasItem::Text(
                Area((600, 50), None),
                Text::new(
                    "Status: Active",
                    "00FF00", // Green
                    255,
                    Some(200),
                    18, // Medium-small font size
                    24, // Line height
                    font
                )
            ));
        }

        // Example 5: Magenta normal rectangle
        println!("Drawing rectangle at (100, 700) with size (200, 150)");
        ctx.draw(CanvasItem::Shape(
            Area((100, 700), None),
            Shape::Rectangle(2, (200, 150)),
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
