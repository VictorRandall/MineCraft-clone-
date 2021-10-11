use std;
use rand::Rng;
use gdnative::api::{ArrayMesh, Mesh, MeshInstance, OpenSimplexNoise, SurfaceTool, Spatial, StaticBody, Material};
use gdnative::prelude::*;

//struct ChunkPos{
//	x: i32,
//	y: i32,
//	z: i32
//}

struct Vertex{
	uv: Vector2,
	vertex: Vector3
}

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct VoxelWorld{
	chunks: std::collections::HashMap<String, VoxelChunk>,
	chunk_size: u8,
	seed: i64
}

#[methods]
impl VoxelWorld {
    fn new(_owner: &Spatial) -> Self {
        VoxelWorld{
        	chunks: std::collections::HashMap::new(),
        	chunk_size: 16u8,
        	seed: rand::thread_rng().gen()
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Spatial) {
    	for x in 0i32..4i32{
			for y in 0i32..4i32{
				for z in 0i32..4i32{
					godot_print!("{},{},{}", x, y, z);
					self.chunks.insert(
						format!("{},{},{}", x, y, z),
						VoxelChunk::new(Vector3::new(x as f32 * (self.chunk_size as f32 - 1.0f32), y as f32 * (self.chunk_size as f32 - 1.0f32), z as f32 * (self.chunk_size as f32 - 1.0f32)),
						self.chunk_size, 
						self.seed));
					self.chunks.get_mut(&format!("{},{},{}", x, y, z)).unwrap().generate(&owner)
				}
			}
		}
    }
    
    #[export]
    fn _process(&mut self, owner: &Spatial, _delta: f64){
    }
    
   	#[export]
   	fn set_voxel(&mut self, owner: &Spatial,pos:Vector3){
   		let cpos_x: f32 = (pos.x.floor() / self.chunk_size as f32).floor();
		let cpos_y: f32 = (pos.y.floor() / self.chunk_size as f32).floor();
		let cpos_z: f32 = (pos.z.floor() / self.chunk_size as f32).floor();
		
		let bpos_x: f32 = pos.x.floor() % self.chunk_size as f32;
		let bpos_y: f32 = pos.y.floor() % self.chunk_size as f32;
		let bpos_z: f32 = pos.z.floor() % self.chunk_size as f32;
		
		godot_print!("the chunk position is {}, {}, {}\nthe block position is {}, {}, {}", cpos_x, cpos_y, cpos_z, bpos_x, bpos_y, bpos_z);
		
//		self.chunks.get_mut(&format!("{},{},{}", x, y, z)).unwrap().generate(&owner)
		unsafe {
			owner.get_node(format!("chunk {} {} {}",cpos_x,cpos_y,cpos_z)).unwrap().assume_safe().cast::<MeshInstance>().unwrap()
			.set_mesh(self.chunks.get_mut(&format!("{},{},{}", cpos_x, cpos_y, cpos_z)).unwrap().generate_mesh());
		};
   	}
}

pub struct VoxelChunk{
	position: Vector3,
	data: Vec<Vec<Vec<u8>>>,
	size: u8,
	seed: i64
}

impl VoxelChunk{
	pub fn new(pos:Vector3,size: u8, seed: i64) -> Self{
		VoxelChunk{
			position: pos,
			data: vec![vec![vec![0u8;size as usize];size as usize];size as usize],
			size: size,
			seed: seed
		}
	}
	
	pub fn generate(&mut self,owner: &Spatial){
		let noise = OpenSimplexNoise::new();
		let meshinst = MeshInstance::new();
		noise.set_seed(self.seed);
		for x in 0..self.size as i32{
			for y in 0..self.size as i32{
				for z in 0..self.size as i32{
					if noise.get_noise_3d(x as f64 + self.position.x as f64, y as f64 + self.position.y as f64, z as f64 + self.position.z as f64)*30f64+30f64 > y as f64 + self.position.y as f64{
						self.data[x as usize][y as usize][z as usize] = 1u8;
					}
					//caves 👇️
//					if noise.get_noise_3d(
//						x as f64 + self.position.x as f64,
//						y as f64 + self.position.y as f64,
//						z as f64 + self.position.z as f64) < -0.25{
//							self.data[x as usize][y as usize][z as usize] = 0u16;
//					}
				}
			}
		}
		meshinst.set_mesh(self.generate_mesh());
		meshinst.set_translation(self.position);
		meshinst.set_name(format!("chunk {} {} {}",self.position.x,self.position.y,self.position.z));
		meshinst.create_trimesh_collision();
		owner.add_child(meshinst,true);
	}
	
