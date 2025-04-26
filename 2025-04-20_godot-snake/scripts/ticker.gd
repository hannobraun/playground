extends RefCounted
class_name Ticker

const TICK = 0.1

var time_acc = 0

func on_update(delta):
	time_acc += delta

func tick():
	var should_tick = time_acc >= TICK
	
	if should_tick:
		time_acc -= TICK

	return should_tick
