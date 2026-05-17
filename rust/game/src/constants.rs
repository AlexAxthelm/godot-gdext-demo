// ---------------------------------------------------------------------------
// Field
// ---------------------------------------------------------------------------
pub const FIELD_W: f32 = 1280.0;
pub const FIELD_H: f32 = 720.0;

// ---------------------------------------------------------------------------
// Nodes (capturable points)
// ---------------------------------------------------------------------------
pub const NODE_RADIUS: f32 = 32.0;       // visual + capture area radius
pub const CAPTURE_RATE: f32 = 20.0;      // progress per second per unit inside
pub const CAPTURE_MAX: f32 = 100.0;      // progress needed to flip ownership
pub const NODE_SELECT_RADIUS: f32 = 80.0; // tap-near-node selects units within this

// ---------------------------------------------------------------------------
// Units
// ---------------------------------------------------------------------------
pub const UNIT_RADIUS: f32 = 6.0;
pub const UNIT_SPEED: f32 = 90.0;        // px/s toward destination
pub const DRIFT_SPEED: f32 = 18.0;       // px/s gentle drift
pub const DRIFT_CHANGE_INTERVAL: f32 = 0.2; // seconds between drift direction changes
pub const DRIFT_RADIUS: f32 = 40.0;      // max distance from anchor before drifting back

pub const ATTACK_RANGE: f32 = 14.0;      // units fight when closer than this
pub const PURSUE_RANGE: f32 = 60.0;      // units notice enemies within this range

// ---------------------------------------------------------------------------
// Production
// ---------------------------------------------------------------------------
pub const SPAWN_INTERVAL: f32 = 3.0;     // seconds between unit spawns at a captured node
pub const MAX_UNITS_PER_TEAM: u32 = 2000;

// ---------------------------------------------------------------------------
// Starting conditions
// ---------------------------------------------------------------------------
pub const STARTING_UNITS: u32 = 120;

// ---------------------------------------------------------------------------
// Input
// ---------------------------------------------------------------------------
pub const DRAG_THRESHOLD: f32 = 12.0;   // px before a press becomes a drag
