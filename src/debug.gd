extends Label


# Declare member variables here. Examples:
# var a = 2
# var b = "text"

onready var player: KinematicBody = get_node("../KinematicBody")

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	var debug_text: String = "player.translation = " + str(self.player.translation) + "\nbpos = " + str(self.player.bpos)
	self.text = debug_text
