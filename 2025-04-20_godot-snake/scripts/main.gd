extends Node2D

const TILE_SIZE = Vector2(16, 16)
const TICK = 0.1

var field_size

var input = PlayerInput.new()
var snake = Snake.new(input)

var time_acc = 0

func _ready():
	field_size = get_viewport().get_visible_rect().size / TILE_SIZE
	snake.pos = field_size / 2

func _input(event: InputEvent):
	if event is InputEventKey and event.is_pressed():
		if event.keycode == KEY_ESCAPE:
			get_tree().quit()
			
		if event.keycode == KEY_UP:
			input.on_new_direction(PlayerInput.UP)
		if event.keycode == KEY_LEFT:
			input.on_new_direction(PlayerInput.LEFT)
		if event.keycode == KEY_DOWN:
			input.on_new_direction(PlayerInput.DOWN)
		if event.keycode == KEY_RIGHT:
			input.on_new_direction(PlayerInput.RIGHT)

func _process(delta):
	time_acc += delta

	while time_acc >= TICK:
		snake.vel = input.direction()
		snake.pos += snake.vel
		time_acc -= TICK

	if snake.pos.x < 0:
		snake.pos.x = field_size.x - 1
	if snake.pos.y < 0:
		snake.pos.y = field_size.y - 1
	if snake.pos.x >= field_size.x:
		snake.pos.x = 0
	if snake.pos.y >= field_size.y:
		snake.pos.y = 0

	queue_redraw()

func _draw():
	draw_rect(
		Rect2(snake.pos * TILE_SIZE, TILE_SIZE),
		Color(1, 1, 1, 1),
		true,
	)
