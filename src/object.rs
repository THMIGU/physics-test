use rapier2d::{dynamics::RigidBodyHandle, geometry::ColliderHandle, math::Vec2};
use sdl3::{rect::Rect, render::Canvas, video::Window};

use crate::physics::Physics;

#[derive(Clone, Copy)]
pub enum PhysicsHandle {
	RigidBody(RigidBodyHandle),
	Collider(ColliderHandle),
}

pub struct Object {
	pub physics_handle: PhysicsHandle,
	pub size: Vec2,
	pub pos: Vec2,
}

impl Object {
	pub fn from_rigid_body(handle: RigidBodyHandle) -> Self {
		Self {
			physics_handle: PhysicsHandle::RigidBody(handle),
			size: Vec2::new(0_f32, 0_f32),
			pos: Vec2::new(0_f32, 0_f32),
		}
	}

	pub fn from_collider(handle: ColliderHandle) -> Self {
		Self {
			physics_handle: PhysicsHandle::Collider(handle),
			size: Vec2::new(0_f32, 0_f32),
			pos: Vec2::new(0_f32, 0_f32),
		}
	}

	fn pos(&self, physics: &Physics) -> Vec2 {
		match self.physics_handle {
			PhysicsHandle::RigidBody(handle) => {
				let body = &physics.rigid_body_set[handle];
				body.translation()
			}
			PhysicsHandle::Collider(handle) => {
				let collider = &physics.collider_set[handle];
				collider.translation()
			}
		}
	}

	fn size(&self, physics: &Physics) -> Vec2 {
		match self.physics_handle {
			PhysicsHandle::RigidBody(handle) => {
				let body = &physics.rigid_body_set[handle];

				let collider_handle = body.colliders()[0];
				let collider = &physics.collider_set[collider_handle];

				let aabb = collider.compute_aabb();

				Vec2::new(aabb.maxs.x - aabb.mins.x, aabb.maxs.y - aabb.mins.y)
			}
			PhysicsHandle::Collider(handle) => {
				let collider = &physics.collider_set[handle];
				let aabb = collider.compute_aabb();

				Vec2::new(aabb.maxs.x - aabb.mins.x, aabb.maxs.y - aabb.mins.y)
			}
		}
	}

	pub fn update(&mut self, physics: &Physics) {
		self.pos = self.pos(physics);
		self.size = self.size(physics);
	}

	pub fn render(&self, canvas: &mut Canvas<Window>, scl: f32) {
		let window = canvas.window();
		let (_, h) = window.size();

		let rect = Rect::new(
			(self.pos.x * scl) as i32,
			h as i32 - (self.pos.y * scl) as i32 - (self.size.y * scl / 2_f32) as i32,
			(self.size.x * scl) as u32,
			(self.size.y * scl) as u32,
		);

		canvas
			.fill_rect(rect)
			.unwrap();
	}
}
