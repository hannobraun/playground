extends RefCounted
class_name Ticker

const TICK = 0.1

var time_acc = 0

func on_update(delta):
	time_acc += delta
