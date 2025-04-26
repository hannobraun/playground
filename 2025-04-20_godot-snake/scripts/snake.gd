extends RefCounted
class_name Snake

var positions = Array()
var velocity

func _init(field, input):
	positions.push_back(field.size / 2)
	velocity = input.direction()

func update(field, input):
	velocity = input.direction()
	
	for i in range(positions.size()):
		var position = positions[i]
		
		position += velocity
		position = field.normalize_position(position)
		
		positions[i] = position
