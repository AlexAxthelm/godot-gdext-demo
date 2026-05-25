use godot::classes::{INode2D, InputEvent, InputEventMouseButton, InputEventMouseMotion, Label, Node2D};
use godot::global::MouseButton;
use godot::prelude::*;

use crate::constants::*;
use crate::map_node::MapNode;
use crate::team::Team;
use crate::unit::{Unit, UnitState};

// Simple deterministic LCG so we don't need the rand crate.
struct Lcg(u64);
impl Lcg {
    fn next_f32(&mut self) -> f32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 33) as f32) / (u32::MAX as f32)
    }
}

// Input state machine
#[derive(PartialEq)]
enum InputMode {
    Idle,
    PressDown { origin: Vector2 },
    Dragging { origin: Vector2, current: Vector2 },
}

#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct Game {
    rng: Lcg,

    nodes: Vec<Gd<MapNode>>,
    units: Vec<Gd<Unit>>,

    selected: Vec<usize>,
    input_mode: InputMode,

    ai_timer: f32,
    winner: Option<Team>,

    stats_label: Option<Gd<Label>>,

    base: Base<Node2D>,
}

// ---------------------------------------------------------------------------
// Layout helpers
// ---------------------------------------------------------------------------

fn node_positions() -> Vec<(Vector2, Team)> {
    let w = FIELD_W;
    let h = FIELD_H;
    vec![
        // Player side (left) — 4 nodes
        (Vector2::new(w * 0.10, h * 0.25), Team::Player),
        (Vector2::new(w * 0.10, h * 0.75), Team::Player),
        (Vector2::new(w * 0.25, h * 0.15), Team::Player),
        (Vector2::new(w * 0.25, h * 0.85), Team::Player),
        // Neutral center — 2 nodes
        (Vector2::new(w * 0.50, h * 0.33), Team::Neutral),
        (Vector2::new(w * 0.50, h * 0.67), Team::Neutral),
        // Enemy side (right) — 4 nodes
        (Vector2::new(w * 0.75, h * 0.15), Team::Enemy),
        (Vector2::new(w * 0.75, h * 0.85), Team::Enemy),
        (Vector2::new(w * 0.90, h * 0.25), Team::Enemy),
        (Vector2::new(w * 0.90, h * 0.75), Team::Enemy),
    ]
}

#[godot_api]
impl INode2D for Game {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            rng: Lcg(12345),
            nodes: Vec::new(),
            units: Vec::new(),
            selected: Vec::new(),
            input_mode: InputMode::Idle,
            ai_timer: 0.0,
            winner: None,
            stats_label: None,
            base,
        }
    }

    fn ready(&mut self) {
        self.setup_nodes();
        self.spawn_starting_units();

        // Stats overlay — top-left label, no font resource needed
        let mut label = Label::new_alloc();
        label.set_position(Vector2::new(8.0, 8.0));
        // Bright color so it's visible against the dark field
        label.add_theme_color_override(
            "font_color",
            Color::from_rgb(1.0, 1.0, 0.2),
        );
        self.base_mut().add_child(&label);
        self.stats_label = Some(label);
    }

    fn process(&mut self, delta: f64) {
        let delta = delta as f32;
        if self.winner.is_some() {
            return;
        }
        self.tick_units(delta);
        self.resolve_combat();
        self.tick_capture(delta);
        self.tick_spawns(delta);
        self.tick_ai(delta);
        self.check_win();
        self.update_stats(delta);
        self.base_mut().queue_redraw();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.winner.is_some() {
            return;
        }

        if let Ok(mb) = event.clone().try_cast::<InputEventMouseButton>() {
            self.handle_mouse_button(mb);
        } else if let Ok(mm) = event.try_cast::<InputEventMouseMotion>() {
            self.handle_mouse_motion(mm);
        }
    }

    fn draw(&mut self) {
        // Selection drag circle
        if let InputMode::Dragging { origin, current } = self.input_mode {
            let center = (origin + current) * 0.5;
            let radius = (current - origin).length() * 0.5;
            let drag_color = Color::from_rgba(1.0, 1.0, 1.0, 0.25);
            let drag_outline = Color::from_rgba(1.0, 1.0, 1.0, 0.8);
            self.base_mut()
                .draw_circle(center, radius, drag_color);
            self.base_mut().draw_arc(
                center,
                radius,
                0.0,
                std::f32::consts::TAU,
                32,
                drag_outline,
            );
        }

        // Selection highlight rings on selected units
        let selected_color = Color::from_rgba(1.0, 1.0, 0.2, 0.9);
        let selected_copy = self.selected.clone();
        for &idx in &selected_copy {
            if let Some(unit) = self.units.get(idx) {
                let u = unit.bind();
                if !u.dead {
                    let pos = u.position();
                    drop(u);
                    self.base_mut().draw_arc(
                        pos,
                        UNIT_RADIUS + 3.0,
                        0.0,
                        std::f32::consts::TAU,
                        16,
                        selected_color,
                    );
                }
            }
        }

        // Win banner
        if let Some(winner) = self.winner {
            let text = match winner {
                Team::Player => "You win!",
                Team::Enemy  => "Enemy wins!",
                Team::Neutral => "",
            };
            // Draw a simple background rect then text via draw_string would need
            // a font resource — keep it simple with a colored rect + label via
            // queue_redraw triggering node drawing. We'll just draw a colored bar.
            let bar_color = winner.color();
            self.base_mut().draw_rect(
                Rect2::new(
                    Vector2::new(FIELD_W * 0.3, FIELD_H * 0.45),
                    Vector2::new(FIELD_W * 0.4, FIELD_H * 0.1),
                ),
                bar_color,
            );
            let _ = text; // font drawing needs a FontFile resource; skip for now
        }
    }
}

