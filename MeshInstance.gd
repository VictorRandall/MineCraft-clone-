	extends MeshInstance
#tool

# Called when the node enters the scene tree for the first time.
func _ready():
#	var st:SurfaceTool = SurfaceTool.new()
#
#	st.add_uv(Vector2(1,0))
#	st.add_vertex(Vector3(0,1,0))
#	st.add_uv(Vector2(0,1))
#	st.add_vertex(Vector3(1,1,0))
#	st.add_uv(Vector2(0,0))
#	st.add_vertex(Vector3(0,1,1)) 
#
#	st.generate_normals()
#	var finalmesh = st.commit()
#	mesh = finalmesh# Replace with function body.
	pass

# Called every frame. 'delta' is the elapsed time since the previous frame.
export var img_size:float = 4
export(Vector2) var id

func _process(delta):
	var st:SurfaceTool = SurfaceTool.new()

	st.begin(Mesh.PRIMITIVE_TRIANGLES)
	st.set_material(preload("res://src/blocks.tres"))

	st.add_uv(Vector2(0,0.0))
	st.add_vertex(Vector3(0,1,0))
	st.add_uv(Vector2(0.25,0))
	st.add_vertex(Vector3(1,1,0))
	st.add_uv(Vector2(0,0.25))
	st.add_vertex(Vector3(0,1,1)) 

	st.add_uv(Vector2(0,0.25))
	st.add_vertex(Vector3(0,1,1))
	st.add_uv(Vector2(0.25,0))
	st.add_vertex(Vector3(1,1,0))
	st.add_uv(Vector2(0.25,0.25))
	st.add_vertex(Vector3(1,1,1))

	st.generate_normals()
	mesh = st.commit()# Replace with function body.
	pass
