use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Spatial)]

//mod voxelchunk;

pub struct VoxelWorld;

#[methods]
impl VoxelWorld {
	fn new(_owner: &Spatial) -> Self {
		VoxelWorld
	}

	#[export]
	fn _ready(&self, _owner: &Spatial) {
		godot_print!("_ready (rust)");
	}
	#[export]
	fn _process(&self, owner: &Spatial, _delta: f64){
//		return
		let st = gdnative::api::SurfaceTool::new();
		st.begin(gdnative::api::Mesh::PRIMITIVE_TRIANGLES);
		st.add_uv(Vector2::new(0.0, 0.0));
		st.add_vertex(Vector3::new(0.0,1.0,0.0));
		st.add_uv(Vector2::new(0.25, 0.0));
		st.add_vertex(Vector3::new(1.0,1.0,0.0));
		st.add_uv(Vector2::new(0.0, 0.25));
		st.add_vertex(Vector3::new(0.0,1.0,1.0));

		st.add_uv(Vector2::new(0.0, 0.0));
		st.add_vertex(Vector3::new(0.0,0.0,0.0));
		st.add_uv(Vector2::new(0.0, 0.0));
		st.add_vertex(Vector3::new(0.0,0.0,0.0));
		st.add_uv(Vector2::new(0.0, 0.0));
		st.add_vertex(Vector3::new(0.0,0.0,0.0));
		
		st.generate_normals(false);
		owner.get_node("MeshInstance").unwrap().assume_safe().cast::<MeshInstance>().unwrap().mesh = st.commit(_,_);
	}
}

fn init(handle: InitHandle) {
	handle.add_tool_class::<VoxelWorld>();
}

godot_init!(init);

//?: como q faz pra pegar um node
//unsafe {
//	owner.get_node(NodePath).unwrap().assume_safe().cast::<NodeType>().unwrap()
//}
//?: como juntar/transformar duas strings/int/float/etc em uma string so
//format!()
