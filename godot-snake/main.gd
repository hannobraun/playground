extends Node2D

const TILE_SIZE = Vector2(32, 32)
const TICK = 0.2

var field_size

var pos = Vector2(0, 0)
var acc = 0

func _ready():
	field_size = get_viewport().get_visible_rect().size / TILE_SIZE
	pos = field_size / 2

func _draw():
	draw_rect(
		Rect2(pos * TILE_SIZE, TILE_SIZE),
		Color(1, 1, 1, 1),
		true,
	)

func _process(delta):
	acc += delta

	while acc >= TICK:
		pos += Vector2(1, 0)
		acc -= TICK

	queue_redraw()
