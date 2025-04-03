extends Node

const enemy_stuff = preload("res://Scenes/Enemy.tscn")
func _ready() -> void:
	for i in 100:
		var c_enemy = enemy_stuff.instantiate()
		add_child(c_enemy)
		


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
