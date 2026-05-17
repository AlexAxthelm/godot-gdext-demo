use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

use crate::constants::*;
use crate::team::Team;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UnitState {
    Moving,   // heading toward destination
    Drifting, // arrived, gentle random wander around anchor
    Pursuing, // chasing a nearby enemy
    Dead,
}

#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct Unit {
    pub team: Team,
    pub state: UnitState,

    // where the player/AI told this unit to go
    pub destination: Vector2,
    // anchor point for drift (updated when we arrive)
    pub drift_anchor: Vector2,
    // current drift target (within DRIFT_RADIUS of anchor)
    pub drift_target: Vector2,
    pub drift_timer: f32,

    // pursuit target (NodePath would be fragile; use instance id)
    pub pursue_target_id: Option<i64>,

    pub dead: bool,

    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for Unit {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            team: Team::Neutral,
            state: UnitState::Drifting,
            destination: Vector2::ZERO,
            drift_anchor: Vector2::ZERO,
            drift_target: Vector2::ZERO,
            drift_timer: 0.0,
            pursue_target_id: None,
            dead: false,
            base,
        }
    }

    fn draw(&mut self) {
        if self.dead {
            return;
        }
        let color = self.team.color();
        // filled circle body
        self.base_mut()
            .draw_circle(Vector2::ZERO, UNIT_RADIUS, color);
        // darker outline
        let outline = Color::from_rgb(
            color.r * 0.6,
            color.g * 0.6,
            color.b * 0.6,
        );
        self.base_mut().draw_arc(
            Vector2::ZERO,
            UNIT_RADIUS,
            0.0,
            std::f32::consts::TAU,
            16,
            outline,
        );
    }
}

#[godot_api]
impl Unit {
    /// Called each frame by the Game scene. Returns true if the unit died.
    pub fn tick(&mut self, delta: f32, rng: &mut impl FnMut() -> f32) -> bool {
        if self.dead {
            return true;
        }

        let pos = self.base().get_position();

        match self.state {
            UnitState::Moving => {
                let diff = self.destination - pos;
                if diff.length() < UNIT_SPEED * delta {
                    let dest = self.destination;
                    self.base_mut().set_position(dest);
                    self.drift_anchor = self.destination;
                    self.drift_target = self.destination;
                    self.drift_timer = 0.0;
                    self.state = UnitState::Drifting;
                } else {
                    let step = diff.normalized() * UNIT_SPEED * delta;
                    self.base_mut().set_position(pos + step);
                }
            }
            UnitState::Drifting => {
                self.drift_timer -= delta;
                if self.drift_timer <= 0.0 {
                    // pick new drift target within DRIFT_RADIUS of anchor
                    let angle = rng() * std::f32::consts::TAU;
                    let dist  = rng() * DRIFT_RADIUS;
                    self.drift_target = self.drift_anchor
                        + Vector2::new(angle.cos() * dist, angle.sin() * dist);
                    self.drift_timer = DRIFT_CHANGE_INTERVAL;
                }
                let diff = self.drift_target - pos;
                if diff.length() > 1.0 {
                    let step = diff.normalized() * DRIFT_SPEED * delta;
                    self.base_mut().set_position(pos + step);
                }
            }
            UnitState::Pursuing => {
                // pursuit resolution is handled in Game where we have access
                // to all units; we just move here if the game set a destination
                let diff = self.destination - pos;
                if diff.length() > 1.0 {
                    let step = diff.normalized() * UNIT_SPEED * delta;
                    self.base_mut().set_position(pos + step);
                }
            }
            UnitState::Dead => {
                self.dead = true;
                return true;
            }
        }

        self.base_mut().queue_redraw();
        false
    }

    pub fn die(&mut self) {
        self.dead = true;
        self.state = UnitState::Dead;
    }

    pub fn position(&self) -> Vector2 {
        self.base().get_position()
    }

    pub fn set_pos(&mut self, p: Vector2) {
        self.base_mut().set_position(p);
    }

    pub fn send_to(&mut self, dest: Vector2) {
        self.destination = dest;
        self.state = UnitState::Moving;
        self.pursue_target_id = None;
    }
}
