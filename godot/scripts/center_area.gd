extends Control

## Holds the drifting balls. Balls fly out to staged holders on demand and
## are replenished from the queue by the game orchestrator.

signal ball_consumed()

const BALL_SCENE := preload("res://scenes/ball.tscn")

var balls: Array = []

func add_ball(color: String) -> void:
	var b: Ball = BALL_SCENE.instantiate()
	add_child(b)
	b.setup(color, Rect2(Vector2.ZERO, size))
	b.position = Vector2(
		randf_range(b.radius, size.x - b.radius),
		randf_range(b.radius, size.y - b.radius))
	balls.append(b)

func has_color(color: String) -> bool:
	for b in balls:
		if b.color_name == color and not b.is_flying:
			return true
	return false

func count() -> int:
	return balls.size()

## Send one idle ball of `color` to `target_global`. Calls `on_arrive` when it
## lands, then frees the ball and emits ball_consumed. Returns false if none.
func match_one(color: String, target_global: Vector2, on_arrive: Callable) -> bool:
	for b in balls:
		if b.color_name == color and not b.is_flying:
			balls.erase(b)
			var ball: Ball = b
			ball.fly_to(target_global, func() -> void:
				on_arrive.call()
				ball.queue_free()
				ball_consumed.emit())
			return true
	return false
