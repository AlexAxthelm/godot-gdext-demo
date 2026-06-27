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

## Implementation Notes (deferred — design phase only)

- Godot 4, 2D
- gdext (Rust extension) may or may not be needed for this simpler game — TBD
- Ball drifting: simple 2D physics or manual idle animation
- Level data: likely defined in JSON or Godot Resources
