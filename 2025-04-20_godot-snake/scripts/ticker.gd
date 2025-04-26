extends RefCounted
class_name Ticker

const TICK = 0.1

var _accumulated_time = 0

func on_update(delta):
	_accumulated_time += delta

func tick():
	var should_tick = _accumulated_time >= TICK
	
	if should_tick:
		_accumulated_time -= TICK

	return should_tick
