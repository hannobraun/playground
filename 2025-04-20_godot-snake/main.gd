extends Node2D

const TILE_SIZE = Vector2(16, 16)
const TICK = 0.1

const UP = Vector2(0, -1)
const LEFT = Vector2(-1, 0)
const DOWN = Vector2(0, 1)
const RIGHT = Vector2(1, 0)

var field_size

var pos = Vector2(0, 0)
var vel = Vector2(1, 0)

var acc = 0

func _ready():
	field_size = get_viewport().get_visible_rect().size / TILE_SIZE
	pos = field_size / 2

func _input(event: InputEvent):
	if event is InputEventKey:
		if event.keycode == KEY_UP and vel != DOWN:
			vel = UP
		if event.keycode == KEY_LEFT and vel != RIGHT:
			vel = LEFT
		if event.keycode == KEY_DOWN and vel != UP:
			vel = DOWN
		if event.keycode == KEY_RIGHT and vel != LEFT:
			vel = RIGHT

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
