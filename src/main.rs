extern crate sdl2;
extern crate sdl2_image;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{self, InitFlag, LoadTexture};
use std::time::Duration;
use smart_road::*;


use car::{Car, DirectionCar};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = sdl2_image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("Traffic Simulation", 800, 800)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let background_texture = texture_creator.load_texture("assets/intersection.png")?;
    let car_texture = texture_creator.load_texture("assets/car.png")?;

    let mut cars = vec![
        Car::new(car_texture.clone(), 400, 0, DirectionCar::Down),
        Car::new(car_texture.clone(), 0, 400, DirectionCar::Right),
        Car::new(car_texture.clone(), 400, 800, DirectionCar::Up),
        Car::new(car_texture.clone(), 800, 400, DirectionCar::Left),
    ];

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }

        for car in &mut cars {
            car.update();
        }

        canvas.clear();
        canvas.copy(&background_texture, None, None)?;

        for car in &cars {
            car.draw(&mut canvas);
        }

        canvas.present();
        ::std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}
