extends Node

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
#	get_parent().penis()
	print("_ready (gdscript)")
	pass

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta) -> void:
	printL("_process (gdscript) \ndelta = " + str(delta))
	pos()

func printL(msg:String):
	$Label.text = msg

func pos():
	get_node("Spatial/StaticBody/Position3D").translation = get_node("Spatial/KinematicBody").translation / get_node("Spatial/StaticBody").scale
