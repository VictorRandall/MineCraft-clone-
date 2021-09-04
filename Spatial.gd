extends Spatial


var array: Array = []

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	array.append(1)
	array.append(2)
	array.remove(1)
	array.insert(0,"ghey")
	print(array)
	get_tree().reload_current_scene()


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta) -> void:
	pass
