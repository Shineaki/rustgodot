extends Node

var usr_data = null
# auth.email
# auth.localid
# auth.idtoken

func api_get_request(endpoint: String, callback: Callable) -> void:
	for _i in self.get_children():
		print(_i)
	var http_request = HTTPRequest.new()
	http_request.request_completed.connect(callback)
	http_request.request_completed.connect(http_request.queue_free.unbind(4))
	add_child(http_request)

	# Perform the HTTP request. The URL below returns a PNG image as of writing.
	var header = ["Authorization: Bearer %s" % Globals.usr_data.idtoken]
	var error = http_request.request("https://rustgodotgame.web.app/api/%s" % endpoint, header)#, HTTPClient.METHOD_POST)
	if error != OK:
		push_error("An error occurred in the HTTP request.")

func api_post_request(endpoint: String, data_to_send: Dictionary, callback: Callable) -> void:
	var http_request = HTTPRequest.new()
	add_child(http_request)
	
	var request_data = JSON.stringify(data_to_send)
	print(request_data)
	http_request.request_completed.connect(callback)

	# Perform the HTTP request. The URL below returns a PNG image as of writing.
	var header = ["Authorization: Bearer %s" % Globals.usr_data.idtoken]
	var error = http_request.request("https://rustgodotgame.web.app/api/%s" % endpoint, header, HTTPClient.METHOD_POST, request_data)
	if error != OK:
		push_error("An error occurred in the HTTP request.")

func parse_response(rsp):
	var parsed_json = JSON.new()
	parsed_json.parse(rsp.get_string_from_utf8())
	return parsed_json.get_data()

func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
