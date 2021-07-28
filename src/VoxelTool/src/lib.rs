use gdnative::api::{ArrayMesh, Mesh, MeshInstance, OpenSimplexNoise, SurfaceTool, Spatial, StaticBody};
use gdnative::prelude::*;


pub struct VoxelChunk{
	pos:Vector3,
	size_x:i32,
	size_y:i32,
	size_z:i32,
}

impl VoxelChunk{
	fn new(position:Vector3, size_X:i32, size_Y:i32, size_Z:i32) -> Self {
		VoxelChunk{
			pos:position,
			size_x:size_X,
			size_y:size_Y,
			size_z:size_Z,
		}
	}
	
}

#[derive(NativeClass)]
#[inherit(Spatial)]


pub struct VoxelWorld{
	size_x:i32,
	size_y:i32,
	size_z:i32,
} 

#[methods]
impl VoxelWorld {
	fn new(_owner: &Spatial) -> Self {
		VoxelWorld{
			size_x: 1,
			size_y: 1,
			size_z: 1,
		}
	}


	#[export]
	fn _ready(&self, _owner: &Spatial) {
		godot_print!("_ready (rust)");
	}
	#[export]
	fn _process(&mut self, owner: &Spatial, _delta: f64){
		let input = Input::godot_singleton();
		let st = SurfaceTool::new();

//		st.begin(Mesh::PRIMITIVE_TRIANGLES);
		st.begin(Mesh::PRIMITIVE_LINES);
		
		
		if input.is_action_just_pressed("test"){
			self.size_x += 1;
			self.size_y += 1;
			self.size_z += 1;
		}else if input.is_action_just_pressed("test2"){
			self.size_x -= 1;
			self.size_y -= 1;
			self.size_z -= 1;
		}
		

		for x in self.size_x - (self.size_x * 2)..self.size_x / 2{
			for y in self.size_y - (self.size_y * 2)..self.size_y / 2{
				for z in self.size_z - (self.size_z * 2)..self.size_z / 2{
					cube(&st, Vector3::new(x as f32,y as f32,z as f32));
				}
			}
		}

		

		st.generate_normals(false);
		let mesh: Ref<ArrayMesh> = st.commit(gdnative::Null::null(), Mesh::ARRAY_COMPRESS_DEFAULT).unwrap();
		godot_print!("commited mesh!");
		unsafe {
			owner.get_node("MeshInstance").unwrap().assume_safe().cast::<MeshInstance>().unwrap().set_mesh(mesh)
		};
	}

}

fn init(handle: InitHandle) {
	handle.add_tool_class::<VoxelWorld>();
}

fn cube(st:&Ref<SurfaceTool, Unique>, pos:Vector3){
	
	let modi_x:f32 = pos.x;
	let modi_y:f32 = pos.y;
	let modi_z:f32 = pos.z;

	let modi_uv_x:f32 = 0.0;
	let modi_uv_y:f32 = 0.0;



	//top
	st.add_uv(Vector2::new(0.0, 0.0));
	st.add_vertex(Vector3::new(0.0+modi_x,1.0+modi_y,0.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(1.0+modi_x,1.0+modi_y,0.0+modi_z));
	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(0.0+modi_x,1.0+modi_y,1.0+modi_z));

	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(0.0+modi_x,1.0+modi_y,1.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(1.0+modi_x,1.0+modi_y,0.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.25));
	st.add_vertex(Vector3::new(1.0+modi_x,1.0+modi_y,1.0+modi_z));
		
	//botton
	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(0.0+modi_x,0.0+modi_y,1.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(1.0+modi_x,0.0+modi_y,0.0+modi_z));
	st.add_uv(Vector2::new(0.0, 0.0));
	st.add_vertex(Vector3::new(0.0+modi_x,0.0+modi_y,0.0+modi_z));

	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(1.0+modi_x,0.0+modi_y,1.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(1.0+modi_x,0.0+modi_y,0.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.25));
	st.add_vertex(Vector3::new(0.0+modi_x,0.0+modi_y,1.0+modi_z));

//	left
	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(1.0+modi_x,0.0+modi_y,0.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(1.0+modi_x,0.0+modi_y,1.0+modi_z));
	st.add_uv(Vector2::new(0.0, 0.0));
	st.add_vertex(Vector3::new(1.0+modi_x,1.0+modi_y,0.0+modi_z));

	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(1.0+modi_x,0.0+modi_y,1.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(1.0+modi_x,1.0+modi_y,1.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.25));
	st.add_vertex(Vector3::new(1.0+modi_x,1.0+modi_y,0.0+modi_z));

//	right
	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(0.0+modi_x,1.0+modi_y,0.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(0.0+modi_x,0.0+modi_y,1.0+modi_z));
	st.add_uv(Vector2::new(0.0, 0.0));
	st.add_vertex(Vector3::new(0.0+modi_x,0.0+modi_y,0.0+modi_z));

	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(0.0+modi_x,1.0+modi_y,1.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(0.0+modi_x,0.0+modi_y,1.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.25));
	st.add_vertex(Vector3::new(0.0+modi_x,1.0+modi_y,0.0+modi_z));
	
//	front
	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(0.0+modi_x,0.0+modi_y,1.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(0.0+modi_x,1.0+modi_y,1.0+modi_z));
	st.add_uv(Vector2::new(0.0, 0.0));
	st.add_vertex(Vector3::new(1.0+modi_x,0.0+modi_y,1.0+modi_z));

	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(1.0+modi_x,0.0+modi_y,1.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(0.0+modi_x,1.0+modi_y,1.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.25));
	st.add_vertex(Vector3::new(1.0+modi_x,1.0+modi_y,1.0+modi_z));

//	back
	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(1.0+modi_x,0.0+modi_y,0.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(0.0+modi_x,1.0+modi_y,0.0+modi_z));
	st.add_uv(Vector2::new(0.0, 0.0));
	st.add_vertex(Vector3::new(0.0+modi_x,0.0+modi_y,0.0+modi_z));

	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(1.0+modi_x,1.0+modi_y,0.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(0.0+modi_x,1.0+modi_y,0.0+modi_z));
	st.add_uv(Vector2::new(0.25, 0.25));
	st.add_vertex(Vector3::new(1.0+modi_x,0.0+modi_y,0.0+modi_z));
}

godot_init!(init);