	fn generate_mesh(&self) -> gdnative::Ref<ArrayMesh>{
		let grass:Vec<Vec<Vertex>> = vec![ //grass
		    							vec![//top
											Vertex{uv: Vector2::new(1.0, 0.0),vertex: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(2.0, 0.0),vertex: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 1.0),vertex: Vector3::new(0.0,1.0,1.0)},
											
											Vertex{uv: Vector2::new(1.0, 1.0),vertex: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(2.0, 0.0),vertex: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(2.0, 1.0),vertex: Vector3::new(1.0,1.0,1.0)}
		    							],
		    							
		    							vec![//botton
		    								Vertex{uv: Vector2::new(2.0, 0.0),vertex: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(3.0, 1.0),vertex: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(2.0, 1.0),vertex: Vector3::new(0.0,0.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(3.0, 0.0),vertex: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(3.0, 1.0),vertex: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(2.0, 0.0),vertex: Vector3::new(0.0,0.0,1.0)}
		    							],
		    							
		    							vec![//right
		    								Vertex{uv: Vector2::new(1.0, 1.0), vertex: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(0.0, 1.0), vertex: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(1.0, 0.0), vertex: Vector3::new(1.0,1.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(0.0, 1.0), vertex: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(0.0, 0.0), vertex: Vector3::new(1.0,1.0,1.0)},
		    								Vertex{uv: Vector2::new(1.0, 0.0), vertex: Vector3::new(1.0,1.0,0.0)}
		    							],
		    							
		    							vec![//left
		    								Vertex{uv: Vector2::new(0.0, 0.0), vertex: Vector3::new(0.0,1.0,0.0)},
		    								Vertex{uv: Vector2::new(1.0, 1.0), vertex: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(0.0, 1.0), vertex: Vector3::new(0.0,0.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(1.0, 0.0), vertex: Vector3::new(0.0,1.0,1.0)},
		    								Vertex{uv: Vector2::new(1.0, 1.0), vertex: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(0.0, 0.0), vertex: Vector3::new(0.0,1.0,0.0)}
		    							],
		    							
		    							vec![//back
											Vertex{uv: Vector2::new(0.0,1.0), vertex: Vector3::new(0.0,0.0,1.0)},
											Vertex{uv: Vector2::new(0.0,0.0), vertex: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(1.0,1.0), vertex: Vector3::new(1.0,0.0,1.0)},
											
											Vertex{uv: Vector2::new(1.0,1.0), vertex: Vector3::new(1.0,0.0,1.0)},
											Vertex{uv: Vector2::new(0.0,0.0), vertex: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(1.0,0.0), vertex: Vector3::new(1.0,1.0,1.0)},
		    							],
		    							
		    							vec![//front
											Vertex{uv: Vector2::new(0.0,1.0), vertex: Vector3::new(1.0,0.0,0.0)},
											Vertex{uv: Vector2::new(1.0,0.0), vertex: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(1.0,1.0), vertex: Vector3::new(0.0,0.0,0.0)},
											
											Vertex{uv: Vector2::new(0.0,0.0), vertex: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(1.0,0.0), vertex: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(0.0,1.0), vertex: Vector3::new(1.0,0.0,0.0)},
		    							]
		    						];
		
		let st = SurfaceTool::new();
		
		st.begin(Mesh::PRIMITIVE_TRIANGLES);
		
		for x in 0..self.size{
			for y in 0..self.size{
				for z in 0..self.size{
					self.build_voxel(&st, &grass, Vector3::new(x as f32,y as f32,z as f32), vec![4u8,4u8]);
				}
			}
		}
		
		st.set_material(ResourceLoader::godot_singleton().load(
            GodotString::from_str("res://assets/new_spatialmaterial.tres"),
            GodotString::from_str("Resource"), false).unwrap().cast::<Material>().unwrap());
        st.generate_normals(false);
        
        return st.commit(gdnative::Null::null(), Mesh::ARRAY_COMPRESS_DEFAULT).unwrap()
	}
	
	fn build_voxel(&self, st: &Ref<SurfaceTool, Unique>, mesh:&Vec<Vec<Vertex>> ,pos: Vector3, size: Vec<u8>){
		if self.get_voxel(Vector3::new(pos.x,pos.y,pos.z)) == 0u8{ return }
    
		if self.get_voxel(Vector3::new(pos.x,pos.y + 1.0f32,pos.z)) == 0u8{
		    for v in &mesh[0]{//top
		    	st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[1] as f32));
		    	st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
		    }
        }
        
        if self.get_voxel(Vector3::new(pos.x,pos.y - 1.0f32,pos.z)) == 0u8{
			for v in &mesh[1]{//botton
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[1] as f32));
				st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
			}
		}
		
		if self.get_voxel(Vector3::new(pos.x + 1.0f32,pos.y,pos.z)) == 0u8{
			for v in &mesh[2]{//right
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[1] as f32));
				st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
			}
		}
		
		if self.get_voxel(Vector3::new(pos.x - 1.0f32,pos.y,pos.z)) == 0u8{
			for v in &mesh[3]{//left
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[1] as f32));
				st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
			}
		}
		
		if self.get_voxel(Vector3::new(pos.x,pos.y,pos.z + 1.0f32)) == 0u8{
			for v in &mesh[4]{//back
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[1] as f32));
				st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
			}
		}
		
		if self.get_voxel(Vector3::new(pos.x,pos.y,pos.z - 1.0f32)) == 0u8{
			for v in &mesh[5]{//front
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[1] as f32));
				st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
			}
		}
	}
	
	pub fn get_voxel(&self,pos:Vector3) -> u8{
		if pos.x < self.size as f32	&& pos.x > 0.0f32 && pos.y < self.size as f32 && pos.y > 0.0f32 && pos.z < self.size as f32 && pos.z > 0.0f32 {
			self.data[pos.x as usize][pos.y as usize][pos.z as usize]
		} else { 0u8 }
	}
}

fn init(handle: InitHandle) {
    handle.add_tool_class::<VoxelWorld>();
}

godot_init!(init);

