use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(StaticBody)]

pub struct VoxelChunk;

impl VoxelChunk {
	fn new(_owner: &StaticBody) -> Self {
		VoxelChunk
	}
}
