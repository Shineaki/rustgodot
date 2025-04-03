extends CharacterBody2D


const SPEED = 50
var directions = [
	Vector2(1, 0), Vector2(-1, 0),
	Vector2(1, 1), Vector2(-1, -1),
	Vector2(0, 1), Vector2(0, -1),
	Vector2(-1, 1), Vector2(1, -1),
	]
var c_dir = Vector2(1, 0)

func _ready() -> void:
	$AnimatedSprite2D.play("Run")
	pass

func _physics_process(delta: float) -> void:
	$AnimatedSprite2D.flip_h = c_dir.x < 0
	var c_velocity = c_dir.normalized() * SPEED * delta
	if move_and_collide(c_velocity):
		c_dir = directions.pick_random()
