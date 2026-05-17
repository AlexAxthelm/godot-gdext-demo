use godot::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Team {
    Player,
    Enemy,
    Neutral,
}

impl Team {
    pub fn color(&self) -> Color {
        match self {
            Team::Player  => Color::from_rgb(0.2, 0.5, 1.0),
            Team::Enemy   => Color::from_rgb(1.0, 0.3, 0.2),
            Team::Neutral => Color::from_rgb(0.6, 0.6, 0.6),
        }
    }

    pub fn is_enemy_of(&self, other: Team) -> bool {
        match (self, other) {
            (Team::Neutral, _) | (_, Team::Neutral) => false,
            (a, b) => a != &b,
        }
    }
}
