extends Control

## Preview strip: shows the next few upcoming balls (top = next up).
## Read-only — the player cannot reorder it.

const SHOWN := 6

var preview: Array = []

func show_queue(full_queue: Array) -> void:
	preview = full_queue.slice(0, SHOWN)
	queue_redraw()

func _draw() -> void:
	draw_rect(Rect2(Vector2.ZERO, size), Color(0.15, 0.15, 0.25, 1), true)
	var r := 14.0
	var gap := size.y / float(SHOWN)
	for i in preview.size():
		var c := Vector2(size.x * 0.5, gap * (float(i) + 0.5))
		var col: Color = Palette.get_color(preview[i])
		if i > 0:
			col.a = 0.85  # only the next-up ball is full strength
		draw_circle(c, r, col)
		draw_arc(c, r, 0.0, TAU, 24, Color(0, 0, 0, 0.25), 1.5)
