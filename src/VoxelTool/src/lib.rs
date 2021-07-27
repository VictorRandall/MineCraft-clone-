use gdnative::api::{ArrayMesh, Mesh, MeshInstance, OpenSimplexNoise, SurfaceTool, Spatial, StaticBody};
use gdnative::prelude::*;

//#[derive(NativeClass)]
//#[inherit(StaticBody)]
//#[register_with(Self::register_properties)]
//#[inherit(StaticBody)]
//mod voxelchunk;
//
//pub struct VoxelChunk {
//	position:f32,
//	data:f32,
//}
//
//#[methods]
//impl VoxelChunk {
//	fn new(_owner: &StaticBody) -> Self {
//		VoxelChunk{
//			position: 1.0,
//			data: 1.0,
//		}
//	}
//} 

#[derive(NativeClass)]
#[inherit(Spatial)]
#[register_with(Self::register_properties)]

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
//		let mut array = TypedArray::new();
//		st.begin(Mesh::PRIMITIVE_TRIANGLES);
		st.begin(Mesh::PRIMITIVE_LINES);
//		st.add_smooth_group(true);
//		
		if input.is_action_just_pressed("test"){
			self.size_x += 1;
			self.size_y += 1;
			self.size_z += 1;
		}else if input.is_action_just_pressed("test2"){
			self.size_x -= 1;
			self.size_y -= 1;
			self.size_z -= 1;
		}
		

		//this block was commented to experiment add_triangle_fan in line 208
		for x in 0..self.size_x{
			for y in 0..self.size_y{
				for z in 0..self.size_z{
					cube(&st, Vector3::new(x as f32,y as f32,z as f32));
					godot_print!("created block number Vector3({},{},{})!", x,y,z)
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
	let modi_uv_x:f32 = 0.0;

//	data.insert(0,1);
//	godot_print!("{}",data);


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

//?: como q faz pra pegar um node
//unsafe {
//	owner.get_node(NodePath).unwrap().assume_safe().cast::<NodeType>().unwrap()
//}
//?: como juntar/transformar duas strings/int/float/etc em uma so string
//format!()
//try using add_triangle_fan
//	st.add_triangle_fan(
//		TypedArray::<Vector3>(Vector3::new(0.0,1.0,0.0),Vector3::new(0.0,1.0,0.0),Vector3::new(0.0,1.0,1.0)),
//		TypedArray::<Vector2>(Vector2::new(0.0,0.0),Vector2::new(1.0,0.0),Vector2::new(1.0,1.0)),
//		gdnative::Null::null(),
//		gdnative::Null::null(),
//		gdnative::Null::null(),
//		gdnative::Null::null()
//	);








