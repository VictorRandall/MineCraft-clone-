use gdnative::api::{ArrayMesh, Mesh, MeshInstance, OpenSimplexNoise, SurfaceTool, Spatial, StaticBody};
use gdnative::prelude::*;

pub struct Voxel<T,Y,R,S>{
	solid:bool,
	id:u16,
//	mesh:Vec<Option<gdnative::Ref<ArrayMesh, gdnative::prelude::Unique>>>,
	parameters:Option<Vec<T>>,
	parameters2:Option<Vec<Y>>,
	parameters3:Option<Vec<R>>,
	parameters4:Option<Vec<S>>,
}

impl<T,Y,R,S> Voxel<T,Y,R,S>{
	pub fn new(sld:bool, id:u16,prmtrs:Option<Vec<T>>,prmtrs2:Option<Vec<Y>>,prmtrs3:Option<Vec<R>>,prmtrs4:Option<Vec<S>>) -> Self{
		return Voxel{
			solid:sld,
			id:id,
//			mesh:vec![Some(gdnative::Ref<ArrayMesh, gdnative::prelude::Unique>]),
			parameters:prmtrs,
			parameters2:prmtrs2,
			parameters3:prmtrs3,
			parameters4:prmtrs4,
		}
	}
	pub fn is_solid(&self) -> bool{
		return self.solid
	}
}
