use std;
use rand::Rng;
use gdnative::api::{Spatial};
use gdnative::prelude::*;

mod voxel_chunk;

use voxel_chunk::VoxelChunk;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct VoxelTerrain{
	chunks: std::collections::HashMap<String, VoxelChunk>,
	seed:i64,
//	seed2:i64,	
	chunk_size: i32,
//	#[property]
//	material:Material
}

#[methods]
impl VoxelTerrain {
	fn new(_owner: &Spatial) -> Self {
		VoxelTerrain{
			chunks: std::collections::HashMap::new(),
			seed: rand::thread_rng().gen(),
			//seed2: rand::thread_rng().gen(),
			chunk_size: 10i32,
//			material:mtrl
		}
	}

	//FIXME: this function doesnt return the real voxel data
	#[export]
	fn get_voxel(&self, owner: &Spatial, x:i32, y:i32, z:i32) -> u32{
		return 1u32
	}

//      TODO: make a hashmap (dictionary but in rust) to store the chunks instead of a vec (array but in rust)
//	#[export]
	fn add_chunk(&mut self,owner: &Spatial,x:i32, y:i32, z:i32){
		if self.chunks.contains_key(&format!("{},{},{}", x, y, z)){
			return;
		}
		godot_print!("generated chunk at {},{},{}", x, y, z);
		self.chunks.insert(
			format!("{},{},{}", x, y, z),
			VoxelChunk::new(Vector3::new(x as f32 * chunk_size as f32 - 1.0f32,y as f32 * chunk_size as f32 - 1.0f32,z as f32 * chunk_size as f32 - 1.0f32),
			(chunk_size + 1) as usize, 
			self.seed)
		);//, self.seed2
		self.chunks.get_mut(&format!("{},{},{}", x, y, z)).unwrap().generate(&owner);
	}

	fn get_chunk(x:i32, y:i32, z:i32){
		return;
	}

	fn remove_chunk(x:i32, y:i32, z:i32){
		return;
	}

	#[export]
	fn _ready(&mut self, owner: &Spatial) {
		let player = unsafe {
			owner.get_node("../KinematicBody").unwrap().assume_safe().cast::<KinematicBody>().unwrap()
		};
		
		let p_pos: Vec<i32> = vec![
			(player.translation().x / 10.0f32) as i32,
			(player.translation().y / 10.0f32) as i32,
			(player.translation().z / 10.0f32) as i32];
		
		godot_print!("{:?}",self.chunks);
//		for x in 30-(p_pos[0]*10)..30+(p_pos[0]*10){
//			for y in 30-(p_pos[0]*10)..30+(p_pos[0]*10){
//				for z in 30-(p_pos[0]*10)..30+(p_pos[0]*10){
//					godot_print!("i have aids");
//					self.add_chunk(owner,x,y,z);
//					self.chunks.insert(
//						format!("{},{},{}", x, y, z),
//						VoxelChunk::new(Vector3::new(x as f32 * 10f32 - 1.0f32,y as f32 * 10f32 - 1.0f32,z as f32 * 10f32 - 1.0f32),11usize, self.seed));//, self.seed2
//					self.chunks.get_mut(&format!("{},{},{}", x, y, z)).unwrap().generate(&owner);
//				}
//			}
//		}
//		for x in 0i32..8i32{
//			for y in 0i32..8i32{
//				for z in 0i32..8i32{
//					self.add_chunk(owner,x,y,z);
//					self.chunks.insert(format!("{},{},{}", x, y, z),VoxelChunk::new(Vector3::new(x as f32 * 10f32 - 1.0f32,y as f32 * 10f32 - 1.0f32,z as f32 * 10f32 - 1.0f32),11usize, self.seed));//, self.seed2
//					godot_print!("{},{},{}", x, y, z);
//					self.chunks.get_mut(&format!("{},{},{}", x, y, z)).unwrap().generate(&owner)
//					self.chunks[i].generate(&owner, x as i32, y as i32, z as i32);
//					self.chunks.push(VoxelChunk::new(Vector3::new(x as f32,y as f32,z as f32),50usize, self.seed));
//				}
//			}
//		}
//		for i in 0..self.chunks.len(){
//			for x in 0..4{
//				for y in 0..4{
//					for z in 0..4{
//						self.chunks[i].generate(&owner, x as i32, y as i32, z as i32);
//			}}}
//			;
//			self.chunks[i].restart(&owner);
//		}
	}
	#[export]
	fn _process(&mut self, owner: &Spatial, _delta: f64){
		let player = unsafe {
			owner.get_node("../KinematicBody").unwrap().assume_safe().cast::<KinematicBody>().unwrap()
		};
		let label = unsafe {
			owner.get_node("../Label").unwrap().assume_safe().cast::<Label>().unwrap()
		};
//		let player_old_pos: Vec<i32;3usize> = player.translation();
		let area:i32 = 4i32;
		
		let p_pos: Vec<i32> = vec![
		(player.translation().x / 10.0f32) as i32,
		(player.translation().y / 10.0f32) as i32,
		(player.translation().z / 10.0f32) as i32];
		
		label.set_text(format!("Vector3(x{}, y{}, z{})\nVector3(cx{}, cy{}, cz{}\nVector3(ax{} {}, ay{} {}, az{} {})",
			player.translation().x, player.translation().y, player.translation().z,
			p_pos[0], p_pos[1], p_pos[2],
			p_pos[0] + area,p_pos[0] - area,
			p_pos[1] + area,p_pos[1] - area,
			p_pos[2] + area,p_pos[2] - area
		));
//		{

		
//		godot_print!("{}..{}",p_pos[0] + area,p_pos[0] - area);
			for x in p_pos[0] - area..p_pos[0] + area{
				for y in p_pos[1] - area..p_pos[1] + area{
					for z in p_pos[2] - area..p_pos[2] + area{
//						godot_print!("i have aids");
						self.add_chunk(owner,x,y,z);
					}
				}
			}
//		}
	}

}


fn init(handle: InitHandle) {
	handle.add_tool_class::<VoxelTerrain>();
}

godot_init!(init);
