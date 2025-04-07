extends CharacterBody2D


const SPEED = 100.0


func _physics_process(delta: float) -> void:
	var x_direction := Input.get_axis("ui_left", "ui_right")
	var y_direction := Input.get_axis("ui_up", "ui_down")
	if x_direction or y_direction:
		velocity = Vector2(x_direction, y_direction).normalized() * SPEED
		$Animator.flip_h = velocity.x < 0 or (velocity.x == 0 and $Animator.flip_h)
	else:
		velocity = Vector2(0.0, 0.0)
	
	if velocity.is_zero_approx():
		$Animator.play("Idle")
	else:
		$Animator.play("Run")
	
	#$LeftHand.flip_position($Animator.flip_h)
	$RightHand.flip_position($Animator.flip_h)
	$RightHand.set_animation($Animator.animation)

	move_and_slide()
