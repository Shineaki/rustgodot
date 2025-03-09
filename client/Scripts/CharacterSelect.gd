extends Control


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	$SpinnerBox.visible = true
	$PlayContainer.visible = false
	$CharacterContainer.visible = false
	print("Fetching " + UserData.user_id)
	var scene = load("res://Scenes/PlayerContainer.tscn")
	var collection = Firebase.Firestore.collection('characters')
	var document = await collection.get_doc(UserData.user_id)
	for value in document.keys():
		var pci = scene.instantiate()
		pci.set_player_name(document.get(value).get("name"))
		pci.set_player_level("Level " + str(document.get(value).get("level")))
		$CharacterContainer/VBoxContainer.add_child(pci)
	$SpinnerBox.visible = false
	$PlayContainer.visible = true
	$CharacterContainer.visible = true

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func _on_play_button_pressed() -> void:
	get_tree().change_scene_to_file("res://Scenes/MainScene.tscn")
