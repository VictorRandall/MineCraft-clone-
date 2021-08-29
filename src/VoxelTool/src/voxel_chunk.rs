//use std;
//use rand::Rng;
use gdnative::api::{ArrayMesh, Mesh, MeshInstance, OpenSimplexNoise, SurfaceTool, Spatial, StaticBody};
use gdnative::prelude::*;

mod voxel;

use voxel::Voxel;

//#[derive(NativeClass, Debug)]
//#[no_constructor]
pub struct VoxelChunk{
	pos:Vector3,
	size:usize,
	data: Vec<Vec<Vec<u16>>>,
	update:bool,
}

//#[methods]
impl VoxelChunk{
//	#[export]
	pub fn new(position:Vector3,s:usize) -> Self {
		VoxelChunk{
			pos:position,
			size:s,
			data: vec![vec![vec![0u16;s];s];s],//the default value is 8
			update:true,
		}
	}

	pub fn get_voxel(&self,x:f32,y:f32,z:f32) -> u16 {
		if x < self.size as f32 && y < self.size as f32 && z < self.size as f32 {
			self.data[x][y][z]
		} else { 0u16 }
	}

	pub fn start(&mut self, owner: &Spatial){
//		godot_print!("{}",self.update);
		if self.update == true{
//			std::thread::spawn(||{
//				let noise = OpenSimplexNoise::new();
				let meshinst = MeshInstance::new();
//				let meshinst = unsafe {
//					owner.get_node("MeshInstance").unwrap().assume_safe().cast::<MeshInstance>().unwrap()
//				};
				for x in 0..self.size as i32{
					for y in 0..self.size as i32{
						for z in 0..self.size as i32{
							if 8.0f64 > y as f64{ //noise.get_noise_2d(x as f64, z as f64)*5f64+10f64{
								self.data[x as usize][y as usize][z as usize] = 1u16;
							}//else{
//								self.data[x as usize ][y as usize][z as usize] = 0u16;
//							}
						}
					}
				}
//				godot_print!("{:#?}", self.data);
				meshinst.set_mesh(self.chunk_mesh().expect("onosecond"));
				meshinst.set_name(format!("chunk{}{}{}",self.pos.x,self.pos.y,self.pos.z));
				owner.add_child(meshinst,true);
				self.update = false;
//			});
		}
	}

	fn chunk_mesh(&self) -> Option<gdnative::Ref<ArrayMesh>>{
		let st = SurfaceTool::new();

		st.begin(Mesh::PRIMITIVE_TRIANGLES);
//		st.begin(Mesh::PRIMITIVE_LINES);
		

		for x in 0..self.size as i32{
			for y in 0..self.size as i32{
				for z in 0..self.size as i32{
					&self.custom_voxel(
						&st, 
						Vector3::new(x as f32 + (self.pos.x * self.size as f32),y as f32  + (self.pos.y * self.size as f32),z as f32 + (self.pos.z * self.size as f32)),
					);
//					godot_print!("{},{},{}",x,y,z)
				}
			}
		}
		

		st.generate_normals(false);
		let mesh: Ref<ArrayMesh> = st.commit(gdnative::Null::null(), Mesh::ARRAY_COMPRESS_DEFAULT).unwrap();
		return Some(mesh);
	}

//	fn restart(&mut self, owner: &Spatial){
//		self.data.clear();
//		godot_print!("chunk{}{}{}",self.pos.x,self.pos.y,self.pos.z);
//		unsafe {
//			owner.get_node(format!("chunk{}{}{}",self.pos.x,self.pos.y,self.pos.z)).unwrap().assume_safe().cast::<MeshInstance>().unwrap().queue_free();
//		};
//	}

	fn custom_voxel(&self,st:&Ref<SurfaceTool, Unique>, pos:Vector3){
		
		let offset_x:f32 = pos.x;
		let offset_y:f32 = pos.y;
		let offset_z:f32 = pos.z;

		let offset_uv_x:f32 = 0.0;
		let offset_uv_y:f32 = 0.0;

		if self.get_voxel(offset_x as usize,offset_y as usize, offset_z as usize) == 0u16{
//			godot_print!("the id is 0");
			return;
		}
//		godot_print!("the id is 1");
//		
//		

		//top
//		godot_print!("pos = Vector3({},{},{})",offset_x as usize,(offset_y + 1.0f32) as usize,offset_z as usize);
		if self.get_voxel(offset_x as usize,(offset_y + 1.0f32) as usize, offset_z as usize) == 0u16{
//			godot_print!("top t");
			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,1.0+offset_z));

			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,1.0+offset_z));
		}//else{godot_print!("top f");}
			
		//botton
		if self.get_voxel(offset_x as usize,(offset_y + 1.0f32) as usize, offset_z as usize) == 0u16{
//			godot_print!("botton t");
			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,0.0+offset_z));

			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.25));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,1.0+offset_z));
		}//else{godot_print!("botton f");}

	//	left
		if self.get_voxel((offset_x + 1.0f32) as usize, offset_y as usize, offset_z as usize) == 0u16{
//			godot_print!("left t");
			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,0.0+offset_z));

			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,0.0+offset_z));
		}//else{godot_print!("left f");}

	//	right
		if self.get_voxel((offset_x + 1.0f32) as usize, offset_y as usize, offset_z as usize) == 0u16{
//			godot_print!("right t");
			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,0.0+offset_z));

			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.25));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,0.0+offset_z));
		}//else{godot_print!("right f");}
		
	//	front
		if self.get_voxel(offset_x as usize, offset_y as usize, (offset_z + 1.0f32) as usize) == 0u16{
//			godot_print!("front t");
			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,1.0+offset_z));

			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,1.0+offset_z));
		}//else{godot_print!("front f");}

	//	back
		if self.get_voxel(offset_x as usize, offset_y as usize, (offset_z - 1.0f32) as usize) == 0u16{
//			godot_print!("back t");
			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,0.0+offset_z));
			
			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,0.0+offset_z));
		}//else{godot_print!("back f");}
	}
}
