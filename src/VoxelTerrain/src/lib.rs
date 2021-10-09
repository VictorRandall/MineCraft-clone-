use gdnative::prelude::*;
use gdnative::api::{ArrayMesh, Mesh, MeshInstance, OpenSimplexNoise, SurfaceTool, Spatial, StaticBody, Material};

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct HelloWorld;

#[methods]
impl HelloWorld {
    fn new(_owner: &Spatial) -> Self {
        HelloWorld
    }

    #[export]
    fn _ready(&self, owner: &Spatial) {
        let st = SurfaceTool::new();
        let meshinst = MeshInstance::new();
        
        let size:Vec<u8> = vec![4u8,4u8];
        let id:Vec<u8> = vec![0u8,2u8];
        
        st.begin(Mesh::PRIMITIVE_TRIANGLES);
        
        st.add_uv(Vector2::new((0.0 + id[0] as f32) / size[0] as f32, (0.0 + id[1] as f32) / size[1] as f32));
		st.add_vertex(Vector3::new(0.0,1.0,0.0));
		st.add_uv(Vector2::new((1.0 + id[0] as f32) / size[0] as f32, (0.0 + id[1] as f32) / size[1] as f32));
		st.add_vertex(Vector3::new(1.0,1.0,0.0));
		st.add_uv(Vector2::new((0.0 + id[0] as f32) / size[0] as f32, (1.0 + id[1] as f32) / size[1] as f32));
		st.add_vertex(Vector3::new(0.0,1.0,1.0));
        
        st.add_uv(Vector2::new((0.0 + id[0] as f32) / size[0] as f32, (1.0 + id[1] as f32) / size[1] as f32));
		st.add_vertex(Vector3::new(0.0,1.0,1.0));
		st.add_uv(Vector2::new((1.0 + id[0] as f32) / size[0] as f32, (0.0 + id[1] as f32) / size[1] as f32));
		st.add_vertex(Vector3::new(1.0,1.0,0.0));
		st.add_uv(Vector2::new((1.0 + id[0] as f32) / size[0] as f32, (1.0 + id[1] as f32) / size[1] as f32));
		st.add_vertex(Vector3::new(1.0,1.0,1.0));
        
        st.set_material(ResourceLoader::godot_singleton().load(
            GodotString::from_str("res://assets/new_spatialmaterial.tres"),
            GodotString::from_str("Resource"), false).unwrap().cast::<Material>().unwrap());
        st.generate_normals(false);
        
        meshinst.set_mesh(st.commit(gdnative::Null::null(), Mesh::ARRAY_COMPRESS_DEFAULT).unwrap());
        owner.add_child(meshinst,true);
    }
}

fn init(handle: InitHandle) {
    handle.add_class::<HelloWorld>();
}

godot_init!(init);
	
