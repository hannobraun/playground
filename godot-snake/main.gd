extends Node2D

const TILE_SIZE = Vector2(32, 32)
const TICK = 0.2

var field_size

var pos = Vector2(0, 0)
var vel = Vector2(1, 0)

var acc = 0

func _ready():
	field_size = get_viewport().get_visible_rect().size / TILE_SIZE
	pos = field_size / 2

func _input(event: InputEvent):
	if event.keycode == KEY_UP:
		vel = Vector2(0, -1)
	if event.keycode == KEY_LEFT:
		vel = Vector2(-1, 0)
	if event.keycode == KEY_DOWN:
		vel = Vector2(0, 1)
	if event.keycode == KEY_RIGHT:
		vel = Vector2(1, 0)

func _process(delta):
	acc += delta

	while acc >= TICK:
		pos += vel
		acc -= TICK

	if pos.x < 0:
		pos.x = field_size.x - 1
	if pos.y < 0:
		pos.y = field_size.y - 1
	if pos.x >= field_size.x:
		pos.x = 0
	if pos.y >= field_size.y:
		pos.y = 0

	queue_redraw()

func _draw():
	draw_rect(
		Rect2(pos * TILE_SIZE, TILE_SIZE),
		Color(1, 1, 1, 1),
		true,
	)
