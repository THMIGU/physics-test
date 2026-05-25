mod fps;
mod physics;

use rapier2d::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder, math::Vec2};
use sdl3::event::Event;
use std::time::{Duration, Instant};

use crate::{fps::FPS, physics::Physics};

const TICK_RATE: f64 = 60_f64;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
	let sdl_context = sdl3::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem
		.window("physics-test", WINDOW_WIDTH, WINDOW_HEIGHT)
		.position_centered()
		.build()
		.unwrap();

	let mut canvas = window.into_canvas();

	let mut event_pump = sdl_context
		.event_pump()
		.unwrap();

	let mut last_frame = Instant::now();
	let mut accumulator = Duration::new(0, 0);
	let tick_time = Duration::from_secs_f64(1_f64 / TICK_RATE);

	let mut fps = FPS::new();

	let mut physics = Physics::new();

	let collider = ColliderBuilder::cuboid(100.0, 0.1).build();
	physics
		.collider_set
		.insert(collider);

	let rigid_body = RigidBodyBuilder::dynamic()
		.translation(Vec2::new(0.0, 10.0))
		.build();
	let collider = ColliderBuilder::ball(0.5)
		.restitution(0.7)
		.build();
	let ball_body_handle = physics
		.rigid_body_set
		.insert(rigid_body);
	physics
		.collider_set
		.insert_with_parent(collider, ball_body_handle, &mut physics.rigid_body_set);

	'running: loop {
		let now = Instant::now();
		let frame_duration = now.duration_since(last_frame);
		accumulator += frame_duration;
		last_frame = now;

		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {
					..
				} => break 'running,
				_ => {}
			}
		}

		while accumulator >= tick_time {
			physics.step();

			let ball_body = &physics.rigid_body_set[ball_body_handle];
			println!("Ball Altitude: {}", ball_body.translation().y);

			accumulator -= tick_time;
		}

		let display_fps = fps.fps(frame_duration);

		canvas
			.window_mut()
			.set_title(&format!("physics-test | {:.0} FPS", display_fps))
			.unwrap();
	}
}
