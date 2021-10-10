use std;
use gdnative::prelude::*;
use gdnative::api::{ArrayMesh, Mesh, MeshInstance, OpenSimplexNoise, SurfaceTool, Spatial, StaticBody, Material};

struct ChunkPos{
	x:i32,
	y:i32,
	z:i32
}

struct Vertex{
	uv: Vector2,
	vertex: Vector3
}

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct HelloWorld{
//	block_types: Vec<Vec<Vec<Vertex>>>,
	chunks_data: Vec<Vec<Vec<u16>>>,
	chunk_size: u8
}

#[methods]
impl HelloWorld {
    fn new(_owner: &Spatial) -> Self {
        HelloWorld{
        	chunks_data: vec![vec![vec![1u16;16usize];16usize];16usize],
        	chunk_size: 16u8
        }
    }

    #[export]
    fn _ready(&mut self, owner: &Spatial) {
		self.create_chunk(owner);
    }
    
    fn create_chunk(&mut self,owner:&Spatial){
    	let noise = OpenSimplexNoise::new();
        
        for x in 0..self.chunk_size{
        	for y in 0..self.chunk_size{
        		for z in 0..self.chunk_size{
        			if noise.get_noise_2d(x as f64, z as f64) > y as f64{
        				self.chunks_data[x as usize][y as usize][z as usize] = 0u16;
//        				self.chunks_data[x as usize][(y - 1u8) as usize][z as usize] = 2u16;
        			}
        		}
        	}
		}
		godot_print!("{:?}",self.chunks_data);
        let meshinst = MeshInstance::new();
        
        meshinst.set_mesh(self.create_chunk_mesh());
        owner.add_child(meshinst,true);
    }
    
