extends Spatial


# Declare member variables here. Examples:
var noise = OpenSimplexNoise.new()
# var b = "text"


# Called when the node enters the scene tree for the first time.
func _ready():
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	var idk = 2 + delta
	rotation.x += noise.get_noise_3d(idk,idk,idk)
	rotation.y += noise.get_noise_3d(idk,idk,idk)
#	rotation.z += noise.get_noise_3d(idk,idk,idk)
