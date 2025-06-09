extends RefCounted
class_name Snake

var length = 3
var positions = Array()
var velocity

func _init(field, input):
	positions.push_back(field.size / 2)
	velocity = input.direction()

func update(field, input):
	velocity = input.direction()
	
	var previous_position = positions[0]
	positions[0] = positions[0] + velocity
	positions[0] = field.normalize_position(positions[0])
	
	for i in range(1, positions.size()):
		var next_previous_position = positions[i]
		positions[i] = previous_position
		previous_position = next_previous_position

	if positions.size() < length:
		positions.push_back(positions.back() - velocity)
