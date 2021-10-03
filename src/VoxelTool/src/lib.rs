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
			chunk_size: 5i32,
//			material:mtrl
		}
	}

	//FIXME: this function doesnt return the real voxel data
//	#[export]
//	fn get_voxel(&self, owner: &Spatial, x:i32, y:i32, z:i32) -> i16{
//		return 1i16
//	}
//
	fn add_chunk(&mut self,owner: &Spatial,x:i32, y:i32, z:i32){
		if self.chunks.contains_key(&format!("{},{},{}", x, y, z)){
			return;
		}
//		godot_print!("generated chunk at {},{},{}", x, y, z);
		self.chunks.insert(
			format!("{},{},{}", x, y, z),
			VoxelChunk::new(Vector3::new(x as f32 * self.chunk_size as f32 - 1.0f32,y as f32 * self.chunk_size as f32 - 1.0f32,z as f32 * self.chunk_size as f32 - 1.0f32),
			(self.chunk_size + 1) as usize, 
			self.seed)
		);//, self.seed2
		self.chunks.get_mut(&format!("{},{},{}", x, y, z)).unwrap().generate(&owner);
	}


	fn get_chunk(&mut self, owner: &Spatial, key:String) -> Option<&mut VoxelChunk>{//x:i32, y:i32, z:i32
//		return self.chunks.get_mut(&format!("{},{},{}", x, y, z));
		return self.chunks.get_mut(&key);
	}

	#[export]
	fn remove_chunk(&mut self, owner: &Spatial,x:i32, y:i32, z:i32){
		let chunk = self.get_chunk(owner,format!("{},{},{}", x, y, z)).expect("this chunk doesnt exist");
		chunk.remove_chunk_node(owner);
		
		self.chunks.remove(&format!("{},{},{}", x, y, z));
	}

	#[export]
	fn _ready(&mut self, owner: &Spatial) {
		let player = unsafe {
			owner.get_node("../KinematicBody").unwrap().assume_safe().cast::<KinematicBody>().unwrap()
		};
		
		let p_pos: Vec<i32> = vec![
		(player.translation().x / self.chunk_size as f32) as i32,
		(player.translation().y / self.chunk_size as f32) as i32,
		(player.translation().z / self.chunk_size as f32) as i32];
		
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
		let area:i32 = 2i32;
//		let mut more_text:String = "no chunk has to be removed".to_string();
		
		let p_pos: Vec<i32> = vec![
		(player.translation().x / self.chunk_size as f32) as i32,
		(player.translation().y / self.chunk_size as f32) as i32,
		(player.translation().z / self.chunk_size as f32) as i32];
		
		label.set_text(format!("Vector3(x{}, y{}, z{})\nVector3(cx{}, cy{}, cz{})\nVector3(ax{} {}, ay{} {}, az{} {})",
			player.translation().x, player.translation().y, player.translation().z,
			p_pos[0], p_pos[1], p_pos[2],
			p_pos[0] + area,p_pos[0] - area,
			p_pos[1] + area,p_pos[1] - area,
			p_pos[2] + area,p_pos[2] - area
//			more_text
		));
//		{
			for (key,value) in &mut self.chunks {
				value.set_should_remove(true);
//				godot_print!("{}",value.get_should_remove());
			}
			
			
			for x in p_pos[0] - area..p_pos[0] + area{
				for y in p_pos[1] - area..p_pos[1] + area{
					for z in p_pos[2] - area..p_pos[2] + area{
//						godot_print!("i have aids");
						self.add_chunk(owner,x,y,z);
//						self.get_chunk(owner,x,y,z).unwrap().set_should_remove(false);
						self.get_chunk(owner,format!("{},{},{}",x,y,z)).unwrap().set_should_remove(false);
//						godot_print!("{}",self.get_chunk(owner,format!("{},{},{}",x,y,z)).unwrap().get_should_remove());
					}
				}
			}
			
			for (key, mut value) in self.chunks.iter_mut() {
//				godot_print!("{} / {}", key, value);
//				self.chunks.remove(&key);
				value.remove_chunk_node(owner);
			}
//			
//			for (key, value) in self.chunks.iter_mut() {
//			for key in 0..self.chunks.len() as i32{
//				if value.get_should_remove() == true{// &&
//					godot_print!("{}",value.get_should_remove());
//					godot_print!("the chunk: 'chunk {} {} {}' can be removed",
//						value.get_position()[0],
//						value.get_position()[1],
//						value.get_position()[2]);
//					value.get_position()[0] < (p_pos[0] - area) as f32 &&
//					value.get_position()[0] > (p_pos[0] + area) as f32 &&
//					value.get_position()[1] < (p_pos[1] - area) as f32 &&
//					value.get_position()[1] > (p_pos[1] + area) as f32 &&
//					value.get_position()[2] < (p_pos[2] - area) as f32 &&
//					value.get_position()[2] > (p_pos[2] + area) as f32{
//					more_text = format!("the chunk: 'chunk {} {} {}' has to be removed",
//						value.get_position()[0],
//						value.get_position()[1],
//						value.get_position()[2]);
					
//					key.remove_chunk(&owner, value.get_position()[0] as i32, value.get_position()[1] as i32, value.get_position()[2] as i32);
//					self.chunks.remove(key);
//				}
//			}
				
			
			
//				godot_print!("key is {} and the position of the chunk is Vector3({},{},{})", key, value.get_position()[0],value.get_position()[1],value.get_position()[2]);
//				self.get_chunk(owner,key.to_string()).unwrap().set_should_remove(true);
				
//				for x in p_pos[0] - area..p_pos[0] + area{
//					for y in p_pos[1] - area..p_pos[1] + area{
//						for z in p_pos[2] - area..p_pos[2] + area{
//							if key == &format!("{},{},{}",x,y,z){
//								self.get_chunk().set_should_remove(false);
//							}
//						}
//					}
//				}
				
		
//		godot_print!("{}..{}",p_pos[0] + area,p_pos[0] - area);
//		}
	}

}


fn init(handle: InitHandle) {
	handle.add_tool_class::<VoxelTerrain>();
}

godot_init!(init);
