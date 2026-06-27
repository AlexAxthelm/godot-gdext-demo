# New Game Design: Ball-Sorting Puzzle

## Context

The current Godot/gdext demo has performance issues. Rather than fix them, we're starting fresh with a simpler, well-defined puzzle game in 2D.

---

## Core Concept

A turn-based color-sorting puzzle. Players clear a board by strategically selecting **holders** to collect colored balls from a shared center area. The challenge is choosing the right holder at the right time given a fixed, preview-able ball queue.

---

## Game Elements

### Center Area
- ~20 balls drifting loosely (soft-body / gentle physics, 2D)
- As balls are removed, new ones flow in from the queue to maintain the count
- Balls are colored; colors match holder colors

### Ball Queue
- Displayed on one side of the screen (e.g., right side)
- Fixed order per level (replaying the level = same sequence)
- Designed with "runs" of the same color (tunable per level) to reward planning
- Acts purely as a **preview** — player cannot reorder it

### Holder Grid
- Holders arranged on a grid (left/bottom side of screen, or top — TBD)
- Each holder: a rectangle with N color-coded slots (N is the holder's capacity)
- Each holder has **one color** — it only accepts balls of that color
- A holder is **available** only if it is not landlocked (has at least one free orthogonal neighbor or is on the edge of the grid)
- Selecting a landlocked holder is not possible

### Staging Area
- A limited number of slots (e.g., 3) between the holder grid and the center
- When a holder is selected, it animates from the grid to an open staging slot
- While staged, matching colored balls in the center automatically fly to it
- When all slots on the holder are filled → it animates off-screen (cleared)
- If no staging slots are free, the player cannot select more holders

---

## Player Actions

1. **Select a holder** from the grid (must be non-landlocked, staging slot must be free)
2. Watch it animate to staging
3. Balls of matching color auto-collect onto it over time
4. When full → holder auto-departs

That's it — the only input is "which holder to select next."

---

## Challenge / Fail Condition

Players can get **stuck** if:
- All staging slots are occupied by partially-filled holders
- The current queue doesn't have the right colors to fill them soon
- The remaining holders in the grid are all landlocked (can't be selected)

A reset/restart option is needed. Optionally: an "undo last selection" feature.

---

## Win Condition

All balls from the queue have been placed onto holders and departed. The ball queue is empty, the center is empty, all holders are cleared.

---

## Level Structure

Each level defines:
- Ball queue (color sequence, length)
- Holder grid layout (positions, colors, slot counts)
- Number of staging slots
- Center area ball count

Total balls across all holder capacities = total balls in queue (exact match).

---

## Layout (2D, portrait)

```
┌─────────────────────────────────┐
│  Queue   │   Center balls  │ Stage │  ← top ~1/3
│ (5-6 vis)│                 │       │
├──────────┴─────────────────┴───────┤
│                                     │
│          Holder Grid                │  ← bottom ~2/3
│                                     │
└─────────────────────────────────────┘
```

- **Queue** (left of center): vertical strip, shows next ~5–6 balls, scrolls as balls are consumed
- **Center ball area** (top center): balls drift around here
- **Staging area** (right of center): vertical strip with 3 slots (level-defined, default 3) for active holders
- **Holder grid** (bottom): grid of all holders for the level

---

## Resolved Design Decisions

- **Layout**: Portrait
- **Undo**: None — only recovery is full level restart
- **Staging slot count**: Level-defined, default 3
- **Visual style**: Minimal/geometric for now — polish deferred
- **Holder grid**: Default 4×4 (16 holders)
- **Slot count per holder**: Varies per holder (level-defined)
- **Stuck state**: Passive — always-visible restart button; no active detection

---

---

## Implementation Plan

### What to Delete

- `rust/game/src/game.rs`, `unit.rs`, `map_node.rs`, `team.rs`, `constants.rs`
- `godot/scenes/hello.tscn`
- `godot/scenes/game.tscn` (recreate from scratch)

### What to Modify

- `rust/game/src/lib.rs` — remove the 5 `mod` declarations, keep the `#[gdextension]` boilerplate (~6 lines). No new Rust code — GDScript handles everything.
- `godot/project.godot` — change viewport to 540×960 portrait, add stretch mode `canvas_items` / aspect `keep`

### Scene Hierarchy

```
game.tscn
├── TopSection (Control)
│   ├── BallQueue (Control)           [ball_queue.gd]
│   │   └── QueueContainer (VBoxContainer)
│   ├── CenterArea (Node2D)           [center_area.gd]
│   │   └── (Ball nodes spawned at runtime)
│   └── StagingArea (Control)         [staging_area.gd]
│       └── SlotsContainer (VBoxContainer)
└── HolderGrid (Control)              [holder_grid.gd]
    └── GridContainer (4 columns)
        └── (HolderCell nodes, spawned at runtime)

ball.tscn — Ball (Node2D) [ball.gd] + ColorRect
holder.tscn — Holder (Node2D) [holder.gd] + ColorRect + slot indicators
```

### Scripts & Responsibilities

| Script | Key Job |
|---|---|
| `game.gd` | Orchestrates everything; routes signals; win detection |
| `level_data.gd` | Resource subclass: `ball_queue[]`, `holders[]`, `staging_slots`, `center_ball_count` |
| `holder_grid.gd` | Spawns holders; 4×4 grid state; landlocked detection (4-neighbor check); signals `holder_tapped` |
| `holder.gd` | Stores color/capacity/filled; emits `tapped` and `complete` |
| `staging_area.gd` | Manages 3 slots; Tweens holder in; on `complete` Tweens holder off-screen; emits `holder_filled` |
| `center_area.gd` | Drifting balls in bounds; `match_color()` flies a ball to holder; emits `ball_consumed` |
| `ball_queue.gd` | Visual preview strip; `consume_front()` shifts array and redraws |
| `ball.gd` | Drift physics in `_process`; `fly_to(pos, callable)` suspends drift and Tweens |

### Ball Drift Physics

Manual `_process` update — no physics engine:
- Init with random velocity (~30px/s)
- Bounce off CenterArea bounds
- Random nudge every 3-5s, clamped to ~50px/s
- `fly_to()` sets `is_flying = true` and runs a Tween

### Level Data Format

Godot Resources (`.tres`) — type-safe, no parsing code:
```
res://scenes/level_data/level_01.tres
  ball_queue: Array[String]  # ["red", "blue", ...]
  holders: Array[Dictionary] # [{pos, color, capacity}]
  staging_slots: int
  center_ball_count: int
```

### Animation

All animations use `create_tween()` (no AnimationPlayer):
- Holder grid→staging: `TRANS_CUBIC`, `EASE_OUT`, 0.35s
- Ball center→holder: `TRANS_BACK`, `EASE_IN`, 0.4s
- Holder staging→off-screen: slide right, 0.3s, then `queue_free()`

### Build Order

1. Strip `lib.rs`, update `project.godot`, create stub `game.tscn` → CI green
2. Add `LevelData` resource + `level_01.tres`, build scene tree layout
3. `holder_grid.gd`: spawn grid from level data, tap detection, landlocked highlight
4. `staging_area.gd` + `holder.gd`: holder tap → animate to staging → fill → clear
5. `ball.gd` + `center_area.gd`: drift balls, fly to holder on match
6. `ball_queue.gd`: preview strip
7. Wire `game.gd` signals end-to-end; add restart button; win detection

### Verification

- `make run` (Godot headless smoke test) must stay green after each phase
- Play through `level_01` start to finish: select holders, watch balls fill, win screen
