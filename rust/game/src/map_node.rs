use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

use crate::constants::*;
use crate::team::Team;

#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct MapNode {
    pub team: Team,
    /// Capture progress toward the current contested team.
    /// Positive = player progress, negative = enemy progress.
    /// Resets to 0 when ownership flips.
    pub capture_progress: f32,
    pub spawn_timer: f32,
    pub id: usize,

    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for MapNode {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            team: Team::Neutral,
            capture_progress: 0.0,
            spawn_timer: 0.0,
            id: 0,
            base,
        }
    }

    fn draw(&mut self) {
        let color = self.team.color();

        // outer ring
        self.base_mut().draw_arc(
            Vector2::ZERO,
            NODE_RADIUS,
            0.0,
            std::f32::consts::TAU,
            32,
            color,
        );

        // capture progress arc (filled sweep)
        if self.capture_progress.abs() > 0.5 {
            let frac = (self.capture_progress / CAPTURE_MAX).clamp(-1.0, 1.0);
            let arc_color = if frac > 0.0 {
                Team::Player.color()
            } else {
                Team::Enemy.color()
            };
            let sweep = frac.abs() * std::f32::consts::TAU;
            self.base_mut().draw_arc(
                Vector2::ZERO,
                NODE_RADIUS - 5.0,
                -std::f32::consts::FRAC_PI_2,
                -std::f32::consts::FRAC_PI_2 + sweep,
                32,
                arc_color,
            );
        }

        // center dot
        self.base_mut()
            .draw_circle(Vector2::ZERO, 6.0, color);
    }
}

#[godot_api]
impl MapNode {
    /// Returns Some(team) if ownership just changed this tick.
    pub fn tick_capture(
        &mut self,
        delta: f32,
        player_units_inside: u32,
        enemy_units_inside: u32,
    ) -> Option<Team> {
        let net = player_units_inside as f32 - enemy_units_inside as f32;
        if net == 0.0 {
            return None;
        }

        self.capture_progress += net * CAPTURE_RATE * delta;
        self.capture_progress = self.capture_progress.clamp(-CAPTURE_MAX, CAPTURE_MAX);

        self.base_mut().queue_redraw();

        if self.capture_progress >= CAPTURE_MAX && self.team != Team::Player {
            self.team = Team::Player;
            self.capture_progress = 0.0;
            return Some(Team::Player);
        }
        if self.capture_progress <= -CAPTURE_MAX && self.team != Team::Enemy {
            self.team = Team::Enemy;
            self.capture_progress = 0.0;
            return Some(Team::Enemy);
        }

        None
    }

    /// Returns true if a new unit should be spawned this tick.
    pub fn tick_spawn(&mut self, delta: f32) -> bool {
        if self.team == Team::Neutral {
            return false;
        }
        self.spawn_timer += delta;
        if self.spawn_timer >= SPAWN_INTERVAL {
            self.spawn_timer -= SPAWN_INTERVAL;
            return true;
        }
        false
    }

    pub fn position(&self) -> Vector2 {
        self.base().get_position()
    }
}
