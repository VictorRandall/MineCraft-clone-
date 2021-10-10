//use std;
//use rand::Rng;
use gdnative::api::{ArrayMesh, Mesh, MeshInstance, OpenSimplexNoise, SurfaceTool, Spatial, StaticBody, Material};
use gdnative::prelude::*;

mod voxel;

use voxel::Voxel;

//#[derive(NativeClass, Debug)]
//#[no_constructor]
#[derive(Debug)]
pub struct VoxelChunk{
	pos:Vector3,
	size:usize,
	data: Vec<Vec<Vec<u16>>>,
	should_remove:bool,
	seed:i64,
	//seed2:i64,
}

//#[methods]
impl VoxelChunk{
//	#[export]
	pub fn new(position:Vector3,s:usize,nseed:i64) -> Self {//,nseed2:i64
		VoxelChunk{
			pos:position,
			size:s,
			data: vec![vec![vec![0u16;s];s];s],//the default value is 8
			should_remove:false,
			seed:nseed,
			//seed2:nseed2,
		}
	}
	
	pub fn get_position(&self) -> Vec<f32>{
		return vec![self.pos.x,self.pos.y,self.pos.z]
	}
	
	pub fn set_should_remove(&mut self, state:bool){
		self.should_remove = state
	}
	
	pub fn get_should_remove(&self) -> bool{
		return self.should_remove
	}
	
	pub fn get_voxel(&self,x:f32,y:f32,z:f32) -> u16 {
		if x < self.size as f32	&& x > 0.0f32 && y < self.size as f32 && y > 0.0f32 && z < self.size as f32 && z > 0.0f32 {
			self.data[x as usize][y as usize][z as usize]
		} else { 0u16 }
	}
	
	pub fn set_voxel(&mut self,x:f32,y:f32,z:f32, id:u16){
		self.data[x as usize][y as usize][z as usize] = id
	}

	pub fn generate(&mut self, owner: &Spatial){//,x: i32,y: i32,z: i32
//		godot_print!("{}",self.update);
//		if self.update == true{
//			std::thread::spawn(||{
				let noise = OpenSimplexNoise::new();
//				let noise2 = OpenSimplexNoise::new();
				let meshinst = MeshInstance::new();
//				let meshinst = unsafe {
//					owner.get_node("MeshInstance").unwrap().assume_safe().cast::<MeshInstance>().unwrap()
//				};
				noise.set_seed(self.seed);
//				noise.set_octaves(5i64);
//				noise.set_period(100.0f64);
//				noise2.set_seed(self.seed2);
//				noise2.set_octaves(2i64);
				for x in 0..self.size as i32{
					for y in 0..self.size as i32{
						for z in 0..self.size as i32{
							if 
//							noise.get_noise_2d(x as f64 + self.pos.x as f64, z as f64 + self.pos.z as f64)
//							*
							noise.get_noise_3d(x as f64 + self.pos.x as f64, y as f64 + self.pos.y as f64, z as f64 + self.pos.z as f64)
//							noise.get_noise_3d(x as f64 + self.pos.x as f64, y as f64 + self.pos.y as f64, z as f64 + self.pos.z as f64)
							*30f64+50f64 > y as f64 + self.pos.y as f64{//+noise2.get_noise_2d(x as f64 + self.pos.x as f64, z as f64 + self.pos.z as f64)
//									self.data[x as usize][y as usize][z as usize] = 1u16;
									self.set_voxel(x as f32,y as f32,z as f32, 1u16);
							}
								
							if noise.get_noise_3d(
								x as f64 + self.pos.x as f64,
								y as f64 + self.pos.y as f64,
								z as f64 + self.pos.z as f64) < -0.25{
									self.data[x as usize][y as usize][z as usize] = 0u16;
								self.set_voxel(x as f32,y as f32 - 10.0f32,z as f32, 0u16);
									
							}//else{
//								self.data[x as usize ][y as usize][z as usize] = 0u16;
//							}
						}
					}
				}
//				godot_print!("{:#?}", self.data);
				meshinst.set_mesh(self.generate_mesh().expect("onosecond"));
				meshinst.set_translation(self.pos);
				meshinst.set_name(format!("chunk {} {} {}",self.pos.x,self.pos.y,self.pos.z));
				meshinst.create_trimesh_collision();
				owner.add_child(meshinst,true);
//				self.update = false;
//			});
//		}
	}

