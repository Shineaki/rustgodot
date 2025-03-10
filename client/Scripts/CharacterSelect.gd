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
	#get_tree().change_scene_to_file("res://Scenes/MainScene.tscn")
		# Create an HTTP request node and connect its completion signal.
	var http_request = HTTPRequest.new()
	add_child(http_request)
	http_request.request_completed.connect(self._http_request_completed)

	# Perform the HTTP request. The URL below returns a PNG image as of writing.
	var header = ["Authorization: Bearer %s" % UserData.token]
	print(header)
	var error = http_request.request("https://rustgodotgame.web.app/api/qwe", header)
	if error != OK:
		push_error("An error occurred in the HTTP request.")

# Called when the HTTP request is completed.
func _http_request_completed(result, response_code, headers, body):
	if result != HTTPRequest.RESULT_SUCCESS:
		push_error("API call failed")
	var json = JSON.new()
	json.parse(body.get_string_from_utf8())
	print(body.get_string_from_utf8())
	var response = json.get_data()
	print(response)
