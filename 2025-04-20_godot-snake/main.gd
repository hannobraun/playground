extends Node2D

const TILE_SIZE = Vector2(16, 16)
const TICK = 0.1

var field_size

var input = PlayerInput.new()

var pos = Vector2(0, 0)
var vel = input.direction

var time_acc = 0

func _ready():
	field_size = get_viewport().get_visible_rect().size / TILE_SIZE
	pos = field_size / 2

func _input(event: InputEvent):
	if event is InputEventKey and event.is_pressed():
		var new_direction

		if event.keycode == KEY_UP:
			new_direction = PlayerInput.UP
		if event.keycode == KEY_LEFT:
			new_direction = PlayerInput.LEFT
		if event.keycode == KEY_DOWN:
			new_direction = PlayerInput.DOWN
		if event.keycode == KEY_RIGHT:
			new_direction = PlayerInput.RIGHT
		
		if new_direction != null:
			var opposite = new_direction * -1
			
			var not_reversing_direction
			var not_duplicating_event
			
			if input.events.is_empty():
				not_reversing_direction = vel != opposite
				not_duplicating_event = vel != new_direction
			else:
				not_reversing_direction = input.events.back() != opposite
				not_duplicating_event = not input.events.has(new_direction)
			
			if not_reversing_direction and not_duplicating_event:
				input.events.push_back(new_direction)

func _process(delta):
	time_acc += delta

	while time_acc >= TICK:
		pos += vel
		
		if not input.events.is_empty():
			vel = input.events.pop_front()

		time_acc -= TICK

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
