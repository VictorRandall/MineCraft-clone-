
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
//	Air(false, MeshType::none),
//	Grass(false, MeshType::cube)
//}




#[derive(NativeClass)]
#[inherit(Spatial)]

pub struct VoxelWorld{
	chunks: Vec<VoxelChunk>
}

#[methods]
impl VoxelWorld {
	fn new(_owner: &Spatial) -> Self {
		VoxelWorld{
			chunks: Vec::new()
		}
	}


	#[export]
	fn _ready(&mut self, owner: &Spatial) {
		godot_print!("_ready (rust)");
		self.chunks.push(VoxelChunk::new(Vector3::new(1.0,0.0,0.0),1,1,1));
		self.chunks.push(VoxelChunk::new(Vector3::new(0.0,0.0,0.0),1,1,1));
	}
	#[export]
	fn _process(&mut self, owner: &Spatial, delta: f64){
		let input = Input::godot_singleton();
		let label = unsafe {owner.get_node("Label").unwrap().assume_safe().cast::<Label>().unwrap()};
//		let mut chunk = VoxelChunk::new(Vector3::new(1.0,0.0,0.0),1,1,1);
		
		if input.is_action_just_pressed("test"){
			self.chunks[0].size.0 += 1;
			self.chunks[0].size.1 += 1;
			self.chunks[0].size.2 += 1;
//			self.chunks.push(VoxelChunk::new(Vector3::new(self.chunks.len() as f32 + 0.0 ,0.0,0.0),1,1,1));
		}else if input.is_action_just_pressed("test2"){
			self.chunks[0].size.0 -= 1;
			self.chunks[0].size.1 -= 1;
			self.chunks[0].size.2 -= 1;
		};
		
		label.set_text(
			format!("self.chunks.len({})\nself.chunks.len() as f32 + 0.0 == {}\ndelta = {}",self.chunks.len(), self.chunks.len() as f32 + 0.0, delta.to_string())
		);
		
//		for i in 0..self.chunks.len(){
		unsafe {
			owner.get_node("MeshInstance").unwrap().assume_safe().cast::<MeshInstance>().unwrap().set_mesh(self.chunks[0].make_chunk())
		};
//		}
	}
}

#[derive(Debug)]
pub struct VoxelChunk{
	pos:Vector3,
	size:(u32,u32,u32),
	data: Vec<u8>
//	data_pos:usize
}

//#[methods]
impl VoxelChunk{
	fn new(position:Vector3, s_x:u32, s_y:u32, s_z:u32) -> Self {
		VoxelChunk{
			pos:position,
			size:(s_x,s_y,s_z),
			data: Vec::new(),
//			owner:ow
		}
	}

//	fn set_size(&mut self,s_x:u32,s_y:u32,s_z:u32){
//		self.size_x = s_x;
//		self.size_y = s_y;
//		self.size_z = s_z;
//	}

//	fn start(){
//		
//		
//	}
//
	fn make_chunk(&mut self) -> gdnative::Ref<ArrayMesh>{
		self.data.clear();
		let st = SurfaceTool::new();

		st.begin(Mesh::PRIMITIVE_TRIANGLES);
//		st.begin(Mesh::PRIMITIVE_LINES);
		

		for x in (self.pos.x * self.size.0 as f32) - self.size.0 as f32 .. self.pos.x * self.size.0 as f32{
			for y in (self.pos.y * self.size.1 as f32) - self.size.1 as f32 .. self.size.1{
				for z in 0..self.size.2{
					VoxelChunk::custom_voxel(
						&st, 
						Vector3::new(x as f32 + (self.pos.x * self.size.0 as f32),y as f32  + (self.pos.y * self.size.1 as f32),z as f32 + (self.pos.z * self.size.2 as f32)),
						&mut self.data
					);
//					godot_print!(
//					"\nx as f32 + (self.pos.x * self.size.0 as f32) == {}\ny as f32  + (self.pos.y * self.size.1 as f32) == {}\nz as f32 + (self.pos.z * self.size.2 as f32) == {}"
//					,x as f32 + (self.pos.x * self.size.0 as f32)
//					,y as f32  + (self.pos.y * self.size.1 as f32)
//					,z as f32 + (self.pos.z * self.size.2 as f32)
//					);
				}
			}
		}
		

		st.generate_normals(false);
		let mesh: Ref<ArrayMesh> = st.commit(gdnative::Null::null(), Mesh::ARRAY_COMPRESS_DEFAULT).unwrap();
//		godot_print!("commited mesh in {} times!", 1);
		return mesh;
//		unsafe {
//			self.owner.get_node("MeshInstance").unwrap().assume_safe().cast::<MeshInstance>().unwrap().set_mesh(mesh)
//		};
	}

//	fn end(){
//
//	}

	fn custom_voxel(st:&Ref<SurfaceTool, Unique>, pos:Vector3, data: &mut Vec<u8>){
		
		let offset_x:f32 = pos.x;
		let offset_y:f32 = pos.y;
		let offset_z:f32 = pos.z;

		let offset_uv_x:f32 = 0.0;
		let offset_uv_y:f32 = 0.0;

		

		data.push(1);
		godot_print!("{}",data.len());

		//top
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
			
		//botton
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

	//	left
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

	//	right
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
