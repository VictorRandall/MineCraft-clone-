use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;

#[methods]
impl HelloWorld {
//	let test:str = "rust"
	    
	fn new(_owner: &Node) -> Self {
		HelloWorld
	}

    #[export]
	fn _ready(&self, _owner: &Node) {
		godot_print!("_ready");
	}
	#[export]
	fn _process(&self, _owner: &Node, delta: f64){
		godot_print!("_process");
	}
//	#[export]
//	fn penis(){
//		godot_print!("penis")
//	}
}

fn init(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
}

godot_init!(init);
