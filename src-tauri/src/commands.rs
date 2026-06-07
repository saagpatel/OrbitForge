use crate::galaxy;
use crate::gpu_gravity::GpuGravity;
use crate::physics::{BodyType, CelestialBody, Vec3};
use crate::procedural;
use crate::scenarios;
use crate::simulation::SimulationState;
use serde::Deserialize;
use std::sync::{Arc, Mutex, MutexGuard};
use tauri::State;

pub type SimState = Arc<Mutex<SimulationState>>;
type CommandResult<T> = Result<T, String>;
const MAX_VELOCITY_COMPONENT: f64 = 100_000.0;
const MAX_THRUST_COMPONENT: f64 = 100_000.0;
const MAX_PLANETS: u32 = 200;

#[derive(Deserialize)]
pub struct BodyData {
    pub x: f64,
    pub y: f64,
    #[serde(default)]
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    #[serde(default)]
    pub vz: f64,
    pub mass: f64,
    pub radius: f64,
    pub color: String,
    pub name: String,
    pub is_fixed: bool,
    #[serde(default)]
    pub body_type: BodyType,
}

#[derive(Deserialize)]
pub struct BodyUpdate {
    pub mass: Option<f64>,
    pub radius: Option<f64>,
    pub color: Option<String>,
    pub name: Option<String>,
    pub is_fixed: Option<bool>,
}

fn lock_sim<'a>(state: &'a State<SimState>) -> CommandResult<MutexGuard<'a, SimulationState>> {
    state
        .lock()
        .map_err(|_| "simulation state lock is poisoned".to_string())
}

fn require_finite(name: &str, value: f64) -> CommandResult<()> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(format!("{name} must be a finite number"))
    }
}

fn require_abs_max(name: &str, value: f64, max_abs: f64) -> CommandResult<()> {
    if value.abs() <= max_abs {
        Ok(())
    } else {
        Err(format!("{name} magnitude must be <= {max_abs}"))
    }
}

fn apply_scenario(sim: &mut SimulationState, name: &str) -> CommandResult<()> {
    match name {
        "sun_earth" => scenarios::load_sun_earth(sim),
        "inner_solar" => scenarios::load_inner_solar(sim),
        "outer_solar" => scenarios::load_outer_solar(sim),
        "full_solar" => scenarios::load_full_solar(sim),
        "binary_star" => scenarios::load_binary_star(sim),
        "figure_eight" => scenarios::load_figure_eight(sim),
        "inclined_solar" => scenarios::load_inclined_solar(sim),
        "asteroid_belt" => scenarios::load_solar_with_belt(sim),
        "galaxy_collision" => galaxy::generate_collision(sim, 300),
        _ => return Err(format!("unknown scenario: {name}")),
    }
    Ok(())
}

fn finalize_imported_state(
    new_state: &mut SimulationState,
    gpu: Option<Arc<GpuGravity>>,
) -> CommandResult<()> {
    let max_id = new_state.bodies.iter().map(|b| b.id).max().unwrap_or(0);
    if new_state.next_id <= max_id {
        new_state.next_id = max_id + 1;
    }
    new_state.validate()?;
    new_state.gpu = gpu;
    new_state.prime_accelerations();
    Ok(())
}

impl BodyData {
    fn validate(&self) -> CommandResult<()> {
        require_finite("x", self.x)?;
        require_finite("y", self.y)?;
        require_finite("z", self.z)?;
        require_finite("vx", self.vx)?;
        require_finite("vy", self.vy)?;
        require_finite("vz", self.vz)?;
        require_finite("mass", self.mass)?;
        require_finite("radius", self.radius)?;
        if self.mass <= 0.0 {
            return Err("mass must be > 0".to_string());
        }
        if self.radius <= 0.0 {
            return Err("radius must be > 0".to_string());
        }
        Ok(())
    }
}

impl BodyUpdate {
    fn validate(&self) -> CommandResult<()> {
        if let Some(mass) = self.mass {
            require_finite("mass", mass)?;
            if mass <= 0.0 {
                return Err("mass must be > 0".to_string());
            }
        }
        if let Some(radius) = self.radius {
            require_finite("radius", radius)?;
            if radius <= 0.0 {
                return Err("radius must be > 0".to_string());
            }
        }
        Ok(())
    }
}

#[tauri::command]
pub fn toggle_pause(state: State<SimState>) -> CommandResult<bool> {
    let mut sim = lock_sim(&state)?;
    sim.paused = !sim.paused;
    Ok(sim.paused)
}

