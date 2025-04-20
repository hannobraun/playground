extends Node2D

const TILE_SIZE = Vector2(32, 32)

var pos = Vector2(0, 0)

func _ready():
	var field_size = get_viewport().get_visible_rect().size / TILE_SIZE
	pos = field_size / 2

func _draw():
	draw_rect(
		Rect2(pos * TILE_SIZE, TILE_SIZE),
		Color(1, 1, 1, 1),
		true,
	)

func _process(_delta):
	queue_redraw()
