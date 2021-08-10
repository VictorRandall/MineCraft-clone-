
use std;
use rand::Rng;
use gdnative::api::{ArrayMesh, Mesh, MeshInstance, OpenSimplexNoise, SurfaceTool, Spatial, StaticBody};
use gdnative::prelude::*;

mod voxel_chunk;

use voxel_chunk::VoxelChunk;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct VoxelTerrain{
	chunks: Vec<VoxelChunk>,
	seed:u64,
	chunk_size: u32,
}

#[methods]
impl VoxelTerrain {
	fn new(_owner: &Spatial) -> Self {
		VoxelTerrain{
			chunks: Vec::<VoxelChunk>::new(),
			seed: rand::thread_rng().gen(),
			chunk_size: 16,
		}
	}


	#[export]
	fn _ready(&mut self, _owner: &Spatial) {
		godot_print!("{}",self.seed);
//		let a = Array3::new();
		for x in 0..2{
			for y in 0..2{
				for z in 0..2{
					self.chunks.push(VoxelChunk::new(Vector3::new(x as f32 * 2f32,y as f32 * 2f32,z as f32 * 2f32),8usize));
				}
			}
		}
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
		

		for i in 0..self.chunks.len(){
			self.chunks[i].start(&owner);
//			self.chunks[i].restart(&owner);
		}
	}

}


fn init(handle: InitHandle) {
	handle.add_tool_class::<VoxelTerrain>();
}

godot_init!(init);
