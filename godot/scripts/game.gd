extends Node2D

const LEVEL: LevelData = preload("res://scenes/level_data/level_01.tres")

@onready var holder_grid: Control = $HolderGrid
@onready var staging: Control = $TopSection/StagingArea
@onready var center: Control = $TopSection/CenterArea
@onready var ball_queue: Control = $TopSection/BallQueue
@onready var restart_button: Button = $TopBar/RestartButton
@onready var win_label: Label = $TopBar/WinLabel

var queue: Array = []  ## balls not yet drawn into the center
var won: bool = false

func _ready() -> void:
	holder_grid.setup(LEVEL)
	staging.setup(LEVEL)
	queue = LEVEL.ball_queue.duplicate()
	for i in mini(LEVEL.center_ball_count, queue.size()):
		center.add_ball(queue.pop_front())
	ball_queue.show_queue(queue)
	holder_grid.holder_tapped.connect(_on_holder_tapped)
	staging.holder_filled.connect(_on_holder_filled)
	center.ball_consumed.connect(_on_ball_consumed)
	restart_button.pressed.connect(_on_restart)

func _on_holder_tapped(holder: Holder) -> void:
	if won or not staging.has_room():
		return
	holder_grid.release_holder(holder)
	staging.stage_holder(holder)
	_pump()

func _on_ball_consumed() -> void:
	if not queue.is_empty():
		center.add_ball(queue.pop_front())
		ball_queue.show_queue(queue)
	_pump()

func _on_holder_filled(_holder: Holder) -> void:
	_pump()
	_check_win()

func _on_restart() -> void:
	get_tree().reload_current_scene()

## Dispatch one matching ball to each staged holder that needs one.
func _pump() -> void:
	for holder in staging.active_holders():
		if holder.in_flight or holder.is_full():
			continue
		if center.has_color(holder.color_name):
			holder.in_flight = true
			center.match_one(holder.color_name, holder.global_position, func() -> void:
				if is_instance_valid(holder):
					holder.in_flight = false
					holder.receive_ball())

func _check_win() -> void:
	if won:
		return
	if queue.is_empty() and center.count() == 0 \
			and staging.active_holders().is_empty() and holder_grid.is_empty():
		won = true
		win_label.text = "You win!"
		win_label.visible = true