    fn create_chunk_mesh(&self) -> gdnative::Ref<ArrayMesh>{
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
    	
    	let dirt:Vec<Vec<Vertex>> = vec![ //dirt
		    							vec![//top
											Vertex{uv: Vector2::new(2.0, 0.0),vertex: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(3.0, 0.0),vertex: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(2.0, 1.0),vertex: Vector3::new(0.0,1.0,1.0)},
											
											Vertex{uv: Vector2::new(2.0, 1.0),vertex: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(3.0, 0.0),vertex: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(3.0, 1.0),vertex: Vector3::new(1.0,1.0,1.0)}
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
		    								Vertex{uv: Vector2::new(3.0, 1.0), vertex: Vector3::new(1.0,0.0,0.0)},
		    								Vertex{uv: Vector2::new(2.0, 1.0), vertex: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(3.0, 0.0), vertex: Vector3::new(1.0,1.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(2.0, 1.0), vertex: Vector3::new(1.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(2.0, 0.0), vertex: Vector3::new(1.0,1.0,1.0)},
		    								Vertex{uv: Vector2::new(3.0, 0.0), vertex: Vector3::new(1.0,1.0,0.0)}
		    							],
		    							
		    							vec![//left
		    								Vertex{uv: Vector2::new(2.0, 0.0), vertex: Vector3::new(0.0,1.0,0.0)},
		    								Vertex{uv: Vector2::new(3.0, 1.0), vertex: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(2.0, 1.0), vertex: Vector3::new(0.0,0.0,0.0)},
		    								
		    								Vertex{uv: Vector2::new(2.0, 0.0), vertex: Vector3::new(0.0,1.0,1.0)},
		    								Vertex{uv: Vector2::new(2.0, 1.0), vertex: Vector3::new(0.0,0.0,1.0)},
		    								Vertex{uv: Vector2::new(3.0, 0.0), vertex: Vector3::new(0.0,1.0,0.0)}
		    							],
		    							
		    							vec![//back
											Vertex{uv: Vector2::new(2.0,1.0), vertex: Vector3::new(0.0,0.0,1.0)},
											Vertex{uv: Vector2::new(2.0,0.0), vertex: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(3.0,1.0), vertex: Vector3::new(1.0,0.0,1.0)},
											
											Vertex{uv: Vector2::new(3.0,1.0), vertex: Vector3::new(1.0,0.0,1.0)},
											Vertex{uv: Vector2::new(2.0,0.0), vertex: Vector3::new(0.0,1.0,1.0)},
											Vertex{uv: Vector2::new(3.0,0.0), vertex: Vector3::new(1.0,1.0,1.0)},
		    							],
		    							
		    							vec![//front
											Vertex{uv: Vector2::new(2.0,1.0), vertex: Vector3::new(1.0,0.0,0.0)},
											Vertex{uv: Vector2::new(3.0,0.0), vertex: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(3.0,1.0), vertex: Vector3::new(0.0,0.0,0.0)},
											
											Vertex{uv: Vector2::new(2.0,0.0), vertex: Vector3::new(1.0,1.0,0.0)},
											Vertex{uv: Vector2::new(3.0,0.0), vertex: Vector3::new(0.0,1.0,0.0)},
											Vertex{uv: Vector2::new(2.0,1.0), vertex: Vector3::new(1.0,0.0,0.0)},
		    							]
		    						];
    
    	let st = SurfaceTool::new();
    
    	let size:Vec<u8> = vec![4u8,4u8];
        
		st.begin(Mesh::PRIMITIVE_TRIANGLES);
//		st.begin(Mesh::PRIMITIVE_LINES);
        
        for x in 0..self.chunk_size{
        	for y in 0..self.chunk_size{
        		for z in 0..self.chunk_size{
//        			godot_print!("{}",self.get_voxel(Vector3::new(x as f32,y as f32,z as f32)));
//        			if self.get_voxel(Vector3::new(x as f32,y as f32,z as f32)) == 0u16{ return }
//        			if self.get_voxel(Vector3::new(x as f32,y as f32,z as f32)) == 1u16{
        				self.create_custom_voxel(&st,Vector3::new(x as f32,y as f32,z as f32),&grass,&size);
//        			}else{
//        				self.create_custom_voxel(&st,Vector3::new(x as f32 + pos.x,y as f32 + pos.y,z as f32 + pos.z),&dirt,&size);
//        			}
        		}
        	}
		}
		
        st.set_material(ResourceLoader::godot_singleton().load(
            GodotString::from_str("res://assets/new_spatialmaterial.tres"),
            GodotString::from_str("Resource"), false).unwrap().cast::<Material>().unwrap());
        st.generate_normals(false);
        
        return st.commit(gdnative::Null::null(), Mesh::ARRAY_COMPRESS_DEFAULT).unwrap()
    }
    
    fn create_custom_voxel(&self,st:&Ref<SurfaceTool, Unique>, pos:Vector3, mesh:&Vec<Vec<Vertex>>, size:&Vec<u8>){
    	if self.get_voxel(Vector3::new(pos.x,pos.y,pos.z)) == 0u16{ return }
    
		if self.get_voxel(Vector3::new(pos.x,pos.y + 1.0f32,pos.z)) == 0u16{
		    for v in &mesh[0]{//top
		    	st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
		    	st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
		    }
        }
        
        if self.get_voxel(Vector3::new(pos.x,pos.y - 1.0f32,pos.z)) == 0u16{
			for v in &mesh[1]{//botton
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
				st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
			}
		}
		
		if self.get_voxel(Vector3::new(pos.x + 1.0f32,pos.y,pos.z)) == 0u16{
			for v in &mesh[2]{//right
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
				st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
			}
		}
		
		if self.get_voxel(Vector3::new(pos.x - 1.0f32,pos.y,pos.z)) == 0u16{
			for v in &mesh[3]{//left
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
				st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
			}
		}
		
		if self.get_voxel(Vector3::new(pos.x,pos.y,pos.z + 1.0f32)) == 0u16{
			for v in &mesh[4]{//back
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
				st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
			}
		}
		
		if self.get_voxel(Vector3::new(pos.x,pos.y,pos.z - 1.0f32)) == 0u16{
			for v in &mesh[5]{//front
				st.add_uv(Vector2::new(v.uv.x / size[0] as f32,v.uv.y / size[0] as f32));
				st.add_vertex(Vector3::new(v.vertex.x + pos.x, v.vertex.y + pos.y, v.vertex.z + pos.z));
			}
		}
	}
    fn get_voxel(&self,pos:Vector3) -> u16 {
		if pos.x < self.chunk_size as f32 && pos.x > 0.0f32 && pos.y < self.chunk_size as f32 && pos.y > 0.0f32 && pos.z < self.chunk_size as f32 && pos.z > 0.0f32 {
			self.chunks_data[pos.x as usize][pos.y as usize][pos.z as usize]
		} else { 0u16 }
	}
	
	#[export]
	fn set_voxel(&mut self, owner: &Spatial, pos:Vector3){//pos.floor() / self.chunk_size is the chunk    pos.floor() % self.chunk_size is the block position
		let cpos_x: f32 = (pos.x / self.chunk_size as f32).floor();
		let cpos_y: f32 = (pos.y / self.chunk_size as f32).floor();
		let cpos_z: f32 = (pos.z / self.chunk_size as f32).floor();
		
		let bpos_x: f32 = pos.x.floor() % self.chunk_size as f32;
		let bpos_y: f32 = pos.y.floor() % self.chunk_size as f32;
		let bpos_z: f32 = pos.z.floor() % self.chunk_size as f32;
		
		godot_print!("the chunk position is {}, {}, {}\nthe block position is {}, {}, {}", cpos_x,cpos_y,cpos_z,bpos_x,bpos_y,bpos_z);
	}
}

fn init(handle: InitHandle) {
    handle.add_tool_class::<HelloWorld>();
}

godot_init!(init);
	