// ---------------------------------------------------------------------------
// Setup
// ---------------------------------------------------------------------------

#[godot_api]
impl Game {
    fn update_stats(&mut self, delta: f32) {
        let fps = if delta > 0.0 { (1.0 / delta) as i32 } else { 0 };
        let player_units = self.units.iter().filter(|u| u.bind().team == Team::Player).count();
        let enemy_units  = self.units.iter().filter(|u| u.bind().team == Team::Enemy).count();
        let player_nodes = self.nodes.iter().filter(|n| n.bind().team == Team::Player).count();
        let enemy_nodes  = self.nodes.iter().filter(|n| n.bind().team == Team::Enemy).count();

        let text = format!(
            "FPS: {fps}\nPlayer  units: {player_units}  nodes: {player_nodes}\nEnemy   units: {enemy_units}  nodes: {enemy_nodes}"
        );

        if let Some(label) = &mut self.stats_label {
            label.set_text(text.as_str());
        }
    }
    fn setup_nodes(&mut self) {
        let positions = node_positions();
        let base = self.base().get_path();
        for (i, (pos, team)) in positions.into_iter().enumerate() {
            let mut mn = MapNode::new_alloc();
            {
                let mut b = mn.bind_mut();
                b.team = team;
                b.id = i;
            }
            mn.set_position(pos);
            // Add as child before storing
            self.base_mut().add_child(&mn);
            self.nodes.push(mn);
        }
        let _ = base;
    }

    fn spawn_starting_units(&mut self) {
        for team in [Team::Player, Team::Enemy] {
            // Find first node for this team
            let anchor = self
                .nodes
                .iter()
                .find(|n| n.bind().team == team)
                .map(|n| n.bind().position())
                .unwrap_or(Vector2::new(FIELD_W * 0.5, FIELD_H * 0.5));

            for _ in 0..STARTING_UNITS {
                let angle = self.rng.next_f32() * std::f32::consts::TAU;
                let dist  = self.rng.next_f32() * NODE_SELECT_RADIUS;
                let pos   = anchor + Vector2::new(angle.cos() * dist, angle.sin() * dist);
                self.spawn_unit(team, pos, anchor);
            }
        }
    }

    fn spawn_unit(&mut self, team: Team, pos: Vector2, drift_anchor: Vector2) {
        let mut unit = Unit::new_alloc();
        {
            let mut b = unit.bind_mut();
            b.team = team;
            b.state = UnitState::Drifting;
            b.drift_anchor = drift_anchor;
            b.drift_target = drift_anchor;
            b.drift_timer = self.rng.next_f32() * DRIFT_CHANGE_INTERVAL;
        }
        unit.set_position(pos);
        self.base_mut().add_child(&unit);
        self.units.push(unit);
    }

    // ---------------------------------------------------------------------------
    // Per-frame logic
    // ---------------------------------------------------------------------------

    fn tick_units(&mut self, delta: f32) {
        for unit in &mut self.units {
            let r1 = self.rng.next_f32();
            let r2 = self.rng.next_f32();
            let mut rng_pair = {
                let mut used = false;
                move || { if !used { used = true; r1 } else { r2 } }
            };
            let mut b = unit.bind_mut();
            b.tick(delta, &mut rng_pair);
        }
        // Remove dead units from scene
        let mut i = 0;
        while i < self.units.len() {
            if self.units[i].bind().dead {
                let mut u = self.units.remove(i);
                u.queue_free();
                self.selected.clear();
            } else {
                i += 1;
            }
        }
    }

