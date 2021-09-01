extends KinematicBody

var moviment: Vector3 = Vector3();
var speed:float = 5.0
var G:float = -10.0;
var cam_mov:float = 0.2;
onready var cam:Spatial = $Spatial;

func _ready() -> void:
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)

func _input(event: InputEvent) -> void:
	if event is InputEventMouseMotion:
		var movement = event.relative;
		cam.rotation.x += -deg2rad(movement.y * cam_mov);
		cam.rotation.x = clamp(cam.rotation.x, deg2rad(-90), deg2rad(90));
		rotation.y += -deg2rad(movement.x * cam_mov);

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	if moviment.x != 0 and moviment.z != 0:
		moviment.x = lerp(moviment.x, 0, delta * 3);
		moviment.z = lerp(moviment.z, 0, delta * 3);
		print(lerp(moviment.x, 0, delta * 3));
		print(lerp(moviment.z, 0, delta * 3));
	
#	print();
	if Input.is_action_pressed("gp_up"):
		moviment += -transform.basis.z * speed;
	elif Input.is_action_pressed("gp_down"):
		moviment += transform.basis.z * speed;
	
	if Input.is_action_pressed("gp_left"):
		moviment += -transform.basis.x * speed;
	elif Input.is_action_pressed("gp_right"):
		moviment += transform.basis.x * speed;
	
	if not is_on_floor():
		moviment.y -= -G * delta;
	else:
		moviment.y = 0.0;
		if Input.is_action_pressed("gp_jump"):
			moviment.y += 10.0;
	
	moviment = move_and_slide(moviment, Vector3.UP);
