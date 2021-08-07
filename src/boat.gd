extends RigidBody

var mouse_moviment = Vector2()

# Called when the node enters the scene tree for the first time.
func _ready():
	Input.set_mouse_mode(Input.MOUSE_MODE_CAPTURED)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _input(event):
	if event is InputEventMouseMotion:
		mouse_moviment += event.relative

func _physics_process(delta):
	if mouse_moviment != Vector2():
		$H.rotation_degrees.y += -mouse_moviment.x
		$H/V.rotation_degrees.x += -mouse_moviment.y
		if $H/V.rotation_degrees.x <= -90:
			$H/V.rotation_degrees.x = -90
		if $H/V.rotation_degrees.x <= 0:
			$H/V.rotation_degrees.x = 0
		mouse_moviment = Vector2()
	
	if Input.is_action_pressed("ui_up"):
		add_central_force(global_transform.basis.xform(Vector3.FORWARD*20))
		if Input.is_action_pressed("ui_left"):
			add_torque(Vector3(0,5,0))
		if Input.is_action_pressed("ui_right"):
			add_torque(Vector3(0,-5,0))
	
	elif Input.is_action_pressed("ui_down"):
		add_central_force(global_transform.basis.xform(Vector3.BACK*10))
		if Input.is_action_pressed("ui_left"):
			add_torque(Vector3(0,5,0))
		if Input.is_action_pressed("ui_right"):
			add_torque(Vector3(0,-5,0))
			
	if $Floaty.global_transform.origin.y <= 0:
		add_force(Vector3.UP*5*-$Floaty.global_transform.origin, $Floaty.global_transform.origin-global_transform.origin)
	if $Floaty2.global_transform.origin.y <= 0:
		add_force(Vector3.UP*5*-$Floaty2.global_transform.origin, $Floaty2.global_transform.origin-global_transform.origin)
	if $Floaty3.global_transform.origin.y <= 0:
		add_force(Vector3.UP*5*-$Floaty3.global_transform.origin, $Floaty3.global_transform.origin-global_transform.origin)
	if $Floaty4.global_transform.origin.y <= 0:
		add_force(Vector3.UP*5*-$Floaty4.global_transform.origin, $Floaty4.global_transform.origin-global_transform.origin)