#[tauri::command]
pub fn set_speed(state: State<SimState>, multiplier: f64) -> CommandResult<f64> {
    require_finite("multiplier", multiplier)?;
    let mut sim = lock_sim(&state)?;
    sim.speed_multiplier = multiplier.clamp(0.25, 8.0);
    Ok(sim.speed_multiplier)
}

#[tauri::command]
pub fn load_test_scenario(state: State<SimState>) -> CommandResult<()> {
    let mut sim = lock_sim(&state)?;
    scenarios::load_sun_earth(&mut sim);
    Ok(())
}

#[tauri::command]
pub fn clear_simulation(state: State<SimState>) -> CommandResult<()> {
    let mut sim = lock_sim(&state)?;
    sim.clear();
    Ok(())
}

#[tauri::command]
pub fn add_body(state: State<SimState>, body_data: BodyData) -> CommandResult<u32> {
    body_data.validate()?;
    let mut sim = lock_sim(&state)?;
    let id = sim.allocate_id();
    let mass = body_data.mass.max(0.01);
    let radius = body_data.radius.max(0.5);
    let mut body = CelestialBody::new(
        id,
        &body_data.name,
        Vec3::new(body_data.x, body_data.y, body_data.z),
        Vec3::new(body_data.vx, body_data.vy, body_data.vz),
        mass,
        radius,
        &body_data.color,
        body_data.is_fixed,
    );
    body.body_type = body_data.body_type;
    sim.add_body(body);
    Ok(id)
}

#[tauri::command]
pub fn remove_body(state: State<SimState>, id: u32) -> CommandResult<()> {
    let mut sim = lock_sim(&state)?;
    sim.remove_body(id);
    Ok(())
}

#[tauri::command]
pub fn update_body(state: State<SimState>, id: u32, fields: BodyUpdate) -> CommandResult<()> {
    fields.validate()?;
    let mut sim = lock_sim(&state)?;
    let mut needs_reprime = false;
    if let Some(body) = sim.find_body_mut(id) {
        if let Some(mass) = fields.mass {
            body.mass = mass.max(0.01);
            needs_reprime = true;
        }
        if let Some(radius) = fields.radius {
            body.radius = radius.max(0.5);
        }
        if let Some(color) = fields.color {
            body.color = color;
        }
        if let Some(name) = fields.name {
            body.name = name;
        }
        if let Some(is_fixed) = fields.is_fixed {
            body.is_fixed = is_fixed;
            needs_reprime = true;
        }
    } else {
        return Err(format!("body {id} not found"));
    }
    if needs_reprime {
        sim.prime_accelerations();
    }
    Ok(())
}

#[tauri::command]
pub fn update_body_velocity(
    state: State<SimState>,
    id: u32,
    vx: f64,
    vy: f64,
    vz: Option<f64>,
) -> CommandResult<()> {
    require_finite("vx", vx)?;
    require_finite("vy", vy)?;
    let vz = vz.unwrap_or(0.0);
    require_finite("vz", vz)?;
    require_abs_max("vx", vx, MAX_VELOCITY_COMPONENT)?;
    require_abs_max("vy", vy, MAX_VELOCITY_COMPONENT)?;
    require_abs_max("vz", vz, MAX_VELOCITY_COMPONENT)?;
    let mut sim = lock_sim(&state)?;
    if let Some(body) = sim.find_body_mut(id) {
        body.velocity = Vec3::new(vx, vy, vz);
    } else {
        return Err(format!("body {id} not found"));
    }
    Ok(())
}

#[tauri::command]
pub fn set_spacecraft_thrust(
    state: State<SimState>,
    id: u32,
    tx: f64,
    ty: f64,
    tz: f64,
) -> CommandResult<()> {
    require_finite("tx", tx)?;
    require_finite("ty", ty)?;
    require_finite("tz", tz)?;
    require_abs_max("tx", tx, MAX_THRUST_COMPONENT)?;
    require_abs_max("ty", ty, MAX_THRUST_COMPONENT)?;
    require_abs_max("tz", tz, MAX_THRUST_COMPONENT)?;
    let mut sim = lock_sim(&state)?;
    if let Some(body) = sim.find_body_mut(id) {
        if body.body_type == BodyType::Spacecraft {
            body.thrust = Vec3::new(tx, ty, tz);
        } else {
            return Err(format!("body {id} is not a spacecraft"));
        }
    } else {
        return Err(format!("body {id} not found"));
    }
    Ok(())
}

