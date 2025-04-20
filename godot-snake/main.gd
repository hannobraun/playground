extends Node2D

func _draw():
	draw_rect(
		Rect2(Vector2(0, 0), Vector2(32, 32)),
		Color(1, 1, 1, 1),
		true,
	)

func _process(_delta):
	queue_redraw()
