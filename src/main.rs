use smart_road::*;
use crate::event::Simulation;

fn main() -> Result<(), String> {
  let sdl_context = sdl2::init()?;
  let video_subsystem = sdl_context.video()?;
  let window = video_subsystem.window("Simulation de Trafic", 800, 800)
      .position_centered()
      .build()
      .map_err(|e| e.to_string())?;

  let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
  let mut event_pump = sdl_context.event_pump()?;
  let mut simulation = Simulation::new();

  loop {
      for event in event_pump.poll_iter() {
          simulation.handle_event(&event);
      }

      simulation.update();
      simulation.draw(&mut canvas);

      ::std::thread::sleep(std::time::Duration::from_millis(100));
  }
}