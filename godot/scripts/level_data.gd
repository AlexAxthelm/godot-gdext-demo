class_name LevelData
extends Resource

## Defines one playable level.
## - ball_queue: full ordered list of every ball to be placed (color names).
##   The first `center_ball_count` are drawn into the center at start;
##   the rest stream in as balls are consumed. Total length MUST equal the
##   sum of all holder capacities, or the level is unsolvable.
## - holders: each entry { "col": int, "row": int, "color": String, "capacity": int }
## - staging_slots: how many holders can be active at once.
## - center_ball_count: how many balls drift in the center at a time.

@export var ball_queue: Array[String] = []
@export var holders: Array[Dictionary] = []
@export var staging_slots: int = 3
@export var center_ball_count: int = 20
@export var grid_cols: int = 4
@export var grid_rows: int = 4
