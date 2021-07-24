use gdnative::api::{ArrayMesh, Mesh, MeshInstance, OpenSimplexNoise, SurfaceTool, Spatial};
use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
//mod voxelchunk;

//pub struct VoxelChunk {
//	position:[f32, 3],
//	data:[u16,4096]
//}
pub struct VoxelWorld;
//
//
//impl VoxelChunk {
//	fn new(_owner: &StaticBody) -> Self {
//		VoxelChunk
//	}
//}

#[methods]
impl VoxelWorld {
	fn new(_owner: &Node) -> Self {
		VoxelWorld
	}


	#[export]
	fn _ready(&self, _owner: &Node) {
		godot_print!("_ready (rust)");
	}
	#[export]
	fn _process(&self, owner: &Node, _delta: f64){
//		return
		let st = SurfaceTool::new();
//		let arraymesh = gdnative::api::ArrayMesh::new();
		st.begin(Mesh::PRIMITIVE_TRIANGLES);
//		st.

		cube(st);

//		end();
		st.generate_normals(false);
		let mesh: Ref<ArrayMesh> = st.commit(gdnative::Null::null(), Mesh::ARRAY_COMPRESS_DEFAULT).unwrap();
		unsafe {owner.get_node("MeshInstance").unwrap().assume_safe().cast::<MeshInstance>().unwrap().set_mesh(mesh)};
	}

}

fn init(handle: InitHandle) {
	handle.add_tool_class::<VoxelWorld>();
}

fn cube(st:gdnative::Ref<SurfaceTool, gdnative::prelude::Unique>){
	//top
	st.add_uv(Vector2::new(0.0, 0.0));
	st.add_vertex(Vector3::new(0.0,1.0,0.0));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(1.0,1.0,0.0));
	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(0.0,1.0,1.0));

	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(0.0,1.0,1.0));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(1.0,1.0,0.0));
	st.add_uv(Vector2::new(0.25, 0.25));
	st.add_vertex(Vector3::new(1.0,1.0,1.0));
		
	//botton
	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(0.0,0.0,1.0));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(1.0,0.0,0.0));
	st.add_uv(Vector2::new(0.0, 0.0));
	st.add_vertex(Vector3::new(0.0,0.0,0.0));

	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(1.0,0.0,1.0));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(1.0,0.0,0.0));
	st.add_uv(Vector2::new(0.25, 0.25));
	st.add_vertex(Vector3::new(0.0,0.0,1.0));

//	left
	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(1.0,0.0,0.0));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(1.0,0.0,1.0));
	st.add_uv(Vector2::new(0.0, 0.0));
	st.add_vertex(Vector3::new(1.0,1.0,0.0));

	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(1.0,0.0,1.0));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(1.0,1.0,1.0));
	st.add_uv(Vector2::new(0.25, 0.25));
	st.add_vertex(Vector3::new(1.0,1.0,0.0));

//	right
	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(0.0,1.0,0.0));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(0.0,0.0,1.0));
	st.add_uv(Vector2::new(0.0, 0.0));
	st.add_vertex(Vector3::new(0.0,0.0,0.0));

	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(0.0,1.0,1.0));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(0.0,0.0,1.0));
	st.add_uv(Vector2::new(0.25, 0.25));
	st.add_vertex(Vector3::new(0.0,1.0,0.0));
	
//	front
	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(0.0,0.0,1.0));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(0.0,1.0,1.0));
	st.add_uv(Vector2::new(0.0, 0.0));
	st.add_vertex(Vector3::new(1.0,0.0,1.0));

	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(1.0,0.0,1.0));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(0.0,1.0,1.0));
	st.add_uv(Vector2::new(0.25, 0.25));
	st.add_vertex(Vector3::new(1.0,1.0,1.0));

//	back
	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(1.0,0.0,0.0));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(0.0,1.0,0.0));
	st.add_uv(Vector2::new(0.0, 0.0));
	st.add_vertex(Vector3::new(0.0,0.0,0.0));

	st.add_uv(Vector2::new(0.0, 0.25));
	st.add_vertex(Vector3::new(1.0,1.0,0.0));
	st.add_uv(Vector2::new(0.25, 0.0));
	st.add_vertex(Vector3::new(0.0,1.0,0.0));
	st.add_uv(Vector2::new(0.25, 0.25));
	st.add_vertex(Vector3::new(1.0,0.0,0.0));
}

//fn end(){
//	
//}

godot_init!(init);

//?: como q faz pra pegar um node
//unsafe {
//	owner.get_node(NodePath).unwrap().assume_safe().cast::<NodeType>().unwrap()
//}
//?: como juntar/transformar duas strings/int/float/etc em uma so string
//format!()









