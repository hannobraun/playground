extends RefCounted
class_name PlayerInput

const UP = Vector2(0, -1)
const LEFT = Vector2(-1, 0)
const DOWN = Vector2(0, 1)
const RIGHT = Vector2(1, 0)

var _direction = RIGHT
var _events = Array()

func direction():
	if not _events.is_empty():
		_direction = _events.pop_front()

	return _direction

func on_new_direction(new_direction):
	if new_direction == null:
		return
		
	var opposite = _direction * -1
	
	var not_reversing_direction
	var not_duplicating_event
	
	if _events.is_empty():
		not_reversing_direction = new_direction != opposite
		not_duplicating_event = new_direction != _direction
	else:
		not_reversing_direction = _events.back() != opposite
		not_duplicating_event = not _events.has(new_direction)
	
	if not_reversing_direction and not_duplicating_event:
		_events.push_back(new_direction)
