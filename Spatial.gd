extends Spatial
tool

onready var mshinst = get_node("MeshInstance")
# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	var st = SurfaceTool.new()
	st.begin(Mesh.PRIMITIVE_TRIANGLES)
	
	
	st.add_triangle_fan([Vector3(0,0,0),Vector3(1,0,0),Vector3(1,0,1)],[Vector2(0,0),Vector2(1,0),Vector2(1,1)])
	
	st.generate_normals()
	mshinst.mesh = st.commit()


# Called every frame. 'delta' is the elapsed time since the previous frame.
#func _process(delta):
#	pass
