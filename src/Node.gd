extends Node


# Declare member variables here. Examples:
# var a = 2
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
#	get_parent().penis()
	printL("_ready (gdscript)")
	pass


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta) -> void:
	printL("_process (gdscript) \ndelta = " + str(delta))
	pass

func printL(msg:String):
	$Label.text = msg
