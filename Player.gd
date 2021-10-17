extends KinematicBody

var moviment: Vector3 = Vector3();
var speed: float
var w_speed: float = 12.0
var r_speed: float = 34.0
var G: float = -10.0
var cam_mov: float = 0.2

var cam3person: bool = false

onready var cam: Spatial = $Spatial
onready var raycast: RayCast = $Spatial/RayCast
onready var player_model: Spatial = $Player

func _ready() -> void:
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)

func _input(event: InputEvent) -> void:
	if event is InputEventMouseMotion:
		var movement = event.relative
		cam.rotation.x += -deg2rad(movement.y * cam_mov)
		cam.rotation.x = clamp(cam.rotation.x, deg2rad(-90), deg2rad(90))
		rotation.y += -deg2rad(movement.x * cam_mov)
	
	match cam3person:
		false:
			cam.get_node("Camera").translation = Vector3(0.0, 0.0, 0.0)
		true:
			cam.get_node("Camera").translation = Vector3(0.0, 1.0, 7.0)
			
	if Input.is_action_just_pressed("gp_switch"):
		cam3person = not cam3person
		player_model.visible = cam3person
		print(player_model.get_children())

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	if moviment.x != 0 and moviment.z != 0:
#		moviment.x = lerp(moviment.x, 0, delta * 3)
#		moviment.z = lerp(moviment.z, 0, delta * 3)
		moviment.x = 0
		moviment.z = 0
		player_model.get_node("AnimationPlayer").play("idle")
#		print(lerp(moviment.x, 0, delta * 3))
#		print(lerp(moviment.z, 0, delta * 3))
	
	if raycast.is_colliding():
		var norm = raycast.get_collision_normal();
		var pos = raycast.get_collision_point() - norm * 0.5
		
		var bx = floor(pos.x) + 0.5
		var by = floor(pos.y) + 0.5
		var bz = floor(pos.z) + 0.5
		var bpos = Vector3(bx, by, bz) - self.translation
		
#		block_outline.translation = bpos
#		block_outline.visible = true
		
		if Input.is_action_pressed("gp_break"):
			get_node("..").set_voxel(Vector3(int(pos.x),int(pos.y),int(pos.z)), 0)
#			print(str(int(bpos.x)) + str(int(bpos.y)) + str(int(bpos.z)))
		
		if Input.is_action_just_pressed("gp_place"):
			get_node("..").set_voxel(Vector3(int(pos.x + norm.x),int(pos.y + norm.y),int(pos.z + norm.z)), 1)
#			print(str(int(bpos.x)) + str(int(bpos.y)) + str(int(bpos.z)))
	
#	print();
	if Input.is_action_pressed("gp_up"):
		moviment += -transform.basis.z * speed
		if not player_model.get_node("AnimationPlayer").current_animation == "walk":
			player_model.get_node("AnimationPlayer").play("walk")
	elif Input.is_action_pressed("gp_down"):
		moviment += transform.basis.z * speed
		if not player_model.get_node("AnimationPlayer").current_animation == "walk":
			player_model.get_node("AnimationPlayer").play("walk")
	
	if Input.is_action_pressed("gp_left"):
		moviment += -transform.basis.x * speed
		if not player_model.get_node("AnimationPlayer").current_animation == "walk":
			player_model.get_node("AnimationPlayer").play("walk")
	elif Input.is_action_pressed("gp_right"):
		moviment += transform.basis.x * speed
		if not player_model.get_node("AnimationPlayer").current_animation == "walk":
			player_model.get_node("AnimationPlayer").play("walk")
	
	if Input.is_action_pressed("gp_sprint"):
		speed = r_speed
	else:
		speed = w_speed
	
	if not is_on_floor():
		moviment.y -= -G * delta
#		pass
	else:
		moviment.y = 0.0
		if Input.is_action_pressed("gp_jump"):
			moviment.y += 10.0
	
	moviment = move_and_slide(moviment, Vector3.UP,true,4,0.785398,true)