    fn resolve_combat(&mut self) {
        // Find pairs of enemy units within ATTACK_RANGE and mark both dead.
        // O(n²) — fine for a few hundred units.
        let len = self.units.len();
        let mut dead_flags = vec![false; len];

        for i in 0..len {
            if dead_flags[i] {
                continue;
            }
            let (team_i, pos_i) = {
                let b = self.units[i].bind();
                (b.team, b.position())
            };

            for j in (i + 1)..len {
                if dead_flags[j] {
                    continue;
                }
                let (team_j, pos_j) = {
                    let b = self.units[j].bind();
                    (b.team, b.position())
                };

                if team_i.is_enemy_of(team_j)
                    && (pos_i - pos_j).length() < ATTACK_RANGE
                {
                    dead_flags[i] = true;
                    dead_flags[j] = true;
                    break; // unit i is resolved
                }
            }
        }

        for (i, dead) in dead_flags.iter().enumerate() {
            if *dead {
                self.units[i].bind_mut().die();
            }
        }

        // Also handle pursuit: units within PURSUE_RANGE steer toward nearest enemy
        let positions_teams: Vec<(Vector2, Team)> = self
            .units
            .iter()
            .map(|u| {
                let b = u.bind();
                (b.position(), b.team)
            })
            .collect();

        for i in 0..self.units.len() {
            let (pos_i, team_i) = positions_teams[i];
            let mut nearest_dist = PURSUE_RANGE;
            let mut nearest_pos = None;

            for (j, &(pos_j, team_j)) in positions_teams.iter().enumerate() {
                if i == j {
                    continue;
                }
                if team_i.is_enemy_of(team_j) {
                    let d = (pos_i - pos_j).length();
                    if d < nearest_dist {
                        nearest_dist = d;
                        nearest_pos = Some(pos_j);
                    }
                }
            }

            if let Some(target_pos) = nearest_pos {
                let mut b = self.units[i].bind_mut();
                // Only switch to pursuing if not already ordered somewhere
                if b.state == UnitState::Drifting {
                    b.destination = target_pos;
                    b.state = UnitState::Pursuing;
                } else if b.state == UnitState::Pursuing {
                    b.destination = target_pos;
                }
            } else {
                // No enemy in range — stop pursuing
                let mut b = self.units[i].bind_mut();
                if b.state == UnitState::Pursuing {
                    b.drift_anchor = b.position();
                    b.drift_target = b.position();
                    b.drift_timer = 0.0;
                    b.state = UnitState::Drifting;
                }
            }
        }
    }

    fn tick_capture(&mut self, delta: f32) {
        let unit_data: Vec<(Vector2, Team)> = self
            .units
            .iter()
            .map(|u| {
                let b = u.bind();
                (b.position(), b.team)
            })
            .collect();

        for node in &mut self.nodes {
            let node_pos = node.bind().position();
            let mut player_count = 0u32;
            let mut enemy_count  = 0u32;

            for (upos, uteam) in &unit_data {
                if (*upos - node_pos).length() < NODE_RADIUS {
                    match uteam {
                        Team::Player  => player_count += 1,
                        Team::Enemy   => enemy_count  += 1,
                        Team::Neutral => {}
                    }
                }
            }

            node.bind_mut()
                .tick_capture(delta, player_count, enemy_count);
        }
    }

    fn tick_spawns(&mut self, delta: f32) {
        let mut spawns: Vec<(Team, Vector2)> = Vec::new();

        for node in &mut self.nodes {
            let mut b = node.bind_mut();
            if b.tick_spawn(delta) {
                spawns.push((b.team, b.position()));
            }
        }

        let player_count = self.units.iter().filter(|u| u.bind().team == Team::Player).count() as u32;
        let enemy_count  = self.units.iter().filter(|u| u.bind().team == Team::Enemy).count() as u32;

        for (team, pos) in spawns {
            let count = if team == Team::Player { player_count } else { enemy_count };
            if count < MAX_UNITS_PER_TEAM {
                let angle  = self.rng.next_f32() * std::f32::consts::TAU;
                let offset = Vector2::new(angle.cos(), angle.sin()) * NODE_RADIUS;
                self.spawn_unit(team, pos + offset, pos);
            }
        }
    }

