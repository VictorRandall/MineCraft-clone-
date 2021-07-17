use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct HelloWorld;

#[methods]
impl HelloWorld {
//	let test:str = "rust"
	    
	fn new(_owner: &Spatial) -> Self {
		HelloWorld
	}

	#[export]
	fn _ready(&self, _owner: &Spatial) {
		godot_print!("_ready (rust)");
	}
	#[export]
	fn _process(&self, owner: &Spatial, delta: f64){
		godot_print!("_process (rust)\ndelta = {}", delta);
		owner.get_node("Node/Label2").unwrap().cast::<Ref<Label>>().set_text("_process (rust)\ndelta = {}")
//		println!("lul")
	}
}

fn init(handle: InitHandle) {
	handle.add_tool_class::<HelloWorld>();
}

godot_init!(init);
