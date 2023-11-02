extends Ball


# Called when the node enters the scene tree for the first time.
func _ready():
	set_physics_process(true) # Replace with function body.

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _physics_process(delta):
	
	if Input.is_action_pressed("start") && target_velocity == Vector3.ZERO:
		target_velocity.x = randf_range(1.0, 3.0) * delta
		target_velocity.z = randf_range(-1.0, 1.0) * delta
		
		initial_position = position
		velocity = target_velocity

	if target_velocity != Vector3.ZERO:
		var current_velocity = velocity
		var collision_object = move_and_collide(current_velocity)

		if collision_object:
			var bounce = current_velocity.bounce(collision_object.get_normal())
			bounce.x = bounce.x + randf_range(-0.9, 2.0) * delta
			velocity = bounce


func exit_screen():
	target_velocity = Vector3.ZERO
	velocity = target_velocity
	position = initial_position

func _on_visible_notifier_screen_exited():
	exit_screen() # Replace with function body.
  
