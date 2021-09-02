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
					self.chunks.insert(x + y + z,VoxelChunk::new(Vector3::new(x as f32 * 10f32 - 1.0f32,y as f32 * 10f32 - 1.0f32,z as f32 * 10f32 - 1.0f32),11usize, self.seed, self.seed2));
					godot_print!("{}", x + y + z);
//					self.chunks[i].generate(&owner, x as i32, y as i32, z as i32);
//					self.chunks.push(VoxelChunk::new(Vector3::new(x as f32,y as f32,z as f32),50usize, self.seed));
				}
			}
		}
		for i in 0..self.chunks.len(){
//			for x in 0..4{
//				for y in 0..4{
//					for z in 0..4{
//						self.chunks[i].generate(&owner, x as i32, y as i32, z as i32);
//			}}}
			self.chunks[i].generate(&owner);
//			self.chunks[i].restart(&owner);
		}
	}
	#[export]
	fn _process(&mut self, owner: &Spatial, _delta: f64){
		let player = unsafe {
			owner.get_node("../KinematicBody").unwrap().assume_safe().cast::<KinematicBody>().unwrap()
		};
//		let player_old_pos: Vec<i32;3usize> = player.translation();
		let player_new_pos: Vec<i32> = vec![
		(player.translation().x / 10.0f32) as i32,
		(player.translation().y / 10.0f32) as i32,
		(player.translation().z / 10.0f32) as i32];
		godot_print!("Vector3(x{}, y{}, z{})\nVector3(cx{}, cy{}, cz{})",
			player.translation().x, player.translation().y, player.translation().z,
			player_new_pos[0], player_new_pos[1], player_new_pos[2]);
//		{
			
//		}
	}

}


fn init(handle: InitHandle) {
	handle.add_tool_class::<VoxelTerrain>();
}

godot_init!(init);
