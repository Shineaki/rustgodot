extends Node2D

var i = 0

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	print(UserData.user_id) # Globals
	#print(Firebase.Auth.auth) # auth of the lib
	#TODO: refresh token every 1 hour


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	i += 1
	if i >= 1000:
		i = 0
		print(Firebase.Auth.auth)
