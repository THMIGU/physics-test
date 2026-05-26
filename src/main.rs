mod fps;
mod object;
mod physics;

use rapier2d::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder, math::Vec2};
use sdl3::{event::Event, pixels::Color};
use std::time::{Duration, Instant};

use crate::{fps::FPS, object::Object, physics::Physics};

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

	let collider = ColliderBuilder::cuboid(100_f32, 0.5).build();
	let collider_handle = physics
		.collider_set
		.insert(collider);

	let mut ground = Object::from_collider(collider_handle);

	let rigid_body = RigidBodyBuilder::dynamic()
		.translation(Vec2::new(3.5, 10_f32))
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

	let mut ball = Object::from_rigid_body(ball_body_handle);

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

			ground.update(&physics);
			ball.update(&physics);

			accumulator -= tick_time;
		}

		let display_fps = fps.fps(frame_duration);

		canvas
			.window_mut()
			.set_title(&format!("physics-test | {:.0} FPS", display_fps))
			.unwrap();

		canvas.set_draw_color(Color::BLACK);
		canvas.clear();

		canvas.set_draw_color(Color::WHITE);
		ground.render(&mut canvas, 100_f32);
		canvas.set_draw_color(Color::GRAY);
		ball.render(&mut canvas, 100_f32);

		canvas.present();
	}
}
