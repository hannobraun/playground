extends RefCounted
class_name Field

const TILE_SIZE = Vector2(16, 16)

var size

func _init(window_size):
	size = window_size / TILE_SIZE

func normalize_position(position):
	if position.x < 0:
		position.x = size.x - 1
	if position.y < 0:
		position.y = size.y - 1
	if position.x >= size.x:
		position.x = 0
	if position.y >= size.y:
		position.y = 0
	
	return position
