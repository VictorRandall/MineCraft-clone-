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
	fn _ready(&self, _owner: &Spatial) {
		godot_print!("_ready (rust)");
	}
	#[export]
	fn _process(&self, _owner: &Spatial, _delta: f64){
		let st = gdnative::api::SurfaceTool::new();
		st.begin(gdnative::api::Mesh::PRIMITIVE_TRIANGLES)
//		st.add_uv(Vector2(0.0,1.0))
	}
}

fn init(handle: InitHandle) {
	handle.add_tool_class::<VoxelWorld>();
}

godot_init!(init);

//?: como q faz pra pegar um node
//unsafe {
//	owner.get_node("../Label2").unwrap().assume_safe().cast::<Label>().unwrap().set_text(format!("_process (rust)\ndelta = {}",delta.to_string()))
//}
