# Getting Started with rust_on_rails Graphics Framework

This guide will help you understand how to use the `rust_on_rails` framework to create graphical applications with Rust.

## Table of Contents
- [Introduction](#introduction)
- [Setting Up Your Project](#setting-up-your-project)
- [Creating Your First App](#creating-your-first-app)
- [Working with Shapes](#working-with-shapes)
- [Adding Images](#adding-images)
- [Adding Text](#adding-text)
- [Handling User Input](#handling-user-input)
- [Tips and Best Practices](#tips-and-best-practices)

## Introduction

The `rust_on_rails` framework provides a simple way to create graphical applications in Rust. It allows you to draw shapes, display images, render text, and handle user interactions. This guide will walk you through the basics of using this framework.

## Setting Up Your Project

1. **Create a New Rust Project**:
   ```bash
   cargo new my_graphics_app
   cd my_graphics_app
   ```

2. **Add the Dependency**:
   Add `rust_on_rails` to your `Cargo.toml`:
   ```toml
   [dependencies]
   rust_on_rails = "0.1.0"  # Replace with the actual version
   image = "0.24.6"         # For image handling
   ```

3. **Project Structure**:
   Create folders for your assets:
   ```
   my_graphics_app/
   ├── src/
   │   └── main.rs
   ├── assets/
   │   ├── images/
   │   │   ├── fly.png
   │   │   └── explosion.png
   │   └── fonts/
   │       └── outfit_bold.ttf
   └── Cargo.toml
   ```

## Creating Your First App

Here's a basic template to get started:

```rust
use rust_on_rails::prelude::*;
use prelude::App;

// Define your app's dimensions
const DEFAULT_WIDTH: u32 = 850;
const DEFAULT_HEIGHT: u32 = 1300;

pub struct MyApp {
    window_size: (u32, u32),
    // Add other fields your app needs here
}

impl App for MyApp {
    async fn new(ctx: &mut Context) -> Self {
        let window_size = (DEFAULT_WIDTH, DEFAULT_HEIGHT);
        println!("Creating new MyApp with window size: {:?}", window_size);

        MyApp {
            window_size,
        }
    }

    async fn draw(&mut self, ctx: &mut Context) {
        // Clear the background with a color
        ctx.clear("000000"); // Black background
        
        // Draw your shapes, images, and text here
    }

    async fn on_click(&mut self, ctx: &mut Context) {
        // Handle mouse click events
    }

    async fn on_move(&mut self, ctx: &mut Context) {
        // Handle mouse movement events
    }

    async fn on_press(&mut self, ctx: &mut Context, t: String) {
        // Handle keyboard press events
    }
}

create_entry_points!(MyApp);
```

## Working with Shapes

The framework supports various shapes:

### Rectangles

```rust
// Draw a rectangle at position (100, 100) with size (200, 150)
ctx.draw(CanvasItem::Shape(
    Area((100, 100), None),
    Shape::Rectangle(2, (200, 150)), // 2 is the stroke width
    "FF00FF", // Magenta color in hex
    255 // Alpha (opacity)
));
```

### Rounded Rectangles

```rust
// Draw a rounded rectangle at (350, 100) with size (200, 150) and corner radius 25
ctx.draw(CanvasItem::Shape(
    Area((350, 100), None),
    Shape::RoundedRectangle(2, (200, 150), 25), // Last parameter is corner radius
    "00FF00", // Green color
    255
));
```

### Ellipses

```rust
// Draw an ellipse at (200, 400) with size (150, 100)
ctx.draw(CanvasItem::Shape(
    Area((200, 400), None),
    Shape::Ellipse(2, (150, 100)),
    "0000FF", // Blue color
    255
));
```

## Adding Images

Images need to be loaded first and then can be drawn:

```rust
// In the new() method:
let image1 = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/fly.png")).unwrap().into());

// In the draw() method:
ctx.draw(CanvasItem::Image(
    Area((100, 300), None), // Position
    Shape::Rectangle(0, (150, 150)), // Shape and size
    image1, // Image key
));
```

## Adding Text

Text requires a font to be loaded:

```rust
// In the new() method:
let font = ctx.add_font(include_bytes!("../assets/fonts/outfit_bold.ttf").to_vec());

// In the draw() method:
ctx.draw(CanvasItem::Text(
    Area((100, 500), None), // Position
    Text::new(
        "Hello, World!", // Text content
        "FFFFFF", // White color in hex
        255, // Alpha (opacity)
        Some(400), // Width constraint (optional)
        24, // Font size
        30, // Line height
        font // Font key
    )
));
```

## Handling User Input

The framework provides methods to handle various user inputs:

### Mouse Clicks

```rust
async fn on_click(&mut self, ctx: &mut Context) {
    println!("User clicked somewhere!");
    // Add your click handling logic here
}
```

### Mouse Movement

```rust
async fn on_move(&mut self, ctx: &mut Context) {
    // Add your mouse movement handling logic here
    // You can access mouse position through ctx
}
```

### Keyboard Input

```rust
async fn on_press(&mut self, ctx: &mut Context, t: String) {
    println!("User pressed key: {}", t);
    // Handle different keys
    if t == "ArrowRight" {
        // Move something right
    } else if t == "ArrowLeft" {
        // Move something left
    }
}
```

## Tips and Best Practices

1. **Organizing Your Code**:
    - Keep related functionality in separate modules
    - Create helper methods for repetitive drawing tasks

2. **Performance Optimization**:
    - Load assets once in the `new()` method
    - Avoid creating new objects in the `draw()` method
    - Use the appropriate shape types for your needs

3. **Debugging**:
    - Use `println!` statements to debug your code
    - Print coordinates and dimensions when shapes don't appear as expected

4. **Memory Management**:
    - Rust's ownership system will help prevent memory leaks
    - Be careful with large images or too many objects

5. **Error Handling**:
    - Use proper error handling when loading assets
    - Provide fallback options when assets fail to load

## Complete Example

Here's a complete example that shows how to create a simple application with shapes, images, and text:

```rust
use rust_on_rails::prelude::*;
use std::sync::Arc;
use prelude::App;

const DEFAULT_WIDTH: u32 = 850;
const DEFAULT_HEIGHT: u32 = 1300;

pub struct MyApp {
    window_size: (u32, u32),
    image_keys: Option<(ImageKey, ImageKey)>,
    font: Option<FontKey>,
}

impl App for MyApp {
    async fn new(ctx: &mut Context) -> Self {
        let window_size = (DEFAULT_WIDTH, DEFAULT_HEIGHT);
        println!("Creating new MyApp with window size: {:?}", window_size);

        // Load images
        let image1 = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/fly.png")).unwrap().into());
        let image2 = ctx.add_image(image::load_from_memory(include_bytes!("../assets/images/explosion.png")).unwrap().into());

        // Load font
        let font = ctx.add_font(include_bytes!("../assets/fonts/outfit_bold.ttf").to_vec());
        
        MyApp {
            window_size,
            image_keys: Some((image1, image2)),
            font: Some(font),
        }
    }

    async fn draw(&mut self, ctx: &mut Context) {
        // Clear background
        ctx.clear("000000");

        // Draw shapes
        ctx.draw(CanvasItem::Shape(
            Area((100, 100), None),
            Shape::RoundedRectangle(2, (200, 150), 25),
            "FF0000", // Red
            255
        ));

        ctx.draw(CanvasItem::Shape(
            Area((350, 100), None),
            Shape::RoundedRectangle(2, (200, 150), 25),
            "00FF00", // Green
            255
        ));

        // Draw images
        if let Some((image1, image2)) = self.image_keys {
            ctx.draw(CanvasItem::Image(
                Area((100, 300), None),
                Shape::Rectangle(0, (150, 150)),
                image1,
            ));

            ctx.draw(CanvasItem::Image(
                Area((350, 300), None),
                Shape::RoundedRectangle(0, (150, 150), 20),
                image2,
            ));
        }

        // Draw text
        if let Some(font) = self.font {
            ctx.draw(CanvasItem::Text(
                Area((100, 500), None),
                Text::new(
                    "Hello, World!",
                    "FFFFFF", // White
                    255,
                    Some(400),
                    36,
                    42,
                    font
                )
            ));

            ctx.draw(CanvasItem::Text(
                Area((100, 550), None),
                Text::new(
                    "Welcome to rust_on_rails",
                    "CCCCCC", // Light gray
                    255,
                    Some(400),
                    24,
                    30,
                    font
                )
            ));
        }
    }

    async fn on_click(&mut self, ctx: &mut Context) {
        println!("on_click event triggered");
    }

    async fn on_move(&mut self, ctx: &mut Context) {
        // Handle mouse movement
    }

    async fn on_press(&mut self, ctx: &mut Context, t: String) {
        println!("Key pressed: {}", t);
    }
}

create_entry_points!(MyApp);
```

This guide should help you get started with the `rust_on_rails` framework. As you become more comfortable, you can explore more advanced features and create more complex applications.