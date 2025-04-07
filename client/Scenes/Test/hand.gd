extends Node2D


# TODO: Refactor to only call it when player flipped
var init_position = Vector2(0, 0)
#var flipped_direction = false

func _ready() -> void:
	init_position = position

func _process(delta: float) -> void:
	pass

func set_animation(player_animation: String) -> void:
	$Weapon.play(player_animation)

func flip_position(player_flipped: bool) -> void:
	if player_flipped: # and not flipped_direction:
		#flipped_direction = true
		position.x = -1 * init_position.x
	else:
		#flipped_direction = false
		position = init_position
