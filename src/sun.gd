extends Spatial


# Declare member variables here. Examples:
# var a: int = 2
# var b: String = "text"


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	rotation.x += 0.5 * delta
	print(rotation)
#	print(get_node("../Spatial").get_voxel(1,1,1))
	
	if Input.is_action_just_pressed("adad"):
		print("a")
		if Engine.time_scale == 0.25:
			Engine.time_scale = 1.0
		else:
			Engine.time_scale = 0.25
