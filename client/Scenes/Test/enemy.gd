extends CharacterBody2D

signal targeted(reference)

func get_object_name() -> String:
	return "Minionka"

func _physics_process(delta: float) -> void:
	pass

func clear_selection() -> void:
	$SelectedAnimation.visible = false
	

func _on_area_2d_input_event(viewport: Node, event: InputEvent, shape_idx: int) -> void:
	if event is InputEventMouseButton and event.is_pressed():
		if event.button_index == 1: # Left click
			$SelectedAnimation.visible = true
			targeted.emit(self)
