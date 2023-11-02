extends Node3D


# Called when the node enters the scene tree for the first time.
func _ready():
	var z_position = -3
	var y_position = 6

	for i in range(4):
		for x in range(5):
			var box_scene = preload("res://box.tscn").instantiate()

			add_child(box_scene)

			z_position = z_position + 1

			box_scene.position = Vector3(y_position, -1, z_position)
			box_scene.scale = Vector3(2, 2, 2)
		
		z_position = -3
		y_position = y_position - 1
		

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	pass
