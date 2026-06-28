extends Control

## Holds active holders in a vertical stack of slots. A selected holder
## travels here, fills with matching balls, then slides off-screen when full.

signal holder_filled(holder: Holder)
signal room_opened()

var slot_count: int = 3
var slots: Array = []  # each: { "holder": Holder|null, "center": Vector2 (local) }

func setup(level: LevelData) -> void:
	slot_count = level.staging_slots
	slots.clear()
	var slot_h := size.y / float(slot_count)
	for i in slot_count:
		slots.append({
			"holder": null,
			"center": Vector2(size.x * 0.5, (i + 0.5) * slot_h),
		})

func has_room() -> bool:
	return _free_slot() != -1

func active_holders() -> Array:
	var out := []
	for s in slots:
		if s["holder"] != null:
			out.append(s["holder"])
	return out

func stage_holder(holder: Holder) -> bool:
	var idx := _free_slot()
	if idx == -1:
		return false
	slots[idx]["holder"] = holder
	holder.staged = true
	holder.set_selectable(false)
	holder.hitbox.input_pickable = false
	holder.complete.connect(_on_holder_complete.bind(idx), CONNECT_ONE_SHOT)
	holder.reparent(self, true)  # keep global transform across reparent
	var sf := _scale_for(holder)
	var tw := create_tween().set_parallel(true)
	tw.tween_property(holder, "position", slots[idx]["center"], 0.35) \
		.set_trans(Tween.TRANS_CUBIC).set_ease(Tween.EASE_OUT)
	tw.tween_property(holder, "scale", Vector2(sf, sf), 0.35) \
		.set_trans(Tween.TRANS_CUBIC).set_ease(Tween.EASE_OUT)
	return true

func _scale_for(holder: Holder) -> float:
	var slot_w := size.x * 0.85
	var slot_h := (size.y / float(slot_count)) * 0.85
	return minf(slot_w / holder.box_size.x, slot_h / holder.box_size.y)

func _free_slot() -> int:
	for i in slots.size():
		if slots[i]["holder"] == null:
			return i
	return -1

func _on_holder_complete(holder: Holder, idx: int) -> void:
	slots[idx]["holder"] = null
	var tw := create_tween()
	tw.tween_property(holder, "position:x", size.x + holder.box_size.x * 2.0, 0.3) \
		.set_trans(Tween.TRANS_CUBIC).set_ease(Tween.EASE_IN)
	tw.tween_callback(holder.queue_free)
	holder_filled.emit(holder)
	room_opened.emit()
