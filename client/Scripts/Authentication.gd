extends Control


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	# https://github.com/GodotNuts/GodotFirebase/wiki/Authentication-and-User-Management#save-encrypted-auth-file
	Firebase.Auth.login_succeeded.connect(on_login_succeeded)
	Firebase.Auth.signup_succeeded.connect(on_signup_succeeded)
	Firebase.Auth.login_failed.connect(on_login_failed)
	Firebase.Auth.signup_failed.connect(on_signup_failed)
	
	if Firebase.Auth.check_auth_file():
		print("Auth file exists, loading it ...")

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func _on_login_button_pressed() -> void:
	var email = $VBoxContainer/EmailInput.text
	var password = $VBoxContainer/PasswordInput.text
	$VBoxContainer/EmailInput.editable = false
	$VBoxContainer/PasswordInput.editable = false
	Firebase.Auth.login_with_email_and_password(email, password)
	$VBoxContainer/StateLabel.text = "Logging in"
	$VBoxContainer/ErrorLabel.text = ""

func _on_signup_button_pressed() -> void:
	var email = $VBoxContainer/EmailInput.text
	var password = $VBoxContainer/PasswordInput.text
	$VBoxContainer/EmailInput.editable = false
	$VBoxContainer/PasswordInput.editable = false
	Firebase.Auth.signup_with_email_and_password(email, password)
	$VBoxContainer/StateLabel.text = "Signing Up"
	$VBoxContainer/ErrorLabel.text = ""

func on_login_succeeded(auth):
	print("on_login_succeeded")
	$VBoxContainer/StateLabel.text = "Login Success!"
	$VBoxContainer/ErrorLabel.text = ""
	$VBoxContainer/EmailInput.editable = true
	$VBoxContainer/PasswordInput.editable = true
	Globals.usr_data = auth
	Firebase.Auth.save_auth(auth)
	get_tree().change_scene_to_file("res://Scenes/CharacterSelect.tscn")
	
func on_signup_succeeded(auth):
	$VBoxContainer/StateLabel.text = "Sign Up Success!"
	$VBoxContainer/ErrorLabel.text = ""
	$VBoxContainer/EmailInput.editable = true
	$VBoxContainer/PasswordInput.editable = true

func on_login_failed(error_code, message):
	$VBoxContainer/StateLabel.text = "Login Failed!"
	$VBoxContainer/ErrorLabel.text = message
	$VBoxContainer/EmailInput.editable = true
	$VBoxContainer/PasswordInput.editable = true

func on_signup_failed(error_code, message):
	$VBoxContainer/StateLabel.text = "Sign Up Failed!"
	$VBoxContainer/ErrorLabel.text = message
	$VBoxContainer/EmailInput.editable = true
	$VBoxContainer/PasswordInput.editable = true
