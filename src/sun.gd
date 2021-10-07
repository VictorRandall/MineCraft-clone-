extends Spatial


# Declare member variables here. Examples:
# var a: int = 2
# var b: String = "text"


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	rotation_degrees.x += 10 * delta
	if rotation_degrees.x > 270:
		rotation_degrees.x = 0
	print(rotation_degrees)
#	print(get_node("../VoxelSistem")"res://assets/new_spatialmaterial.tres"
	get_node("../VoxelSistem").set_voxel(8,1,1)
	
	if Input.is_action_just_pressed("adad"):
		print("a")
		if Engine.time_scale == 0.25:
			Engine.time_scale = 1.0
		else:
			Engine.time_scale = 0.25
