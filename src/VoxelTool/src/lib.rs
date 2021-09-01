use std;
use rand::Rng;
use gdnative::api::{Spatial};
use gdnative::prelude::*;

mod voxel_chunk;

use voxel_chunk::VoxelChunk;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct VoxelTerrain{
	chunks: Vec<VoxelChunk>,
	seed:i64,
	seed2:i64,
	chunk_size: usize,
//	#[property]
//	material:Material
}

#[methods]
impl VoxelTerrain {
	fn new(_owner: &Spatial) -> Self {
		VoxelTerrain{
			chunks: Vec::<VoxelChunk>::new(),
			seed: rand::thread_rng().gen(),
			seed2: rand::thread_rng().gen(),
			chunk_size: 50,
//			material:mtrl
		}
	}

	//FIXME: this function doesnt return the real voxel data
//	if raycast.is_colliding():
//		var norm = raycast.get_collision_normal()
//		var pos = raycast.get_collision_point() - norm * 0.5
//		
//		var bx = floor(pos.x) + 0.5
//		var by = floor(pos.y) + 0.5
//		var bz = floor(pos.z) + 0.5
//		var bpos = Vector3(bx, by, bz) - self.translation
//		
//		block_outline.translation = bpos
//		block_outline.visible = true
//		
//		if Input.is_action_just_pressed("Break"):
//			emit_signal("break_block", pos)
//		if Input.is_action_just_pressed("Place"):
//			emit_signal("place_block", pos + norm, Global.STONE)
	#[export]
	fn get_voxel(&self, owner: &Spatial, x:i32, y:i32, z:i32) -> u32{
		return 1u32
		
	}
	#[export]
	fn _ready(&mut self, owner: &Spatial) {
		godot_print!("{:#?}",self.seed);
		for x in 0..4{
			for y in 0..4{
				for z in 0..4{
					self.chunks.insert(x + y + z,VoxelChunk::new(Vector3::new(x as f32 * 50f32 - 1.0f32,y as f32 * 50f32 - 1.0f32,z as f32 * 50f32 - 1.0f32),51usize, self.seed, self.seed2));
					godot_print!("{}", x + y + z);
//					self.chunks.push(VoxelChunk::new(Vector3::new(x as f32,y as f32,z as f32),50usize, self.seed));
				}
			}
		}
		for i in 0..self.chunks.len(){
			self.chunks[i].start(&owner);
//			self.chunks[i].restart(&owner);
		}
//		let a = Array3::new();
		
	}
	#[export]
	fn _process(&mut self, owner: &Spatial, _delta: f64){
		let input = Input::godot_singleton();
//		let label = unsafe {owner.get_node("Label").unwrap().assume_safe().cast::<Label>().unwrap()};
		
//		if input.is_action_pressed("test"){
//			for i in 0..self.chunks.len(){
//				self.chunks[i].size += 1usize;
//				self.chunks[i].size += 1usize;
//				self.chunks[i].size += 1usize;
//				self.chunks[i].update = true;
//			}
//			self.chunks.push(VoxelChunk::new(Vector3::new(self.chunks.len() as f32 + 0.0 ,0.0,0.0),1,1,1));
//		}else if input.is_action_pressed("test2"){
//			for i in 0..self.chunks.len(){
//				self.chunks[i].size -= 1usize;
//				self.chunks[i].size -= 1usize;
//				self.chunks[i].size -= 1usize;
//				self.chunks[i].update = true;
//			}
//		};
		
		
	}

}


fn init(handle: InitHandle) {
	handle.add_tool_class::<VoxelTerrain>();
}

godot_init!(init);
