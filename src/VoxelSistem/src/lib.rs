use std;
use gdnative::prelude::*;
use gdnative::api::{ArrayMesh, Mesh, MeshInstance, OpenSimplexNoise, SurfaceTool, Spatial, StaticBody, Material};

struct Vertex{
	uv: Vector2,
	v: Vector3
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ChunkPos{
	x: i32,
	y: i32,
	z: i32
}

struct Voxel{
	mesh: Option<Vec<Vec<Vertex>>>,
	solid: bool,
	name: String,
	data: Vec<String>
}

struct VoxelChunk{
	data: Vec<Vec<Vec<u16>>>,
	should_remove: bool
}

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct VoxelSistem{
	block_types: Vec<Voxel>,
	chunks: std::collections::HashMap<ChunkPos, VoxelChunk>,
	chunk_size: u8
}

#[methods]
impl VoxelSistem {
	fn new(_owner: &Spatial) -> Self {
		VoxelSistem{
			block_types: vec![
				Voxel{
					mesh: None,
					solid: false,
					name: "Air".to_string(),
					data: Vec::new()
				},
				Voxel{
					mesh: Some(vec![
		    							vec![//top
											Vertex{uv: Vector2::new(1.0, 0.0), v: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(2.0, 0.0), v: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(0.0,1.0,1.0)},
											
											Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(2.0, 0.0), v: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(2.0, 1.0), v: Vector3::new(1.0,1.0,1.0)}
		    							],
		    							
		    							vec![//botton
		    								Vertex{uv: Vector2::new(2.0, 0.0), v: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(3.0, 1.0), v: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(2.0, 1.0), v: Vector3::new(0.0,0.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(3.0, 0.0), v: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(3.0, 1.0), v: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(2.0, 0.0), v: Vector3::new(0.0,0.0,1.0)}
		    							],
		    							
		    							vec![//right
		    								Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(0.0, 1.0), v: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(1.0, 0.0), v: Vector3::new(1.0,1.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(0.0, 1.0), v: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(0.0, 0.0), v: Vector3::new(1.0,1.0,1.0)},
		    								Vertex{uv: Vector2::new(1.0, 0.0), v: Vector3::new(1.0,1.0,0.0)}
		    							],
		    							
		    							vec![//left
		    								Vertex{uv: Vector2::new(0.0, 0.0), v: Vector3::new(0.0,1.0,0.0)},
		    								Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(0.0, 1.0), v: Vector3::new(0.0,0.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(1.0, 0.0), v: Vector3::new(0.0,1.0,1.0)},
		    								Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(0.0, 0.0), v: Vector3::new(0.0,1.0,0.0)}
		    							],
		    							
		    							vec![//back
											Vertex{uv: Vector2::new(0.0, 1.0), v: Vector3::new(0.0,0.0,1.0)},
											Vertex{uv: Vector2::new(0.0, 0.0), v: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(1.0,0.0,1.0)},
											
											Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(1.0,0.0,1.0)},
											Vertex{uv: Vector2::new(0.0, 0.0), v: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(1.0, 0.0), v: Vector3::new(1.0,1.0,1.0)},
		    							],
		    							
		    							vec![//front
											Vertex{uv: Vector2::new(0.0, 1.0), v: Vector3::new(1.0,0.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 0.0), v: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(0.0,0.0,0.0)},
											
											Vertex{uv: Vector2::new(0.0, 0.0), v: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 0.0), v: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(0.0, 1.0), v: Vector3::new(1.0,0.0,0.0)},
		    							]
		    						]),
					solid: false,
					name: "Grass".to_string(),
					data: Vec::new()
				},
				Voxel{
					mesh: Some(vec![ //dirt
		    							vec![//top
											Vertex{uv: Vector2::new(2.0, 0.0), v: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(3.0, 0.0), v: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(2.0, 1.0), v: Vector3::new(0.0,1.0,1.0)},
											
											Vertex{uv: Vector2::new(2.0, 1.0), v: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(3.0, 0.0), v: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(3.0, 1.0), v: Vector3::new(1.0,1.0,1.0)}
		    							],
		    							
		    							vec![//botton
		    								Vertex{uv: Vector2::new(2.0, 0.0), v: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(3.0, 1.0), v: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(2.0, 1.0), v: Vector3::new(0.0,0.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(3.0, 0.0), v: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(3.0, 1.0), v: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(2.0, 0.0), v: Vector3::new(0.0,0.0,1.0)}
		    							],
		    							
		    							vec![//right
		    								Vertex{uv: Vector2::new(3.0, 1.0), v: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(2.0, 1.0), v: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(3.0, 0.0), v: Vector3::new(1.0,1.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(2.0, 1.0), v: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(2.0, 0.0), v: Vector3::new(1.0,1.0,1.0)},
		    								Vertex{uv: Vector2::new(3.0, 0.0), v: Vector3::new(1.0,1.0,0.0)}
		    							],
		    							
		    							vec![//left
		    								Vertex{uv: Vector2::new(2.0, 0.0), v: Vector3::new(0.0,1.0,0.0)},
		    								Vertex{uv: Vector2::new(3.0, 1.0), v: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(2.0, 1.0), v: Vector3::new(0.0,0.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(2.0, 0.0), v: Vector3::new(0.0,1.0,1.0)},
		    								Vertex{uv: Vector2::new(2.0, 1.0), v: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(3.0, 0.0), v: Vector3::new(0.0,1.0,0.0)}
		    							],
		    							
		    							vec![//back
											Vertex{uv: Vector2::new(2.0, 1.0), v: Vector3::new(0.0,0.0,1.0)},
											Vertex{uv: Vector2::new(2.0, 0.0), v: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(3.0, 1.0), v: Vector3::new(1.0,0.0,1.0)},
											
											Vertex{uv: Vector2::new(3.0, 1.0), v: Vector3::new(1.0,0.0,1.0)},
											Vertex{uv: Vector2::new(2.0, 0.0), v: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(3.0, 0.0), v: Vector3::new(1.0,1.0,1.0)},
		    							],
		    							
		    							vec![//front
											Vertex{uv: Vector2::new(2.0, 1.0), v: Vector3::new(1.0,0.0,0.0)},
											Vertex{uv: Vector2::new(3.0, 0.0), v: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(3.0, 1.0), v: Vector3::new(0.0,0.0,0.0)},
											
											Vertex{uv: Vector2::new(2.0, 0.0), v: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(3.0, 0.0), v: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(2.0, 1.0), v: Vector3::new(1.0,0.0,0.0)},
		    							]
		    						]),
					solid: false,
					name: "Dirt".to_string(),
					data: Vec::new()
				},
			],
			chunks: std::collections::HashMap::<ChunkPos, VoxelChunk>::new(),
			chunk_size: 16u8
		}
	}