	pub fn generate_mesh(&self) -> Option<gdnative::Ref<ArrayMesh>>{
		let st = SurfaceTool::new();

		st.begin(Mesh::PRIMITIVE_TRIANGLES);
//		st.begin(Mesh::PRIMITIVE_LINES);
		
//		let material = ;//.assume_safe()

		for x in 0..self.size as i32{
			for y in 0..self.size as i32{
				for z in 0..self.size as i32{
					&self.custom_voxel(
						&st, 
						Vector3::new(x as f32,y as f32,z as f32),
						vec![4u8,4u8],
//						Vector2::new(1.0,0.0),
					);
//					godot_print!("{},{},{}",x,y,z);
				}
			}
		}
		
		st.set_material(ResourceLoader::godot_singleton().load(
            GodotString::from_str("res://assets/new_spatialmaterial.tres"),
            GodotString::from_str("Resource"), false).unwrap().cast::<Material>().unwrap());
//		st.set_material(material as SubClass<gdnative::prelude::Object, RefKind = true>);
		st.generate_normals(false);
		let mesh: Ref<ArrayMesh> = st.commit(gdnative::Null::null(), Mesh::ARRAY_COMPRESS_DEFAULT).unwrap();
		return Some(mesh);
	}

	pub fn remove_chunk_node(&mut self, owner: &Spatial){
		godot_print!("deleted node: 'chunk {} {} {}'",self.pos.x,self.pos.y,self.pos.z);
		self.data.clear();
		unsafe {
			owner.get_node(format!("chunk {} {} {}",self.pos.x,self.pos.y,self.pos.z)).unwrap().assume_safe().cast::<MeshInstance>().unwrap().queue_free();
		};
	}

	fn custom_voxel(&self,st:&Ref<SurfaceTool, Unique>, pos:Vector3,size:Vec<u8>){
		
		let offset_x:f32 = pos.x;
		let offset_y:f32 = pos.y;
		let offset_z:f32 = pos.z;

		let uv_x:f32 = size[0] as f32;
		let uv_y:f32 = size[1] as f32;
		
//		let OS_uv_x:f32 = offset.x;
//		let OS_uv_y:f32 = offset.y;

		if self.get_voxel(offset_x,offset_y, offset_z) == 0u16{
//			godot_print!("the id is 0");
			return;		
		}
//		godot_print!("the id is 1");
//		
//		

		//top
//		godot_print!("pos = Vector3({},{},{})",offset_x as usize,(offset_y + 1.0f32) as usize,offset_z as usize);
		if self.get_voxel(offset_x,offset_y + 1.0f32, offset_z) == 0u16{
//			godot_print!("top t");
			st.add_uv(Vector2::new(0.0 / uv_x, 0.0 / uv_x));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(1.0 / uv_x, 0.0 / uv_y));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.0 / uv_x, 1.0 / uv_y));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,1.0+offset_z));

			st.add_uv(Vector2::new(0.0, 1.0 / uv_x));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(1.0 / uv_x, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(1.0 / uv_x, 1.0 / uv_x));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,1.0+offset_z));
		}//else{godot_print!("top f");}
			
		//botton
		if self.get_voxel(offset_x,offset_y - 1.0f32, offset_z) == 0u16{
//			godot_print!("botton t");
//			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,1.0+offset_z));
//			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_uv(Vector2::new(1.0 / uv_x, 1.0 / uv_x));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,0.0+offset_z));
//			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_uv(Vector2::new(0.0, 1.0 / uv_x));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,0.0+offset_z));

			st.add_uv(Vector2::new(1.0 / uv_x, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(1.0 / uv_x, 1.0 / uv_x));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,1.0+offset_z));
		}//else{godot_print!("botton f");}

	//	right
		if self.get_voxel(offset_x + 1.0f32, offset_y, offset_z) == 0u16{
//			godot_print!("left t");
			st.add_uv(Vector2::new(0.25, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,0.0+offset_z));

			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,0.0+offset_z));
		}//else{godot_print!("left f");}

	//	left
		if self.get_voxel(offset_x - 1.0f32, offset_y, offset_z) == 0u16{
//			godot_print!("right t");
			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.25));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,0.0+offset_z));

			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.25));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,0.0+offset_z));
		}//else{godot_print!("right f");}
		
	//	back
		if self.get_voxel(offset_x, offset_y, offset_z + 1.0f32) == 0u16{
//			godot_print!("front t");
			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,1.0+offset_z));

			st.add_uv(Vector2::new(0.25, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,1.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,1.0+offset_z));
		}//else{godot_print!("front f");}

	//	front
		if self.get_voxel(offset_x, offset_y, offset_z - 1.0f32) == 0u16{
//			godot_print!("back t");
			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.25));
			st.add_vertex(Vector3::new(0.0+offset_x,0.0+offset_y,0.0+offset_z));
			
			st.add_uv(Vector2::new(0.0, 0.0));
			st.add_vertex(Vector3::new(1.0+offset_x,1.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.25, 0.0));
			st.add_vertex(Vector3::new(0.0+offset_x,1.0+offset_y,0.0+offset_z));
			st.add_uv(Vector2::new(0.0, 0.25));
			st.add_vertex(Vector3::new(1.0+offset_x,0.0+offset_y,0.0+offset_z));
		}//else{godot_print!("back f");}
	}
}	
