extends Control

## Spawns holders into a cols x rows grid and tracks which cells are occupied.
## A holder is "landlocked" (not selectable) only when all four orthogonal
## neighbors are in-bounds AND occupied. Edge holders always have an open side.

signal holder_tapped(holder: Holder)

const HOLDER_SCENE := preload("res://scenes/holder.tscn")
const NEIGHBORS := [Vector2i.LEFT, Vector2i.RIGHT, Vector2i.UP, Vector2i.DOWN]

var cols: int = 4
var rows: int = 4
var cell: Vector2 = Vector2.ZERO
var grid_map: Dictionary = {}  # Vector2i -> Holder

func setup(level: LevelData) -> void:
	_clear()
	cols = level.grid_cols
	rows = level.grid_rows
	cell = Vector2(size.x / float(cols), size.y / float(rows))
	var pad := Vector2(12, 12)
	for h in level.holders:
		var gp := Vector2i(h["col"], h["row"])
		var holder: Holder = HOLDER_SCENE.instantiate()
		add_child(holder)
		holder.setup(h["color"], int(h["capacity"]), gp, cell - pad * 2.0)
		holder.position = cell_center(gp)
		holder.tapped.connect(_on_holder_tapped)
		grid_map[gp] = holder
	refresh_selectable()

func cell_center(gp: Vector2i) -> Vector2:
	return Vector2((gp.x + 0.5) * cell.x, (gp.y + 0.5) * cell.y)

func _clear() -> void:
	for h in grid_map.values():
		if is_instance_valid(h):
			h.queue_free()
	grid_map.clear()

func is_landlocked(pos: Vector2i) -> bool:
	for d in NEIGHBORS:
		var n: Vector2i = pos + d
		if n.x < 0 or n.x >= cols or n.y < 0 or n.y >= rows:
			return false
		if not grid_map.has(n):
			return false
	return true

func refresh_selectable() -> void:
	for gp in grid_map:
		var h: Holder = grid_map[gp]
		if not h.staged:
			h.set_selectable(not is_landlocked(gp))

func release_holder(holder: Holder) -> void:
	## Caller takes ownership of the holder node; grid just frees the cell.
	grid_map.erase(holder.grid_pos)
	refresh_selectable()

func is_empty() -> bool:
	return grid_map.is_empty()

func _on_holder_tapped(holder: Holder) -> void:
	holder_tapped.emit(holder)
