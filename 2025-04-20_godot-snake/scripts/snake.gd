extends RefCounted
class_name Snake

var position
var velocity

func _init(input):
	velocity = input.direction()

func update(input):
	velocity = input.direction()
	position += velocity
