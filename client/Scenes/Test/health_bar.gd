extends Control

var value = 50

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass

func take_damage(damage: int) -> void:
		value -= damage
		if value < 0:
			value = 100
			$UnderBar.value = value
			$OverBar.value = value
		else:
			var tween = create_tween()
			tween.tween_property($OverBar, "value", value, 0.2).set_trans(Tween.TRANS_SINE).set_ease(Tween.EASE_IN_OUT)
			tween.tween_property($UnderBar, "value", value, 0.25).set_trans(Tween.TRANS_SINE).set_ease(Tween.EASE_OUT)
