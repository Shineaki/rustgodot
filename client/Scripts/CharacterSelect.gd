extends Control


# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	pass
	print("Fetching " + UserData.user_id)
	var collection = Firebase.Firestore.collection('characters')
	var document = await collection.get_doc(UserData.user_id)
	print(document)


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta: float) -> void:
	pass
