use ggez::{event, ContextBuilder, graphics::*, conf::{WindowMode, FullscreenType}};
use std::env;

pub mod game;

fn main() {
    // Make a Context and an EventLoop.
    let (mut ctx, mut event_loop) =
        ContextBuilder::new("Heat Simulator", "Soham Beesetti")
           .build()
           .unwrap();
    
    // Screen Config Stuff
    let mut args = env::args();
    if args.len() != 3 { panic!("Please pass cmd line arguments as (executable.exe 800.0 600.0)") }
    args.next();
    let width = args.next().unwrap().trim().parse::<f32>().unwrap();
    let height = args.next().unwrap().trim().parse::<f32>().unwrap();

    set_mode(&mut ctx, WindowMode {
        width,
        height,
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        max_width: 0.0,
        max_height: 0.0,
        min_width: 0.0,
        min_height: 0.0,
        resizable: false
    }).unwrap();

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object
    // so it can load resources like images during setup.
    let mut my_game = game::Game::new(width, height, 20.0);

    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e)
    }
}