    fn tick_ai(&mut self, delta: f32) {
        self.ai_timer -= delta;
        if self.ai_timer > 0.0 {
            return;
        }
        self.ai_timer = 2.5; // reconsider every 2.5 seconds

        // Collect enemy node positions
        let enemy_node_positions: Vec<Vector2> = self
            .nodes
            .iter()
            .filter(|n| n.bind().team == Team::Enemy)
            .map(|n| n.bind().position())
            .collect();

        if enemy_node_positions.is_empty() {
            return;
        }

        // Send each idle enemy unit to a random enemy node
        let node_count = enemy_node_positions.len();
        for unit in &mut self.units {
            let mut b = unit.bind_mut();
            if b.team == Team::Enemy
                && (b.state == UnitState::Drifting || b.state == UnitState::Moving)
            {
                let idx = (self.rng.next_f32() * node_count as f32) as usize;
                let idx = idx.min(node_count - 1);
                let dest = enemy_node_positions[idx];
                let angle  = self.rng.next_f32() * std::f32::consts::TAU;
                let offset = Vector2::new(angle.cos(), angle.sin()) * NODE_RADIUS * 0.5;
                b.send_to(dest + offset);
            }
        }
    }

    fn check_win(&mut self) {
        if self.winner.is_some() {
            return;
        }
        let all_player = self.nodes.iter().all(|n| n.bind().team == Team::Player);
        let all_enemy  = self.nodes.iter().all(|n| n.bind().team == Team::Enemy);
        if all_player {
            self.winner = Some(Team::Player);
        } else if all_enemy {
            self.winner = Some(Team::Enemy);
        }
    }

    // ---------------------------------------------------------------------------
    // Input
    // ---------------------------------------------------------------------------

    fn handle_mouse_button(&mut self, mb: Gd<InputEventMouseButton>) {
        let pos = mb.get_position();

        if mb.get_button_index() == MouseButton::LEFT {
            if mb.is_pressed() {
                self.input_mode = InputMode::PressDown { origin: pos };
            } else {
                // Released
                match self.input_mode {
                    InputMode::PressDown { origin } => {
                        // Short tap
                        self.handle_tap(origin);
                    }
                    InputMode::Dragging { origin, current } => {
                        let center = (origin + current) * 0.5;
                        let radius = (current - origin).length() * 0.5;
                        self.select_units_in_circle(center, radius);
                    }
                    InputMode::Idle => {}
                }
                self.input_mode = InputMode::Idle;
            }
        }
    }

    fn handle_mouse_motion(&mut self, mm: Gd<InputEventMouseMotion>) {
        if let InputMode::PressDown { origin } = self.input_mode {
            let current = mm.get_position();
            if (current - origin).length() > DRAG_THRESHOLD {
                self.input_mode = InputMode::Dragging { origin, current };
            }
        } else if let InputMode::Dragging { origin, .. } = self.input_mode {
            self.input_mode = InputMode::Dragging {
                origin,
                current: mm.get_position(),
            };
        }
    }

    fn handle_tap(&mut self, pos: Vector2) {
        if !self.selected.is_empty() {
            // Units selected — send them to the tap position
            self.send_selected(pos);
            self.selected.clear();
            return;
        }

        // Nothing selected — try to select near a node first
        let mut best_dist = NODE_SELECT_RADIUS;
        let mut best_node_pos = None;
        for node in &self.nodes {
            let b = node.bind();
            if b.team == Team::Player {
                let d = (b.position() - pos).length();
                if d < best_dist {
                    best_dist = d;
                    best_node_pos = Some(b.position());
                }
            }
        }

        if let Some(node_pos) = best_node_pos {
            self.select_units_in_circle(node_pos, NODE_SELECT_RADIUS);
        }
        // If nothing near a node, tap does nothing (could add point-select later)
    }

    fn send_selected(&mut self, dest: Vector2) {
        // Spread units in a small cluster around the destination
        let count = self.selected.len();
        for (i, &idx) in self.selected.iter().enumerate() {
            if let Some(unit) = self.units.get_mut(idx) {
                let angle  = (i as f32 / count as f32) * std::f32::consts::TAU;
                let spread = if count == 1 { 0.0 } else { UNIT_RADIUS * 3.0 };
                let offset = Vector2::new(angle.cos() * spread, angle.sin() * spread);
                unit.bind_mut().send_to(dest + offset);
            }
        }
    }

    fn select_units_in_circle(&mut self, center: Vector2, radius: f32) {
        self.selected.clear();
        for (i, unit) in self.units.iter().enumerate() {
            let b = unit.bind();
            if b.team == Team::Player
                && !b.dead
                && (b.position() - center).length() < radius
            {
                self.selected.push(i);
            }
        }
    }
}