	#[export]
	fn _ready(&self, _owner: &Spatial) {
		godot_print!("Hello, world.");
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
	
	fn build_chunk(&mut self, pos: Vector3){
		if !self.chunks.contains_key(&ChunkPos{x: pos.x as i32,y: pos.y as i32,z: pos.z as i32}){
			let mut chunk: VoxelChunk = VoxelChunk{
										data: vec![vec![vec![0u16;self.chunk_size as usize];self.chunk_size as usize];self.chunk_size as usize],
										should_remove: false
									};
			
			let noise = OpenSimplexNoise::new();
			
			for x in 0..self.chunk_size{
				for y in 0..self.chunk_size{
					for z in 0..self.chunk_size{
						if noise.get_noise_3d(x as f64 + pos.x as f64, y as f64 + pos.y as f64, z as f64 + pos.z as f64)*30f64+10f64 > y as f64 + pos.y as f64{
							chunk.data[x as usize][y as usize][z as usize] = 1u16;
						}
					}
				}
			}
			
			self.chunks.insert(
				ChunkPos{x: pos.x as i32,y: pos.y as i32,z: pos.z as i32}, 
				chunk
			);
		}
	
		
	
	}
	
	
	fn _process(&self, _owner: &Spatial, _delta: f64){
		
	}
	
	
	
	fn build_chunk_mesh(&self,pos: ChunkPos) -> gdnative::Ref<ArrayMesh>{
		let st = SurfaceTool::new();

		st.begin(Mesh::PRIMITIVE_TRIANGLES);
//		st.begin(Mesh::PRIMITIVE_LINES);
        
        for x in 0..self.chunk_size{
        	for y in 0..self.chunk_size{
        		for z in 0..self.chunk_size{
        		//TODO: build_voxel_mesh() should be able to get the mesh from the list of Voxel structs
					self.build_voxel_mesh(
						&st,
						Vector3::new(x as f32,y as f32,z as f32),
						self.chunks.get(&ChunkPos{x: pos.x as i32,y: pos.y as i32,z: pos.z as i32}).unwrap().data[x as usize][y as usize][z as usize],
//						&self.block_types[
//							self.chunks.get(&ChunkPos{x: pos.x as i32,y: pos.y as i32,z: pos.z as i32}).unwrap().data[x as usize][y as usize][z as usize] as usize
//						],
						&vec![4u8,4u8]);
        		}
        	}
		}
		
        st.set_material(ResourceLoader::godot_singleton().load(
            GodotString::from_str("res://assets/new_spatialmaterial.tres"),
            GodotString::from_str("Resource"), false).unwrap().cast::<Material>().unwrap());
        st.generate_normals(false);
        
        return st.commit(gdnative::Null::null(), Mesh::ARRAY_COMPRESS_DEFAULT).unwrap()
	}
	
	fn build_voxel_mesh(&self,st:&Ref<SurfaceTool, Unique>, pos:Vector3, id: u16, size:&Vec<u8>){
    	match self.block_types[id as usize].mesh{
    		None => { return },
    		Some => {
    			if self.get_cvoxel(Vector3::new(pos.x,pos.y + 1.0f32,pos.z)) == 0u16{
					for v in &self.block_types[id as usize].mesh[0]{//top
						st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
						st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
					}
				}
				
				if self.get_cvoxel(Vector3::new(pos.x,pos.y - 1.0f32,pos.z)) == 0u16{
					for v in &self.block_types[id as usize].mesh[1]{//botton
						st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
						st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
					}
				}
				
				if self.get_cvoxel(Vector3::new(pos.x + 1.0f32,pos.y,pos.z)) == 0u16{
					for v in &self.block_types[id as usize].mesh[2]{//right
						st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
						st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
					}
				}
				
				if self.get_cvoxel(Vector3::new(pos.x - 1.0f32,pos.y,pos.z)) == 0u16{
					for v in &self.block_types[id as usize].mesh[3]{//left
						st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
						st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
					}
				}
				
				if self.get_cvoxel(Vector3::new(pos.x,pos.y,pos.z + 1.0f32)) == 0u16{
					for v in &self.block_types[id as usize].mesh[4]{//back
						st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
						st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
					}
				}
				
				if self.get_cvoxel(Vector3::new(pos.x,pos.y,pos.z - 1.0f32)) == 0u16{
					for v in &self.block_types[id as usize].mesh[5]{//front
						st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
						st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
					}
				}
    		}
    	}
	}
	
	fn get_cvoxel(&self,pos: Vector3, cpos: ChunkPos){
		//FIXME: since VoxelChunk doesnt have functions anymore, this function needs to get the chunk data to do this
		if pos.x < self.chunk_size as f32 && pos.x > 0.0f32 && pos.y < self.chunk_size as f32 && pos.y > 0.0f32 && pos.z < self.chunk_size as f32 && pos.z > 0.0f32 {
			self.chunks_data[pos.x as usize][pos.y as usize][pos.z as usize]
		} else { 0u16 }
	}
}

fn init(handle: InitHandle) {
	handle.add_tool_class::<VoxelSistem>();
}

godot_init!(init);

