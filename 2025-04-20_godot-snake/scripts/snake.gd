extends RefCounted
class_name Snake

var position
var velocity

func _init(field, input):
	position = field.size / 2
	velocity = input.direction()

func update(input):
	velocity = input.direction()
	position += velocity
