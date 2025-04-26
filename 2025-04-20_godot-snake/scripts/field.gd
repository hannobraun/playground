extends RefCounted
class_name Field

const TILE_SIZE = Vector2(16, 16)

var size

func _init(window_size):
	size = window_size / TILE_SIZE
