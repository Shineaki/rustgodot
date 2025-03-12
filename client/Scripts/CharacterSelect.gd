extends Control

func api_request(endpoint: String, callback: Callable) -> void:
	var http_request = HTTPRequest.new()
	add_child(http_request)
	http_request.request_completed.connect(callback)

	# Perform the HTTP request. The URL below returns a PNG image as of writing.
	var header = ["Authorization: Bearer %s" % Globals.usr_data.idtoken]
	var error = http_request.request("https://rustgodotgame.web.app/api/%s" % endpoint, header)
	if error != OK:
		push_error("An error occurred in the HTTP request.")

func parse_response(rsp):
	var parsed_json = JSON.new()
	parsed_json.parse(rsp.get_string_from_utf8())
	return parsed_json.get_data()

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	$SpinnerBox.visible = true
	$PlayContainer.visible = false
	$CharacterContainer.visible = false
	# Create an HTTP request node and connect its completion signal.
	api_request("characters", self._get_character_list_completed)

# Called when the HTTP request is completed.
func _get_character_list_completed(result, response_code, headers, body):
	$SpinnerBox.visible = false
	$PlayContainer.visible = true
	$CharacterContainer.visible = true
	if result != HTTPRequest.RESULT_SUCCESS:
		push_error("API call failed")
	var scene = load("res://Scenes/PlayerContainer.tscn")
	
	var response = self.parse_response(body)
	var characters = 0
	for value in response.keys():
		characters += 1
		var pci = scene.instantiate()
		pci.set_player_name(response.get(value).get("name"))
		pci.set_player_level("Level " + str(response.get(value).get("level")))
		$CharacterContainer/VBoxContainer.add_child(pci)
	
	if characters >= 4: # NUMBER_OF_MAX_CHARACTERS_PER_ACCOUNT
		$CharacterContainer/VBoxContainer/CreateNewCharacterButton.visible = false

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func _on_play_button_pressed() -> void:
	api_request("characters", self._http_request_completed)

# Called when the HTTP request is completed.
func _http_request_completed(result, response_code, headers, body):
	if result != HTTPRequest.RESULT_SUCCESS:
		push_error("API call failed")
	var response = self.parse_response(body)
	print(response)

func _on_logout_button_pressed() -> void:
	Firebase.Auth.logout()
	get_tree().change_scene_to_file("res://Scenes/Authentication.tscn")
