extends RefCounted
class_name Snake

var position = Vector2(0, 0)
var velocity

func _init(input):
	velocity = input.direction()

func update(input):
	velocity = input.direction()
	position += velocity
