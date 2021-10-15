use std;
use gdnative::prelude::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ChunkPos{
	x: i32,
	y: i32,
	z: i32
}

struct VoxelChunk{
	data: Vec<Vec<Vec<u16>>>,
	should_remove: bool
}

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct VoxelSistem{
	chunks: std::collections::HashMap<ChunkPos, VoxelChunk>,
	chunk_size: u8
}

#[methods]
impl VoxelSistem {
	fn new(_owner: &Spatial) -> Self {
		VoxelSistem{
			chunks: std::collections::HashMap::<ChunkPos, VoxelChunk>::new(),
			chunk_size: 16u8
		}
	}

	#[export]
	fn _ready(&self, _owner: &Spatial) {
		godot_print!("Hello, world.");
	}
	
	fn build_chunk(&mut self, pos: Vector3){
		if self.chunks.contains_key(&ChunkPos{x: pos.x as i32,y: pos.y as i32,z: pos.z as i32}){
			return;
		}
	
		self.chunks.insert(
			ChunkPos{x: pos.x as i32,y: pos.y as i32,z: pos.z as i32}, 
			VoxelChunk{
				data: vec![vec![vec![0u16;self.chunk_size as usize];self.chunk_size as usize];self.chunk_size as usize],
				should_remove: false
			}
		);
	
//		for x in 0..self.chunk_size{
//			for y in 0..self.chunk_size{
//				for z in 0..self.chunk_size{
//					
//				}
//			}
//		}
	}
	
	fn build_chunk_mesh(&self, pos: Vector3){
		
	}
	
	fn _process(&self, _owner: &Spatial, _delta: f64){
		
	}
	
	#[export]
	fn set_voxel(&mut self, _owner: &Spatial, pos:Vector3, id: u16){
		let cpos: Vec<i32> = vec![
			(pos.x.floor() / self.chunk_size as f32).floor() as i32,
			(pos.y.floor() / self.chunk_size as f32).floor() as i32,
			(pos.z.floor() / self.chunk_size as f32).floor() as i32
		];
		
		let bpos: Vec<f32> = vec![
			pos.x.floor() % self.chunk_size as f32,
			pos.y.floor() % self.chunk_size as f32,
			pos.z.floor() % self.chunk_size as f32
		];
		
		godot_print!("the chunk pos is Vector3({},{},{})\nthe voxel pos is Vector3({},{},{})",
					cpos[0],cpos[1],cpos[2],  
					bpos[0],bpos[1],bpos[2]);
		self.chunks.get_mut(&ChunkPos{x:cpos[0],y:cpos[1],z:cpos[2]}).unwrap().data[bpos[0] as usize][bpos[1] as usize][bpos[2] as usize] = id;
	}
}

fn init(handle: InitHandle) {
	handle.add_tool_class::<VoxelSistem>();
}

godot_init!(init);

