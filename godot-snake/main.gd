extends Node2D

const TILE_SIZE = 32

func _draw():
	draw_rect(
		Rect2(Vector2(0, 0), Vector2(TILE_SIZE, TILE_SIZE)),
		Color(1, 1, 1, 1),
		true,
	)

func _process(_delta):
	queue_redraw()