#[tauri::command]
pub fn load_scenario(state: State<SimState>, name: String) -> CommandResult<()> {
    let mut sim = lock_sim(&state)?;
    apply_scenario(&mut sim, &name)
}

#[tauri::command]
pub fn generate_system(
    state: State<SimState>,
    star_mass: f64,
    planet_count: u32,
    min_spacing: f64,
    max_radius: f64,
) -> CommandResult<()> {
    require_finite("star_mass", star_mass)?;
    require_finite("min_spacing", min_spacing)?;
    require_finite("max_radius", max_radius)?;
    if star_mass <= 0.0 {
        return Err("star_mass must be > 0".to_string());
    }
    if planet_count == 0 {
        return Err("planet_count must be >= 1".to_string());
    }
    if planet_count > MAX_PLANETS {
        return Err(format!("planet_count must be <= {MAX_PLANETS}"));
    }
    if min_spacing <= 0.0 {
        return Err("min_spacing must be > 0".to_string());
    }
    if max_radius <= min_spacing {
        return Err("max_radius must be greater than min_spacing".to_string());
    }

    let mut sim = lock_sim(&state)?;
    procedural::generate_system(&mut sim, star_mass, planet_count, min_spacing, max_radius);
    Ok(())
}

#[tauri::command]
pub fn load_galaxy_collision(
    state: State<SimState>,
    particles_per_galaxy: Option<u32>,
) -> CommandResult<()> {
    let particles = particles_per_galaxy.unwrap_or(300);
    if particles == 0 {
        return Err("particles_per_galaxy must be >= 1".to_string());
    }
    let mut sim = lock_sim(&state)?;
    galaxy::generate_collision(&mut sim, particles);
    Ok(())
}

#[tauri::command]
pub fn set_theta(state: State<SimState>, theta: f64) -> CommandResult<f64> {
    require_finite("theta", theta)?;
    let mut sim = lock_sim(&state)?;
    sim.theta = theta.clamp(0.0, 2.0);
    Ok(sim.theta)
}

#[tauri::command]
pub fn predict_orbit(state: State<SimState>, body_id: u32, steps: u32) -> CommandResult<Vec<Vec3>> {
    let sim = lock_sim(&state)?;
    if sim.find_body(body_id).is_none() {
        return Err(format!("body {body_id} not found"));
    }
    Ok(sim.predict_orbit(body_id, steps.min(2000)))
}

#[tauri::command]
pub fn export_state(state: State<SimState>) -> Result<String, String> {
    let sim = lock_sim(&state)?;
    serde_json::to_string_pretty(&*sim).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_state(state: State<SimState>, json: String) -> Result<(), String> {
    let mut new_state: SimulationState =
        serde_json::from_str(&json).map_err(|e| e.to_string())?;

    let mut sim = lock_sim(&state)?;
    finalize_imported_state(&mut new_state, sim.gpu.clone())?;
    *sim = new_state;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::Vec3;

    #[test]
    fn apply_scenario_loads_expected_outer_solar_bodies() {
        let mut sim = SimulationState::new();

        apply_scenario(&mut sim, "outer_solar").expect("outer solar should load");

        let names: Vec<&str> = sim.bodies.iter().map(|body| body.name.as_str()).collect();
        assert_eq!(sim.bodies.len(), 5);
        assert_eq!(names, vec!["Sun", "Jupiter", "Saturn", "Uranus", "Neptune"]);
        assert!(sim.validate().is_ok());
    }

    #[test]
    fn apply_scenario_rejects_unknown_name() {
        let mut sim = SimulationState::new();
        let err = apply_scenario(&mut sim, "made_up").expect_err("unknown scenario should fail");
        assert!(err.contains("unknown scenario"));
    }

    #[test]
    fn finalize_imported_state_bumps_next_id_and_primes_accelerations() {
        let mut imported = SimulationState::new();
        scenarios::load_sun_earth(&mut imported);

        imported.next_id = 0;
        for body in &mut imported.bodies {
            body.acceleration = Vec3::zero();
        }

        finalize_imported_state(&mut imported, None).expect("import finalization should succeed");

        let max_id = imported.bodies.iter().map(|body| body.id).max().unwrap_or(0);
        assert_eq!(imported.next_id, max_id + 1);
        let earth = imported
            .bodies
            .iter()
            .find(|body| body.name == "Earth")
            .expect("earth should exist");
        assert!(earth.acceleration.magnitude() > 0.0);
    }
}
