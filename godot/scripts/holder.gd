class_name Holder
extends Node2D

## A colored holder with `capacity` slots. Lives in the grid until tapped,
## then moves to staging and fills with matching balls until complete.

signal tapped(holder: Holder)
signal complete(holder: Holder)

var color_name: String = "red"
var capacity: int = 3
var filled: int = 0
var grid_pos: Vector2i = Vector2i.ZERO
var box_size: Vector2 = Vector2(100, 100)
var selectable: bool = false
var staged: bool = false
var in_flight: bool = false  ## a ball is currently flying toward this holder

@onready var hitbox: Area2D = $Hitbox
@onready var shape: CollisionShape2D = $Hitbox/Shape

func _ready() -> void:
	hitbox.input_event.connect(_on_input_event)

func setup(p_color: String, p_capacity: int, p_grid_pos: Vector2i, p_size: Vector2) -> void:
	color_name = p_color
	capacity = p_capacity
	grid_pos = p_grid_pos
	box_size = p_size
	var rect := RectangleShape2D.new()
	rect.size = box_size
	shape.shape = rect
	queue_redraw()

func set_selectable(v: bool) -> void:
	if selectable != v:
		selectable = v
		queue_redraw()

func receive_ball() -> void:
	if filled >= capacity:
		return
	filled += 1
	queue_redraw()
	if filled >= capacity:
		complete.emit(self)

func remaining() -> int:
	return capacity - filled

func is_full() -> bool:
	return filled >= capacity

func _on_input_event(_viewport: Node, event: InputEvent, _shape_idx: int) -> void:
	if staged or not selectable:
		return
	if event is InputEventMouseButton and event.pressed and event.button_index == MOUSE_BUTTON_LEFT:
		tapped.emit(self)

func _draw() -> void:
	var col := Palette.get_color(color_name)
	if not selectable and not staged:
		col = col.darkened(0.55)
	var half := box_size * 0.5
	var rect := Rect2(-half, box_size)
	draw_rect(rect, col, true)
	draw_rect(rect, Color(0, 0, 0, 0.4), false, 2.0)
	# slot pips along the bottom: filled = solid, empty = outline
	var pip_r := 6.0
	var gap := pip_r * 2.6
	var total_w := gap * float(capacity - 1)
	var start_x := -total_w * 0.5
	var y := half.y - pip_r - 8.0
	for i in capacity:
		var c := Vector2(start_x + gap * float(i), y)
		if i < filled:
			draw_circle(c, pip_r, Color.WHITE)
		else:
			draw_arc(c, pip_r, 0.0, TAU, 16, Color(1, 1, 1, 0.5), 2.0)
