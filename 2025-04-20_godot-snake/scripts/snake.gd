extends RefCounted
class_name Snake

var position = Vector2(0, 0)
var vel

func _init(input):
	vel = input.direction()
