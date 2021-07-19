extends MeshInstance

var img_size:float = 4
var id:Vector2 = Vector2(2.0,.0) 


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
func _process(delta):
	var st:SurfaceTool = SurfaceTool.new()

	st.begin(Mesh.PRIMITIVE_TRIANGLES)
	st.set_material(preload("res://src/blocks.tres"))

	st.add_uv(Vector2(0+(id.x/img_size),0+(id.y/img_size)))
	st.add_vertex(Vector3(0,1,0))
	st.add_uv(Vector2(0.25+(id.x/img_size),0+(id.y/img_size)))
	st.add_vertex(Vector3(1,1,0))
	st.add_uv(Vector2(0+(id.x/img_size),0.25+(id.y/img_size)))
	st.add_vertex(Vector3(0,1,1)) 

	st.add_uv(Vector2(0+(id.x/img_size),0.25+(id.y/img_size)))
	st.add_vertex(Vector3(0,1,1))
	st.add_uv(Vector2(0.25+(id.x/img_size),0+(id.y/img_size)))
	st.add_vertex(Vector3(1,1,0))
	st.add_uv(Vector2(0.25+(id.x/img_size),0.25+(id.y/img_size)))
	st.add_vertex(Vector3(1,1,1))

	st.generate_normals()
	mesh = st.commit()# Replace with function body.
	pass
