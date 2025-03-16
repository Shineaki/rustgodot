extends Control


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func _create_char_callback(result, response_code, headers, body):
	if result != HTTPRequest.RESULT_SUCCESS:
		pass
	else:
		get_tree().change_scene_to_file("res://Scenes/CharacterSelect.tscn")

func _on_back_button_pressed() -> void:
	get_tree().change_scene_to_file("res://Scenes/CharacterSelect.tscn")

func _on_create_button_pressed() -> void:
	# TODO:
	# Name regex (only valid chars, etc)
	# Type select
	# DB structure
	var char_name = $Background/VBoxContainer/CharacterName.text
	Globals.api_post_request("create_character", {
		"name": char_name,
		"type": 3
	}, _create_char_callback)
