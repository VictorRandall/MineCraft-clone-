use gdnative::prelude::*;
use vector2d::Vector2D;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct HelloWorld;

#[methods]
impl HelloWorld {
	fn new(_owner: &Spatial) -> Self {
		HelloWorld
	}

	#[export]
	fn _ready(&self, _owner: &Spatial) {
		godot_print!("_ready (rust)");
	}
	#[export]
	fn _process(&self, owner: &Spatial, delta: f64){
		let mut st = gdnative::api::SurfaceTool::new();
		st.begin(gdnative::api::Mesh::PRIMITIVE_TRIANGLES)
		st.add_uv
	}
}

fn init(handle: InitHandle) {
	handle.add_tool_class::<HelloWorld>();
}

godot_init!(init);

//?: como q faz pra pegar um node
//unsafe {
//	owner.get_node("../Label2").unwrap().assume_safe().cast::<Label>().unwrap().set_text(format!("_process (rust)\ndelta = {}",delta.to_string()))
//}
