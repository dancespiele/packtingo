extends Ball


# Called when the node enters the scene tree for the first time.
func _ready():
	set_physics_process(true) # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass


func _on_visible_notifier_screen_exited():
	exit_screen() # Replace with function body.
  
