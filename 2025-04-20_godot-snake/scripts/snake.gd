extends RefCounted
class_name Snake

var pos = Vector2(0, 0)
var vel

func _init(input):
	vel = input.direction()
