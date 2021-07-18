extends CollisionShape

class_name VoxelChunk

var mat = preload("res://src/blocks.tres")
var mesh_instance
var noise_seed
var x
var y
var z
var chunk_size
var should_remove = true
var data = {}


func _init(noise_seed, x, z, chunk_size):
	self.x = x
	self.z = z
	self.chunk_size = chunk_size
	self.noise_seed = noise_seed

func _ready():
	generate_chunk()
#	generate_water()
	

func generate_chunk():
	var st = SurfaceTool.new()
#	st.set_material(mat)
#	st.generate_normals()
	st.begin(Mesh.PRIMITIVE_TRIANGLES)
#	st.create_from(planemesh,)
	for x in range(-4,4):
		for y in range(-1,1):
			for z in range(-4,4):
				cube_faces(st, data, Vector3(x,y,z))
	
	
	var mi = MeshInstance.new()
	var m = Mesh.new()
	var s = ConvexPolygonShape.new()
	var arrm = ArrayMesh.new()
	mi.mesh = st.commit()
#	s.set_points()
	shape = s
	mi.create_trimesh_collision()
	add_child(mi)

func cube_faces(st: SurfaceTool,data: Dictionary,vec: Vector3):
	#top
#	st.add_uv(Vector2(0,1))
#	st.add_uv(Vector2(1,0))
#	st.add_uv(Vector2(0,0))
	
	st.add_uv(Vector2(1,0))
	st.add_vertex(Vector3(vec.x+0,vec.y+1,vec.z+0))
	st.add_uv(Vector2(0,1))
	st.add_vertex(Vector3(vec.x+1,vec.y+1,vec.z+0))
	st.add_uv(Vector2(0,0))
	st.add_vertex(Vector3(vec.x+0,vec.y+1,vec.z+1))
	
	st.add_vertex(Vector3(vec.x+0,vec.y+1,vec.z+1))
	st.add_vertex(Vector3(vec.x+1,vec.y+1,vec.z+0))
	st.add_vertex(Vector3(vec.x+1,vec.y+1,vec.z+1))
	#botton
	st.add_vertex(Vector3(vec.x+0,vec.y+0,vec.z+1))
	st.add_vertex(Vector3(vec.x+1,vec.y+0,vec.z+0))
	st.add_vertex(Vector3(vec.x+0,vec.y+0,vec.z+0))

	st.add_vertex(Vector3(vec.x+1,vec.y+0,vec.z+1))
	st.add_vertex(Vector3(vec.x+1,vec.y+0,vec.z+0))
	st.add_vertex(Vector3(vec.x+0,vec.y+0,vec.z+1))
	#left
	st.add_vertex(Vector3(vec.x+1,vec.y+0,vec.z+0))
	st.add_vertex(Vector3(vec.x+1,vec.y+0,vec.z+1))
	st.add_vertex(Vector3(vec.x+1,vec.y+1,vec.z+0))
#
	st.add_vertex(Vector3(vec.x+1,vec.y+0,vec.z+1))
	st.add_vertex(Vector3(vec.x+1,vec.y+1,vec.z+1))
	st.add_vertex(Vector3(vec.x+1,vec.y+1,vec.z+0))
	#right
	st.add_vertex(Vector3(vec.x+0,vec.y+1,vec.z+0))
	st.add_vertex(Vector3(vec.x+0,vec.y+0,vec.z+1))
	st.add_vertex(Vector3(vec.x+0,vec.y+0,vec.z+0))

	st.add_vertex(Vector3(vec.x+0,vec.y+1,vec.z+1))
	st.add_vertex(Vector3(vec.x+0,vec.y+0,vec.z+1))
	st.add_vertex(Vector3(vec.x+0,vec.y+1,vec.z+0))
	#front
	st.add_vertex(Vector3(vec.x+0,vec.y+0,vec.z+1))
	st.add_vertex(Vector3(vec.x+0,vec.y+1,vec.z+1))
	st.add_vertex(Vector3(vec.x+1,vec.y+0,vec.z+1))

	st.add_vertex(Vector3(vec.x+1,vec.y+0,vec.z+1))
	st.add_vertex(Vector3(vec.x+0,vec.y+1,vec.z+1))
	st.add_vertex(Vector3(vec.x+1,vec.y+1,vec.z+1))
	#back
	st.add_vertex(Vector3(vec.x+1,vec.y+0,vec.z+0))
	st.add_vertex(Vector3(vec.x+0,vec.y+1,vec.z+0))
	st.add_vertex(Vector3(vec.x+0,vec.y+0,vec.z+0))

	st.add_vertex(Vector3(vec.x+1,vec.y+1,vec.z+0))
	st.add_vertex(Vector3(vec.x+0,vec.y+1,vec.z+0))
	st.add_vertex(Vector3(vec.x+1,vec.y+0,vec.z+0))
	
	st.generate_normals()



func generate_water():
	var plane_mesh = PlaneMesh.new()
	plane_mesh.size = Vector2(chunk_size, chunk_size)
	
	# you may need to change to water.tres if that is your material extension
	
	var mesh_instance = MeshInstance.new()
	mesh_instance.mesh = plane_mesh
	
	add_child(mesh_instance)

func set_block(x,y,z,id):
	pass

func get_block(x,y,z):
	var id = 0
	return id
