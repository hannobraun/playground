extends Node2D

var field
var input
var snake

var ticker = Ticker.new()

func _ready():
	field = Field.new(get_viewport().get_visible_rect().size)
	input = PlayerInput.new()
	snake = Snake.new(field, input)

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
	ticker.on_update(delta)

	while ticker.tick():
		snake.update(field, input)

	queue_redraw()

func _draw():
	draw_rect(
		Rect2(snake.position * field.TILE_SIZE, field.TILE_SIZE),
		Color(1, 1, 1, 1),
		true,
	)
