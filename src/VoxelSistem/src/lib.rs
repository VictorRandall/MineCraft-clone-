use std;
use rand::Rng;	
use gdnative::prelude::*;
use gdnative::api::{ArrayMesh, Mesh, MeshInstance, OpenSimplexNoise, SurfaceTool, Spatial, StaticBody, Material};

#[derive(Clone)]
struct Vertex{
	uv: Vector2,
	v: Vector3
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
struct ChunkPos{
	x: i32,
	y: i32,
	z: i32
}

//#[derive(NativeClass, Debug)]
//#[inherit(Reference)]
#[derive(Clone)]
struct Voxel{
	mesh: Option<Vec<Vec<Vertex>>>,
	solid: bool,
	name: String,
	data: Vec<String>
}

//#[methods]
//impl Voxel{
//	#[export]
//	fn new() -> Instance<Voxel, Unique>{
//		let inst = Voxel{
//			mesh: Option<Vec<Vec<Vertex>>>,
//			solid: bool,
//			name: String,
//			data: Vec<String>
//		};
//	}
//}

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct VoxelSistem{
	block_types: Vec<Voxel>,
	chunks: std::collections::HashMap<ChunkPos, VoxelChunk>,
	#[property]
	chunk_size: u8,
	seed: i64
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
											Vertex{uv: Vector2::new(0.0, 1.0), v: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(0.0, 2.0), v: Vector3::new(0.0,1.0,1.0)},
											
											Vertex{uv: Vector2::new(0.0, 2.0), v: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 2.0), v: Vector3::new(1.0,1.0,1.0)}
		    							],
		    							
		    							vec![//botton
		    								Vertex{uv: Vector2::new(2.0, 1.0), v: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(3.0, 2.0), v: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(2.0, 2.0), v: Vector3::new(0.0,0.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(3.0, 1.0), v: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(3.0, 2.0), v: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(2.0, 1.0), v: Vector3::new(0.0,0.0,1.0)}
		    							],
		    							
		    							vec![//right
		    								Vertex{uv: Vector2::new(1.0, 2.0), v: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(0.0, 2.0), v: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(1.0,1.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(0.0, 2.0), v: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(0.0, 1.0), v: Vector3::new(1.0,1.0,1.0)},
		    								Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(1.0,1.0,0.0)}
		    							],
		    							
		    							vec![//left
		    								Vertex{uv: Vector2::new(0.0, 1.0), v: Vector3::new(0.0,1.0,0.0)},
		    								Vertex{uv: Vector2::new(1.0, 2.0), v: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(0.0, 2.0), v: Vector3::new(0.0,0.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(0.0,1.0,1.0)},
		    								Vertex{uv: Vector2::new(1.0, 2.0), v: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(0.0, 1.0), v: Vector3::new(0.0,1.0,0.0)}
		    							],
		    							
		    							vec![//back
											Vertex{uv: Vector2::new(0.0, 2.0), v: Vector3::new(0.0,0.0,1.0)},
											Vertex{uv: Vector2::new(0.0, 1.0), v: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(1.0, 2.0), v: Vector3::new(1.0,0.0,1.0)},
											
											Vertex{uv: Vector2::new(1.0, 2.0), v: Vector3::new(1.0,0.0,1.0)},
											Vertex{uv: Vector2::new(0.0, 1.0), v: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(1.0, 2.0), v: Vector3::new(1.0,1.0,1.0)},
		    							],
		    							
		    							vec![//front
											Vertex{uv: Vector2::new(0.0, 2.0), v: Vector3::new(1.0,0.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 2.0), v: Vector3::new(0.0,0.0,0.0)},
											
											Vertex{uv: Vector2::new(0.0, 1.0), v: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 1.0), v: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(0.0, 2.0), v: Vector3::new(1.0,0.0,0.0)},
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
				Voxel{
					mesh: Some(vec![ //dirt
		    							vec![//top
											Vertex{uv: Vector2::new(0.0, 3.0), v: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 3.0), v: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(0.0, 4.0), v: Vector3::new(0.0,1.0,1.0)},
											
											Vertex{uv: Vector2::new(0.0, 4.0), v: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(1.0, 3.0), v: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 4.0), v: Vector3::new(1.0,1.0,1.0)}
		    							],
		    							
		    							vec![//botton
		    								Vertex{uv: Vector2::new(0.0, 3.0), v: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(1.0, 4.0), v: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(0.0, 4.0), v: Vector3::new(0.0,0.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(1.0, 3.0), v: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(1.0, 4.0), v: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(0.0, 3.0), v: Vector3::new(0.0,0.0,1.0)}
		    							],
		    							
		    							vec![//right
		    								Vertex{uv: Vector2::new(1.0, 4.0), v: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(0.0, 4.0), v: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(1.0, 3.0), v: Vector3::new(1.0,1.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(0.0, 4.0), v: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(0.0, 3.0), v: Vector3::new(1.0,1.0,1.0)},
		    								Vertex{uv: Vector2::new(1.0, 3.0), v: Vector3::new(1.0,1.0,0.0)}
		    							],
		    							
		    							vec![//left
		    								Vertex{uv: Vector2::new(0.0, 3.0), v: Vector3::new(0.0,1.0,0.0)},
		    								Vertex{uv: Vector2::new(1.0, 4.0), v: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(0.0, 4.0), v: Vector3::new(0.0,0.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(0.0, 3.0), v: Vector3::new(0.0,1.0,1.0)},
		    								Vertex{uv: Vector2::new(0.0, 4.0), v: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(1.0, 3.0), v: Vector3::new(0.0,1.0,0.0)}
		    							],
		    							
		    							vec![//back
											Vertex{uv: Vector2::new(0.0, 4.0), v: Vector3::new(0.0,0.0,1.0)},
											Vertex{uv: Vector2::new(0.0, 3.0), v: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(1.0, 4.0), v: Vector3::new(1.0,0.0,1.0)},
											
											Vertex{uv: Vector2::new(1.0, 4.0), v: Vector3::new(1.0,0.0,1.0)},
											Vertex{uv: Vector2::new(0.0, 3.0), v: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(1.0, 3.0), v: Vector3::new(1.0,1.0,1.0)},
		    							],
		    							
		    							vec![//front
											Vertex{uv: Vector2::new(0.0, 4.0), v: Vector3::new(1.0,0.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 3.0), v: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 4.0), v: Vector3::new(0.0,0.0,0.0)},
											
											Vertex{uv: Vector2::new(0.0, 3.0), v: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(1.0, 3.0), v: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(0.0, 4.0), v: Vector3::new(1.0,0.0,0.0)},
		    							]
		    						]),
					solid: false,
					name: "Stone".to_string(),
					data: Vec::new()
				},
			],
			chunks: std::collections::HashMap::<ChunkPos, VoxelChunk>::new(),
			chunk_size: 16u8,
			seed: rand::thread_rng().gen()
		}
	}

	#[export]
	fn _ready(&mut self, owner: &Spatial) {
		godot_print!("Hello, world.");
		for x in 0..10{
			for y in 0..10{
				for z in 0..10{
					self.build_chunk(owner, Vector3::new(x as f32 * (self.chunk_size as f32 - 1f32), y as f32 * (self.chunk_size as f32 - 1f32), z as f32 * (self.chunk_size as f32 - 1f32)))
				}
			}
		}
	}
	
	#[export]
	fn _process(&self, _owner: &Spatial, _delta: f64){
//		godot_print!("Yeet!");
	}
	
	#[export]
	fn set_voxel(&mut self, _owner: &Spatial, pos:Vector3, id: u16){
		let cpos: Vec<i32> = vec![
			(pos.x.floor() / self.chunk_size as f32).floor() as i32,
			(pos.y.floor() / self.chunk_size as f32).floor() as i32,
			(pos.z.floor() / self.chunk_size as f32).floor() as i32
		];
		
		let bpos: Vec<i32> = vec![
			(pos.x.floor() % self.chunk_size as f32) as i32,
			(pos.y.floor() % self.chunk_size as f32) as i32,
			(pos.z.floor() % self.chunk_size as f32) as i32
		];
		
		godot_print!("the chunk pos is Vector3({},{},{})\nthe voxel pos is Vector3({},{},{})",
					cpos[0],cpos[1],cpos[2],  
					bpos[0],bpos[1],bpos[2]);
		
		godot_print!("{}", self.chunks.get_mut(&ChunkPos{x:cpos[0],y:cpos[1],z:cpos[2]}).unwrap().data[bpos[0] as usize][bpos[1] as usize][bpos[2] as usize]);
		self.chunks.get_mut(&ChunkPos{x:cpos[0],y:cpos[1],z:cpos[2]}).unwrap().data[bpos[0] as usize][bpos[1] as usize][bpos[2] as usize] = id;
		godot_print!("{}", self.chunks.get_mut(&ChunkPos{x:cpos[0],y:cpos[1],z:cpos[2]}).unwrap().data[bpos[0] as usize][bpos[1] as usize][bpos[2] as usize]);
	}
	
	fn build_chunk(&mut self, owner: &Spatial, pos: Vector3){
		if !self.chunks.contains_key(&ChunkPos{x: pos.x as i32,y: pos.y as i32,z: pos.z as i32}){
			let mut chunk: VoxelChunk = VoxelChunk::new(self.chunk_size, &self.block_types);
//			let cpos: ChunkPos = &ChunkPos{x: pos.x as i32,y: pos.y as i32,z: pos.z as i32};
			
			let meshinst = MeshInstance::new();
			let noise = OpenSimplexNoise::new();
			noise.set_seed(self.seed);
			
			for x in 0..self.chunk_size{
				for y in 0..self.chunk_size{
					for z in 0..self.chunk_size{
						if noise.get_noise_3d(x as f64 + pos.x as f64, y as f64 + pos.y as f64, z as f64 + pos.z as f64)*50f64+40f64 > y as f64 + pos.y as f64{
							chunk.set_voxel(Vector3::new(x as f32, y as f32, z as f32), 1u16);
							chunk.set_voxel(Vector3::new(x as f32, y as f32 - 1f32, z as f32), 2u16);
							chunk.set_voxel(Vector3::new(x as f32, y as f32 - 4f32, z as f32), 3u16);
						}
//						if noise.get_noise_3d(x as f64 + pos.x as f64, y as f64 + pos.y as f64, z as f64 + pos.z as f64)*30f64+19f64 > y as f64 + pos.y as f64{
//							chunk.data[x as usize][y as usize][z as usize] = 2u16;
//						}
//						if noise.get_noise_3d(x as f64 + pos.x as f64, y as f64 + pos.y as f64, z as f64 + pos.z as f64)*30f64+13f64 > y as f64 + pos.y as f64{
//							chunk.data[x as usize][y as usize][z as usize] = 3u16;
//						}
					}
				}
			}
			
			let st = SurfaceTool::new();

			st.begin(Mesh::PRIMITIVE_TRIANGLES);
	//		st.begin(Mesh::PRIMITIVE_LINES);
		    
		    for x in 0..self.chunk_size{
		    	for y in 0..self.chunk_size{
		    		for z in 0..self.chunk_size{
						chunk.build_voxel_mesh(
							&st,
							Vector3::new(x as f32,y as f32,z as f32),
							chunk.get_voxel(Vector3::new(x as f32,y as f32,z as f32)),
							&vec![8u8,4u8]);
		    		}
		    	}
			}
			
		    st.set_material(ResourceLoader::godot_singleton().load(
		        GodotString::from_str("res://assets/new_spatialmaterial.tres"),
		        GodotString::from_str("Resource"), false).unwrap().cast::<Material>().unwrap());
		    st.add_smooth_group(true);
		    st.generate_normals(false);
		    
//			meshinst.set_mesh(&chunk.build_chunk_mesh(ChunkPos{x: pos.x as i32,y: pos.y as i32,z: pos.z as i32}));
			meshinst.set_mesh(st.commit(gdnative::Null::null(), Mesh::ARRAY_COMPRESS_DEFAULT).expect("couldnt add mesh to node"));
			meshinst.set_translation(Vector3::new(pos.x - 1f32, pos.y - 1f32, pos.z - 1f32,));
			meshinst.set_name(format!("chunk {} {} {}", pos.x as i32, pos.y as i32, pos.z as i32));
			meshinst.create_trimesh_collision();
			owner.add_child(meshinst,true);
			self.chunks.insert(
				ChunkPos{x: pos.x as i32,y: pos.y as i32,z: pos.z as i32}, 
				chunk
			);
		}else{
			let chunk = self.chunks.get_mut(&ChunkPos{x: pos.x as i32,y: pos.y as i32,z: pos.z as i32})
				.expect("somehow, it checked that it has the chunk but it couldnt get the chunk");
			
			let st = SurfaceTool::new();

			st.begin(Mesh::PRIMITIVE_TRIANGLES);
	//		st.begin(Mesh::PRIMITIVE_LINES);
		    
		    for x in 0..self.chunk_size{
		    	for y in 0..self.chunk_size{
		    		for z in 0..self.chunk_size{
						chunk.build_voxel_mesh(
							&st,
							Vector3::new(x as f32,y as f32,z as f32),
							chunk.get_voxel(Vector3::new(x as f32,y as f32,z as f32)),
							&vec![8u8,4u8]);
		    		}
		    	}
			}
			
		    st.set_material(ResourceLoader::godot_singleton().load(
		        GodotString::from_str("res://assets/new_spatialmaterial.tres"),
		        GodotString::from_str("Resource"), false).unwrap().cast::<Material>().unwrap());
		    st.add_smooth_group(true);
		    st.generate_normals(false);
		    
		    unsafe {
				owner.get_node(format!("chunk {} {} {}", pos.x as i32, pos.y as i32, pos.z as i32))
					.unwrap().assume_safe().cast::<MeshInstance>().unwrap().set_mesh(
						st.commit(gdnative::Null::null(), Mesh::ARRAY_COMPRESS_DEFAULT)
							.expect("couldnt add mesh to node")
						);
			};
		}
	}
}

