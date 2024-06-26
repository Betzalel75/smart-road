use smart_road::*;
extern crate sdl2;

use event::Simulation;
use sdl2::image::LoadTexture;
use std::time::Duration;


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("Traffic Simulation", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let background_texture = texture_creator.load_texture("assets/intersection.png")?;
    let car_texture = Rc::new(texture_creator.load_texture("assets/car.png")?);

    let mut event_pump = sdl_context.event_pump()?;
    let mut simulation = Simulation::new();
    loop {
        for event in event_pump.poll_iter() {
            simulation.handle_event(&event, Rc::clone(&car_texture));
        }

        simulation.update();
        simulation.draw(&mut canvas, &background_texture);

        ::std::thread::sleep(Duration::from_millis(100));
    }
}
