extends Control


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	# https://github.com/GodotNuts/GodotFirebase/wiki/Authentication-and-User-Management#save-encrypted-auth-file
	$FormContainer.visible = false
	$SpinnerBox.visible = true
	Firebase.Auth.login_succeeded.connect(on_login_succeeded)
	Firebase.Auth.signup_succeeded.connect(on_signup_succeeded)
	Firebase.Auth.login_failed.connect(on_login_failed)
	Firebase.Auth.signup_failed.connect(on_signup_failed)
	
	if Firebase.Auth.check_auth_file():
		print("Auth file exists, loading it ...")
	else:
		$FormContainer.visible = true
		$SpinnerBox.visible = false

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func _on_login_button_pressed() -> void:
	var email = $FormContainer/EmailInput.text
	var password = $FormContainer/PasswordInput.text
	$FormContainer/EmailInput.editable = false
	$FormContainer/PasswordInput.editable = false
	Firebase.Auth.login_with_email_and_password(email, password)
	$FormContainer/StateLabel.text = "Logging in"
	$FormContainer/ErrorLabel.text = ""

func _on_signup_button_pressed() -> void:
	var email = $FormContainer/EmailInput.text
	var password = $FormContainer/PasswordInput.text
	$FormContainer/EmailInput.editable = false
	$FormContainer/PasswordInput.editable = false
	Firebase.Auth.signup_with_email_and_password(email, password)
	$FormContainer/StateLabel.text = "Signing Up"
	$FormContainer/ErrorLabel.text = ""

func on_login_succeeded(auth):
	print("on_login_succeeded")
	$FormContainer/StateLabel.text = "Login Success!"
	$FormContainer/ErrorLabel.text = ""
	$FormContainer/EmailInput.editable = true
	$FormContainer/PasswordInput.editable = true
	Globals.usr_data = auth
	Firebase.Auth.save_auth(auth)
	get_tree().change_scene_to_file("res://Scenes/CharacterSelect.tscn")
	
func on_signup_succeeded(auth):
	$FormContainer/StateLabel.text = "Sign Up Success!"
	$FormContainer/ErrorLabel.text = ""
	$FormContainer/EmailInput.editable = true
	$FormContainer/PasswordInput.editable = true

func on_login_failed(error_code, message):
	$FormContainer/StateLabel.text = "Login Failed!"
	$FormContainer/ErrorLabel.text = message
	$FormContainer.visible = true
	$SpinnerBox.visible = false
	$FormContainer/EmailInput.editable = true
	$FormContainer/PasswordInput.editable = true

func on_signup_failed(error_code, message):
	$FormContainer/StateLabel.text = "Sign Up Failed!"
	$FormContainer/ErrorLabel.text = message
	$FormContainer/EmailInput.editable = true
	$FormContainer/PasswordInput.editable = true
