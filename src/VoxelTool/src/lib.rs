//extern crate ndarray;

use std;
//use ndarray::Array3;
use rand::Rng;
use gdnative::api::{ArrayMesh, Mesh, MeshInstance, OpenSimplexNoise, SurfaceTool, Spatial, StaticBody};
use gdnative::prelude::*;
//enum MeshType{
//	none,
//	smooth,
//	cube,
//	//custom:{}
//}
//struct BlockType{
//	solid:bool,
//	mshtype: MeshType,
//
//}
//enum blocks{
//	Air(false, MeshType::none)
//	Grass(false, MeshType::cube)
//}
#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct VoxelWorld{
	chunks: Vec<VoxelChunk>,
	seed:u64,
	chunk_size: u32,
}

#[methods]
impl VoxelWorld {
	fn new(_owner: &Spatial) -> Self {
		VoxelWorld{
			chunks: Vec::<VoxelChunk>::new(),
			seed: rand::thread_rng().gen(),
			chunk_size: 16,
		}
	}


	#[export]
	fn _ready(&mut self, _owner: &Spatial) {
		godot_print!("_ready (rust)");
//		let a = Array3::new();
		for x in 0..2{
			for y in 0..2{
				for z in 0..2{
					self.chunks.push(VoxelChunk::new(Vector3::new(x as f32 * 2f32,y as f32 * 2f32,z as f32 * 2f32),8,8,8));
				}
			}
		}
	}
	#[export]
	fn _process(&mut self, owner: &Spatial, _delta: f64){
		let input = Input::godot_singleton();
//		let label = unsafe {owner.get_node("Label").unwrap().assume_safe().cast::<Label>().unwrap()};
//		let mut chunk = VoxelChunk::new(Vector3::new(1.0,0.0,0.0),1,1,1);
		
		if input.is_action_pressed("test"){
			for i in 0..self.chunks.len(){
				self.chunks[i].size += 1;
				self.chunks[i].update = true;
			}
//			self.chunks.push(VoxelChunk::new(Vector3::new(self.chunks.len() as f32 + 0.0 ,0.0,0.0),1,1,1));
		}else if input.is_action_pressed("test2"){
			for i in 0..self.chunks.len(){
				self.chunks[i].size.0 -= 1;
				self.chunks[i].update = true;
			}
		};
		
//		label.set_text(
//			format!("self.chunks.len({})\nself.chunks.len() as f32 + 0.0 == {}\ndelta = {}",self.chunks.len(), self.chunks.len() as f32 + 0.0, delta.to_string())
//		);
		

		for i in 0..self.chunks.len(){
			self.chunks[i].start(&owner);
//			self.chunks[i].restart(&owner);
//			self.mainData.push(self.chunks[i].data);
		}
//		unsafe {
//			owner.get_node("MeshInstance").unwrap().assume_safe().cast::<MeshInstance>().unwrap().set_mesh(self.chunks[0].chunk_mesh())
//		};
//		unsafe {
//			owner.get_node("MeshInstance2").unwrap().assume_safe().cast::<MeshInstance>().unwrap().set_mesh(self.chunks[1].chunk_mesh())
//		};
	}

	fn get_voxel(x:u32,y:u32,z:u32){
		for i in 0..16{for j in 0..16}
	}
}

#[derive(Debug)]
pub struct VoxelChunk{
	pos:Vector3,
	size:u32,
	data: Vec<Vec<Vec<u16>>>,
	update:bool,
}

impl VoxelChunk{
	fn new(position:Vector3, s_x:u32, s_y:u32, s_z:u32) -> Self {
		VoxelChunk{
			pos:position,
			size:(s_x,s_y,s_z),
			data: vec![vec![vec![1u16; s_x as usize]; s_y as usize]; s_z as usize],//the default value is 4
			update:true,
		}
	}

//	fn set_size(&mut self,s_x:u32,s_y:u32,s_z:u32){
//		self.size_x = s_x;
//		self.size_y = s_y;
//		self.size_z = s_z;
//	}




	fn start(&mut self, owner: &Spatial){
		if self.update == true{
//			std::thread::spawn(||{
				let noise = OpenSimplexNoise::new();
				let meshinst = MeshInstance::new();
				for x in 0..self.size{
					for y in 0..self.size{
						for z in 0..self.size{
//							if y as f64 > 10.0f64{ //noise.get_noise_2d(x as f64, z as f64)*5f64+10f64{
								self.data[x as usize][y as usize][z as usize] = 1u16;
//							}//else{
//								self.data[x as usize ][y as usize][z as usize] = 0u16;
//							}
						}
					}
				}
				meshinst.set_mesh(self.chunk_mesh());
				meshinst.set_name(format!("chunk{}{}{}",self.pos.x,self.pos.y,self.pos.z));
				owner.add_child(meshinst,true);
				self.update = false;
//			});
		}
	}

	fn chunk_mesh(&mut self) -> gdnative::Ref<ArrayMesh>{
		let st = SurfaceTool::new();

		st.begin(Mesh::PRIMITIVE_TRIANGLES);
//		st.begin(Mesh::PRIMITIVE_LINES);
		

		for x in 0..self.size{
			for y in 0..self.size{
				for z in 0..self.size{
					VoxelChunk::custom_voxel(
						&st, 
						Vector3::new(x as f32 + (self.pos.x * self.size as f32),y as f32  + (self.pos.y * self.size as f32),z as f32 + (self.pos.z * self.size as f32)),
						&self.data
					);
//					godot_print!("{},{},{}",x,y,z)
				}
			}
		}
		

		st.generate_normals(false);
		let mesh: Ref<ArrayMesh> = st.commit(gdnative::Null::null(), Mesh::ARRAY_COMPRESS_DEFAULT).unwrap();
		return mesh;
	}

//	fn restart(&mut self, owner: &Spatial){
//		self.data.clear();
//		godot_print!("chunk{}{}{}",self.pos.x,self.pos.y,self.pos.z);
//		unsafe {
//			owner.get_node(format!("chunk{}{}{}",self.pos.x,self.pos.y,self.pos.z)).unwrap().assume_safe().cast::<MeshInstance>().unwrap().queue_free();
//		};
//	}

	fn custom_voxel(st:&Ref<SurfaceTool, Unique>, pos:Vector3, data: &Vec<Vec<Vec<u16>>>){
		
		let offset_x:f32 = pos.x;
		let offset_y:f32 = pos.y;
		let offset_z:f32 = pos.z;

		let offset_uv_x:f32 = 0.0;
		let offset_uv_y:f32 = 0.0;

//		if data[offset_x as usize][offset_y as usize][offset_z as usize] == 0{
//			return
//		}
//
//		
		godot_print!("{:#?}",data);

		//top
//		godot_print!("pos = Vector3({},{},{})",offset_x as usize,offset_y as usize,offset_z as usize)
//		if data[offset_x as usize][(offset_y + 1.0f32) as usize][offset_z as usize] != 0u16{
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
//		}
			
		//botton
//		if data[pos.x as usize][(pos.y - 1.0f32) as usize][pos.z as usize] != 0u16{
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
//		}

	//	left
//		if data[(pos.x - 1.0f32) as usize][pos.y as usize][pos.z as usize] != 0u16{
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
//		}

	//	right
//		if data[(pos.x + 1.0f32) as usize][pos.y as usize][pos.z as usize] != 0u16{
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
//		}
		
	//	front
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

	//	back
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
	}
}

fn init(handle: InitHandle) {
	handle.add_tool_class::<VoxelWorld>();
}

godot_init!(init);
