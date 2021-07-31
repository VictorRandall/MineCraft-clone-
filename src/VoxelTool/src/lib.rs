use gdnative::api::{ArrayMesh, Mesh, MeshInstance, OpenSimplexNoise, SurfaceTool, Spatial, StaticBody};
use gdnative::prelude::*;


#[derive(NativeClass)]
#[inherit(Spatial)]


pub struct VoxelWorld;

#[methods]
impl VoxelWorld {
	fn new(_owner: &Spatial) -> Self {
		VoxelWorld
	}


	#[export]
	fn _ready(&self, owner: &Spatial) {
		godot_print!("_ready (rust)");
	}
	#[export]
	fn _process(&mut self, owner: &Spatial, _delta: f64){
		let input = Input::godot_singleton();
		let mut chunk = VoxelChunk::new(Vector3::new(0.0,0.0,0.0),1,1,1);
		
		if input.is_action_just_pressed("test"){
			chunk.size_x += 1;
			chunk.size_y += 1;
			chunk.size_z += 1;
			unsafe {
			owner.get_node("MeshInstance").unwrap().assume_safe().cast::<MeshInstance>().unwrap().set_mesh(chunk.chunk())
			};
		}else if input.is_action_just_pressed("test2"){
			chunk.size_x -= 1;
			chunk.size_y -= 1;
			chunk.size_z -= 1;
			unsafe {
			owner.get_node("MeshInstance").unwrap().assume_safe().cast::<MeshInstance>().unwrap().set_mesh(chunk.chunk())
			};
		};
		
		
	}

}


pub struct VoxelChunk{
	pos:Vector3,
	size_x:u32,
	size_y:u32,
	size_z:u32,
//	owner:& <'a> Spatial
}

//#[methods]
impl VoxelChunk{
	fn new(position:Vector3, s_x:u32, s_y:u32, s_z:u32) -> Self {
		VoxelChunk{
			pos:position,
			size_x:s_x,
			size_y:s_y,
			size_z:s_z,
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
	fn chunk(&mut self) -> gdnative::Ref<ArrayMesh>{
		let st = SurfaceTool::new();

//		st.begin(Mesh::PRIMITIVE_TRIANGLES);
		st.begin(Mesh::PRIMITIVE_LINES);
		

		for x in 0..self.size_x{
			for y in 0..self.size_y{
				for z in 0..self.size_z{
					VoxelChunk::custom_voxel(&st, Vector3::new(x as f32,y as f32,z as f32));
				}
			}
		}
		

		st.generate_normals(false);
		let mesh: Ref<ArrayMesh> = st.commit(gdnative::Null::null(), Mesh::ARRAY_COMPRESS_DEFAULT).unwrap();
		godot_print!("commited mesh!");
		return mesh;
//		unsafe {
//			self.owner.get_node("MeshInstance").unwrap().assume_safe().cast::<MeshInstance>().unwrap().set_mesh(mesh)
//		};
	}

//	fn end(){
//
//	}

	fn custom_voxel(st:&Ref<SurfaceTool, Unique>, pos:Vector3){
		
		let offset_x:f32 = pos.x;
		let offset_y:f32 = pos.y;
		let offset_z:f32 = pos.z;

		let offset_uv_x:f32 = 0.0;
		let offset_uv_y:f32 = 0.0;



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
