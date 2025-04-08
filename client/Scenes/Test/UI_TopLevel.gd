extends CanvasLayer

var player_target = 1

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	$HBoxContainer/TargetUI.set_panel_name("Minion")


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	if Input.is_key_pressed(KEY_ESCAPE):
		if player_target != null:
			player_target.clear_selection()
		player_target = null

	if player_target == null:
		$HBoxContainer/TargetUI.visible = false
	else:
		$HBoxContainer/TargetUI.visible = true


func _on_enemy_targeted(object) -> void:
	var name = object.get_object_name()
	player_target = object
	$HBoxContainer/TargetUI.set_panel_name(name)