#[derive(Clone)]
pub struct VoxelChunk{
	data: Vec<Vec<Vec<u16>>>,
	block_types: Vec<Voxel>,
	should_remove: bool,
	size: u8
}

impl VoxelChunk{
	fn new(size: u8, btypes: &Vec<Voxel>) -> Self{
		VoxelChunk{
			data: vec![vec![vec![0u16;size as usize];size as usize];size as usize],
			block_types: btypes.to_vec(),
			should_remove: false,
			size: size
		}
	}
	
//	fn build_chunk_mesh(&self,pos: ChunkPos) -> gdnative::Ref<ArrayMesh>{
//	}
	
	fn build_voxel_mesh(&self,st:&Ref<SurfaceTool, Unique>, pos:Vector3, id: u16, size:&Vec<u8>){
		if id == 0u16{ return }
//    	match self.block_types[id as usize].mesh{
//    		None => { return },
//    		Some => {
//    		}
//    	}
    	if self.get_voxel(Vector3::new(pos.x,pos.y + 1.0f32,pos.z)) == 0u16{
			for v in &self.block_types[id as usize].mesh.as_ref().expect("this voxel type doesnt have a mesh for the top side")[0]{//top
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
				st.add_vertex(Vector3::new(v.v.x + pos.x, v.v.y + pos.y, v.v.z + pos.z));
			}
		}
		
		if self.get_voxel(Vector3::new(pos.x,pos.y - 1.0f32,pos.z)) == 0u16{
			for v in &self.block_types[id as usize].mesh.as_ref().expect("this voxel type doesnt have a mesh for the botton side")[1]{//botton
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
				st.add_vertex(Vector3::new(v.v.x + pos.x, v.v.y + pos.y, v.v.z + pos.z));
			}
		}
		
		if self.get_voxel(Vector3::new(pos.x + 1.0f32,pos.y,pos.z)) == 0u16{
			for v in &self.block_types[id as usize].mesh.as_ref().expect("this voxel type doesnt have a mesh for the right side")[2]{//right
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
				st.add_vertex(Vector3::new(v.v.x + pos.x, v.v.y + pos.y, v.v.z + pos.z));
			}
		}
		
		if self.get_voxel(Vector3::new(pos.x - 1.0f32,pos.y,pos.z)) == 0u16{
			for v in &self.block_types[id as usize].mesh.as_ref().expect("this voxel type doesnt have a mesh for the left side")[3]{//left
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
				st.add_vertex(Vector3::new(v.v.x + pos.x, v.v.y + pos.y, v.v.z + pos.z));
			}
		}
		
		if self.get_voxel(Vector3::new(pos.x,pos.y,pos.z + 1.0f32)) == 0u16{
			for v in &self.block_types[id as usize].mesh.as_ref().expect("this voxel type doesnt have a mesh for the back side")[4]{//back
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
				st.add_vertex(Vector3::new(v.v.x + pos.x, v.v.y + pos.y, v.v.z + pos.z));
			}
		}
		
		if self.get_voxel(Vector3::new(pos.x,pos.y,pos.z - 1.0f32)) == 0u16{
			for v in &self.block_types[id as usize].mesh.as_ref().expect("this voxel type doesnt have a mesh for the front side")[5]{//front
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
				st.add_vertex(Vector3::new(v.v.x + pos.x, v.v.y + pos.y, v.v.z + pos.z));
			}
		}
	}
	
	pub fn get_voxel(&self,pos: Vector3) -> u16{
		if pos.x < self.size as f32 && pos.x > 0.0f32 && pos.y < self.size as f32 && pos.y > 0.0f32 && pos.z < self.size as f32 && pos.z > 0.0f32 {
			self.data[pos.x as usize][pos.y as usize][pos.z as usize]
		} else { 0u16 }
	}
	
	pub fn set_voxel(&mut self,pos: Vector3, id:u16){
		if pos.x < self.size as f32 && pos.x > 0.0f32 && pos.y < self.size as f32 && pos.y > 0.0f32 && pos.z < self.size as f32 && pos.z > 0.0f32 {
			self.data[pos.x as usize][pos.y as usize][pos.z as usize] = id
		}
	}
}

fn init(handle: InitHandle) {
	handle.add_tool_class::<VoxelSistem>();
}

godot_init!(init);

