class_name Ball
extends Node2D

## A colored ball that drifts within the center area, or flies to a holder.

const MAX_SPEED := 55.0

var color_name: String = "red"
var velocity: Vector2 = Vector2.ZERO
var bounds: Rect2 = Rect2()
var radius: float = 12.0
var is_flying: bool = false
var _nudge_t: float = 0.0

func setup(p_color: String, p_bounds: Rect2) -> void:
	color_name = p_color
	bounds = p_bounds
	velocity = Vector2(randf_range(-30, 30), randf_range(-30, 30))
	_nudge_t = randf_range(3.0, 5.0)
	queue_redraw()

func _process(delta: float) -> void:
	if is_flying:
		return
	position += velocity * delta
	var minp := bounds.position + Vector2(radius, radius)
	var maxp := bounds.position + bounds.size - Vector2(radius, radius)
	if position.x < minp.x:
		position.x = minp.x
		velocity.x = absf(velocity.x)
	elif position.x > maxp.x:
		position.x = maxp.x
		velocity.x = -absf(velocity.x)
	if position.y < minp.y:
		position.y = minp.y
		velocity.y = absf(velocity.y)
	elif position.y > maxp.y:
		position.y = maxp.y
		velocity.y = -absf(velocity.y)
	_nudge_t -= delta
	if _nudge_t <= 0.0:
		_nudge_t = randf_range(3.0, 5.0)
		velocity += Vector2(randf_range(-15, 15), randf_range(-15, 15))
		velocity = velocity.limit_length(MAX_SPEED)

func fly_to(target_global: Vector2, on_done: Callable) -> void:
	is_flying = true
	z_index = 100
	var tw := create_tween()
	tw.tween_property(self, "global_position", target_global, 0.4) \
		.set_trans(Tween.TRANS_BACK).set_ease(Tween.EASE_IN)
	tw.tween_callback(on_done)

func _draw() -> void:
	draw_circle(Vector2.ZERO, radius, Palette.get_color(color_name))
	draw_arc(Vector2.ZERO, radius, 0.0, TAU, 24, Color(0, 0, 0, 0.25), 1.5)
