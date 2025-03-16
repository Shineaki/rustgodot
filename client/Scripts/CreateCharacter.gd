extends Control

var selected_type = null;

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func _create_char_callback(result, response_code, headers, body):
	if result != HTTPRequest.RESULT_SUCCESS:
		print(result)
		print(response_code)
		print(body)
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
	if char_name == null or char_name == "":
		print("Add a character name")
		return
	if selected_type == null:
		print("Select a character type")
		return
	Globals.api_post_request("create_character", {
		"name": char_name,
		"type": selected_type
	}, _create_char_callback)

func _on_type_button_pressed(extra_arg_0: int) -> void:
	# TODO: refactor, instantiate from code?
	var button_list = [
		$Background/VBoxContainer/HBoxContainer/ElfButton,
		$Background/VBoxContainer/HBoxContainer/WizzardButton,
		$Background/VBoxContainer/HBoxContainer/KnightButton,
		$Background/VBoxContainer/HBoxContainer/LizardButton
	]
	var style_box = StyleBoxFlat.new()
	style_box.bg_color = Color.DEEP_SKY_BLUE
	style_box.border_color = Color.DEEP_SKY_BLUE
	for idx in button_list.size():
		var btn: Button = button_list[idx];
		if idx == extra_arg_0:
			btn.add_theme_stylebox_override("normal", style_box)
			btn.add_theme_stylebox_override("hover", style_box)
			btn.add_theme_stylebox_override("pressed", style_box)
		else:
			btn.remove_theme_stylebox_override("normal")
			btn.remove_theme_stylebox_override("hover")
			btn.remove_theme_stylebox_override("pressed")
	selected_type = extra_arg_0
