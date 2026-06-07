use crate::barneshut;
use crate::gpu_gravity::GpuGravity;
use crate::physics::{BodyType, CelestialBody, Vec3};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyData {
    pub kinetic: f64,
    pub potential: f64,
    pub total: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationFrame {
    pub bodies: Vec<CelestialBody>,
    pub tick: u64,
    pub paused: bool,
    pub speed_multiplier: f64,
    pub energy: EnergyData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollisionEvent {
    pub absorbed_id: u32,
    pub survivor_id: u32,
    pub position: Vec3,
    pub combined_mass: f64,
}

#[derive(Serialize, Deserialize)]
pub struct SimulationState {
    pub bodies: Vec<CelestialBody>,
    pub tick: u64,
    pub dt: f64,
    pub g: f64,
    pub softening: f64,
    pub paused: bool,
    pub speed_multiplier: f64,
    pub next_id: u32,
    #[serde(default = "default_theta")]
    pub theta: f64,
    #[serde(skip)]
    pub gpu: Option<Arc<GpuGravity>>,
}

fn default_theta() -> f64 {
    0.5
}

impl SimulationState {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            tick: 0,
            dt: 0.016,
            g: 100.0,
            softening: 10.0,
            paused: false,
            speed_multiplier: 1.0,
            next_id: 0,
            theta: 0.5,
            gpu: None,
        }
    }

    pub fn allocate_id(&mut self) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn add_body(&mut self, body: CelestialBody) -> u32 {
        let id = body.id;
        self.bodies.push(body);
        self.compute_accelerations();
        id
    }

    pub fn remove_body(&mut self, id: u32) {
        self.bodies.retain(|b| b.id != id);
        self.compute_accelerations();
    }

    pub fn find_body_mut(&mut self, id: u32) -> Option<&mut CelestialBody> {
        self.bodies.iter_mut().find(|b| b.id == id)
    }

    pub fn find_body(&self, id: u32) -> Option<&CelestialBody> {
        self.bodies.iter().find(|b| b.id == id)
    }

    pub fn step(&mut self) -> Vec<CollisionEvent> {
        if self.paused || self.bodies.is_empty() {
            return Vec::new();
        }

        // Defensive normalization in case invalid values are injected through deserialization.
        let speed = if self.speed_multiplier.is_finite() {
            self.speed_multiplier.clamp(0.25, 8.0)
        } else {
            1.0
        };
        let sub_steps = speed.ceil().max(1.0) as u32;
        let dt = self.dt * speed / sub_steps as f64;

        let mut all_collisions = Vec::new();

        for _ in 0..sub_steps {
            self.step_verlet(dt);
            let collisions = self.check_collisions();
            if !collisions.is_empty() {
                // Collision merges mutate masses/positions/body set; refresh fields immediately.
                self.compute_accelerations();
            }
            all_collisions.extend(collisions);
        }

        if self.tick % 2 == 0 {
            for body in self.bodies.iter_mut() {
                if !body.is_fixed {
                    body.record_trail();
                }
            }
        }

        self.tick += 1;
        all_collisions
    }

    fn step_verlet(&mut self, dt: f64) {
        for body in self.bodies.iter_mut() {
            if body.is_fixed {
                continue;
            }
            body.position = body.position
                + body.velocity.scale(dt)
                + body.acceleration.scale(0.5 * dt * dt);
        }

        let old_accelerations: Vec<Vec3> =
            self.bodies.iter().map(|b| b.acceleration).collect();

        self.compute_accelerations();

        // Apply spacecraft thrust
        for body in self.bodies.iter_mut() {
            if body.body_type == BodyType::Spacecraft && body.fuel > 0.0 {
                let thrust_mag = body.thrust.magnitude();
                if thrust_mag > 0.001 {
                    let thrust_accel = body.thrust.scale(1.0 / body.mass);
                    body.acceleration += thrust_accel;
                    body.fuel = (body.fuel - thrust_mag * dt * 0.1).max(0.0);
                }
            }
        }

        for (i, body) in self.bodies.iter_mut().enumerate() {
            if body.is_fixed {
                continue;
            }
            body.velocity += (old_accelerations[i] + body.acceleration).scale(0.5 * dt);
        }
    }

    fn compute_accelerations(&mut self) {
        let n = self.bodies.len();

        if n > 500 {
            if let Some(gpu) = &self.gpu {
                if self.compute_accelerations_gpu(gpu.clone()) {
                    return;
                }
            }
        }

        if n > 50 {
            self.compute_accelerations_barneshut();
        } else {
            self.compute_accelerations_brute();
        }
    }

    fn compute_accelerations_brute(&mut self) {
        let n = self.bodies.len();
        let mut accels = vec![Vec3::zero(); n];

        for i in 0..n {
            if self.bodies[i].is_fixed {
                continue;
            }
            for j in 0..n {
                if i == j {
                    continue;
                }
                let diff = self.bodies[j].position - self.bodies[i].position;
                let dist_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z + self.softening * self.softening;
                let dist = dist_sq.sqrt();
                let force_mag = self.g * self.bodies[j].mass / dist_sq;
                let dir = diff.scale(1.0 / dist);
                accels[i] += dir.scale(force_mag);
            }
        }

        for (i, body) in self.bodies.iter_mut().enumerate() {
            body.acceleration = accels[i];
        }
    }

    fn compute_accelerations_gpu(&mut self, gpu: Arc<GpuGravity>) -> bool {
        let positions: Vec<Vec3> = self.bodies.iter().map(|b| b.position).collect();
        let masses: Vec<f64> = self.bodies.iter().map(|b| b.mass).collect();
        let softening_sq = self.softening * self.softening;

        let accels = match gpu.compute_accelerations(&positions, &masses, self.g, softening_sq) {
            Ok(values) => values,
            Err(err) => {
                eprintln!("GPU acceleration failed, falling back to CPU: {err}");
                return false;
            }
        };

        for (i, body) in self.bodies.iter_mut().enumerate() {
            if !body.is_fixed {
                body.acceleration = accels[i];
            } else {
                body.acceleration = Vec3::zero();
            }
        }
        true
    }

    fn compute_accelerations_barneshut(&mut self) {
        let n = self.bodies.len();
        let positions: Vec<Vec3> = self.bodies.iter().map(|b| b.position).collect();
        let masses: Vec<f64> = self.bodies.iter().map(|b| b.mass).collect();

        let tree = barneshut::build_octree(&positions, &masses);
        let softening_sq = self.softening * self.softening;

        let mut accels = vec![Vec3::zero(); n];
        for i in 0..n {
            if self.bodies[i].is_fixed {
                continue;
            }
            accels[i] = tree.compute_acceleration(
                &positions[i],
                i,
                self.g,
                softening_sq,
                self.theta,
            );
        }

        for (i, body) in self.bodies.iter_mut().enumerate() {
            body.acceleration = accels[i];
        }
    }

    fn check_collisions(&mut self) -> Vec<CollisionEvent> {
        let mut collisions = Vec::new();
        let mut absorbed: Vec<bool> = vec![false; self.bodies.len()];

        let n = self.bodies.len();
        for i in 0..n {
            if absorbed[i] {
                continue;
            }
            for j in (i + 1)..n {
                if absorbed[j] {
                    continue;
                }
                let diff = self.bodies[j].position - self.bodies[i].position;
                let dist = (diff.x * diff.x + diff.y * diff.y + diff.z * diff.z).sqrt();
                let overlap = self.bodies[i].radius + self.bodies[j].radius;

                if dist < overlap {
                    let (survivor_idx, absorbed_idx) = if self.bodies[i].mass >= self.bodies[j].mass
                    {
                        (i, j)
                    } else {
                        (j, i)
                    };

                    let m1 = self.bodies[survivor_idx].mass;
                    let m2 = self.bodies[absorbed_idx].mass;
                    let total_mass = m1 + m2;

                    let new_velocity = Vec3::new(
                        (m1 * self.bodies[survivor_idx].velocity.x
                            + m2 * self.bodies[absorbed_idx].velocity.x)
                            / total_mass,
                        (m1 * self.bodies[survivor_idx].velocity.y
                            + m2 * self.bodies[absorbed_idx].velocity.y)
                            / total_mass,
                        (m1 * self.bodies[survivor_idx].velocity.z
                            + m2 * self.bodies[absorbed_idx].velocity.z)
                            / total_mass,
                    );

                    let new_position = Vec3::new(
                        (m1 * self.bodies[survivor_idx].position.x
                            + m2 * self.bodies[absorbed_idx].position.x)
                            / total_mass,
                        (m1 * self.bodies[survivor_idx].position.y
                            + m2 * self.bodies[absorbed_idx].position.y)
                            / total_mass,
                        (m1 * self.bodies[survivor_idx].position.z
                            + m2 * self.bodies[absorbed_idx].position.z)
                            / total_mass,
                    );

                    let r1 = self.bodies[survivor_idx].radius;
                    let r2 = self.bodies[absorbed_idx].radius;
                    let new_radius = (r1 * r1 * r1 + r2 * r2 * r2).cbrt();

                    let collision = CollisionEvent {
                        absorbed_id: self.bodies[absorbed_idx].id,
                        survivor_id: self.bodies[survivor_idx].id,
                        position: new_position,
                        combined_mass: total_mass,
                    };

                    self.bodies[survivor_idx].mass = total_mass;
                    self.bodies[survivor_idx].velocity = new_velocity;
                    self.bodies[survivor_idx].position = new_position;
                    self.bodies[survivor_idx].radius = new_radius;
                    if self.bodies[absorbed_idx].is_fixed {
                        self.bodies[survivor_idx].is_fixed = true;
                    }

                    absorbed[absorbed_idx] = true;
                    collisions.push(collision);
                }
            }
        }

        // Remove absorbed bodies in reverse to preserve indices
        let mut i = self.bodies.len();
        while i > 0 {
            i -= 1;
            if absorbed[i] {
                self.bodies.remove(i);
            }
        }

        collisions
    }

    pub fn predict_orbit(&self, body_id: u32, steps: u32) -> Vec<Vec3> {
        let mut pred = SimulationState {
            bodies: self.bodies.clone(),
            tick: 0,
            dt: self.dt,
            g: self.g,
            softening: self.softening,
            paused: false,
            speed_multiplier: 1.0,
            next_id: self.next_id,
            theta: self.theta,
            gpu: self.gpu.clone(),
        };

        for body in pred.bodies.iter_mut() {
            body.trail.clear();
        }

        let mut path = Vec::with_capacity(steps as usize);

        for _ in 0..steps {
            pred.step_verlet(pred.dt);
            if let Some(body) = pred.find_body(body_id) {
                path.push(body.position);
            } else {
                break;
            }
        }

        path
    }

    fn compute_energies(&self) -> EnergyData {
        let n = self.bodies.len();
        let mut ke = 0.0;
        let mut pe = 0.0;

        for body in &self.bodies {
            let v2 = body.velocity.x * body.velocity.x + body.velocity.y * body.velocity.y + body.velocity.z * body.velocity.z;
            ke += 0.5 * body.mass * v2;
        }

        for i in 0..n {
            for j in (i + 1)..n {
                let diff = self.bodies[j].position - self.bodies[i].position;
                let dist = (diff.x * diff.x + diff.y * diff.y + diff.z * diff.z).sqrt();
                if dist > 0.001 {
                    pe -= self.g * self.bodies[i].mass * self.bodies[j].mass / dist;
                }
            }
        }

        EnergyData {
            kinetic: ke,
            potential: pe,
            total: ke + pe,
        }
    }

    pub fn to_frame(&self) -> SimulationFrame {
        SimulationFrame {
            bodies: self.bodies.clone(),
            tick: self.tick,
            paused: self.paused,
            speed_multiplier: self.speed_multiplier,
            energy: self.compute_energies(),
        }
    }

    pub fn prime_accelerations(&mut self) {
        self.compute_accelerations();
    }

    pub fn validate(&self) -> Result<(), String> {
        let finite = |name: &str, value: f64| {
            if value.is_finite() {
                Ok(())
            } else {
                Err(format!("{name} must be finite"))
            }
        };

        finite("dt", self.dt)?;
        finite("g", self.g)?;
        finite("softening", self.softening)?;
        finite("speed_multiplier", self.speed_multiplier)?;
        finite("theta", self.theta)?;

        if self.dt <= 0.0 {
            return Err("dt must be > 0".to_string());
        }
        if self.g <= 0.0 {
            return Err("g must be > 0".to_string());
        }
        if self.softening < 0.0 {
            return Err("softening must be >= 0".to_string());
        }
        if !(0.25..=8.0).contains(&self.speed_multiplier) {
            return Err("speed_multiplier must be in [0.25, 8.0]".to_string());
        }
        if !(0.0..=2.0).contains(&self.theta) {
            return Err("theta must be in [0, 2]".to_string());
        }

        let mut seen_ids: HashSet<u32> = HashSet::with_capacity(self.bodies.len());
        for body in &self.bodies {
            if !seen_ids.insert(body.id) {
                return Err(format!("duplicate body id: {}", body.id));
            }
            finite("body.position.x", body.position.x)?;
            finite("body.position.y", body.position.y)?;
            finite("body.position.z", body.position.z)?;
            finite("body.velocity.x", body.velocity.x)?;
            finite("body.velocity.y", body.velocity.y)?;
            finite("body.velocity.z", body.velocity.z)?;
            finite("body.mass", body.mass)?;
            finite("body.radius", body.radius)?;
            finite("body.thrust.x", body.thrust.x)?;
            finite("body.thrust.y", body.thrust.y)?;
            finite("body.thrust.z", body.thrust.z)?;
            finite("body.fuel", body.fuel)?;
            finite("body.max_fuel", body.max_fuel)?;

            if body.mass <= 0.0 {
                return Err(format!("body {} has non-positive mass", body.id));
            }
            if body.radius <= 0.0 {
                return Err(format!("body {} has non-positive radius", body.id));
            }
            if body.fuel < 0.0 || body.max_fuel < 0.0 {
                return Err(format!("body {} has negative fuel values", body.id));
            }
            if body.fuel > body.max_fuel {
                return Err(format!("body {} has fuel greater than max_fuel", body.id));
            }
        }

        Ok(())
    }

    pub fn clear(&mut self) {
        self.bodies.clear();
        self.tick = 0;
        self.next_id = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::CelestialBody;
    use crate::scenarios;

    fn make_body(
        id: u32,
        name: &str,
        position: Vec3,
        velocity: Vec3,
        mass: f64,
        radius: f64,
        is_fixed: bool,
    ) -> CelestialBody {
        CelestialBody::new(id, name, position, velocity, mass, radius, "#ffffff", is_fixed)
    }

    #[test]
    fn collision_conserves_mass_and_momentum() {
        let mut sim = SimulationState::new();
        sim.bodies.push(make_body(
            0,
            "A",
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(10.0, -3.0, 2.0),
            2.0,
            5.0,
            false,
        ));
        let mut fixed = make_body(
            1,
            "B",
            Vec3::new(7.0, 0.0, 0.0),
            Vec3::new(-4.0, 6.0, -1.0),
            3.0,
            5.0,
            true,
        );
        fixed.is_fixed = true;
        sim.bodies.push(fixed);

        let initial_mass: f64 = sim.bodies.iter().map(|b| b.mass).sum();
        let initial_momentum_x: f64 = sim.bodies.iter().map(|b| b.mass * b.velocity.x).sum();
        let initial_momentum_y: f64 = sim.bodies.iter().map(|b| b.mass * b.velocity.y).sum();
        let initial_momentum_z: f64 = sim.bodies.iter().map(|b| b.mass * b.velocity.z).sum();
        let initial_volume = sim.bodies[0].radius.powi(3) + sim.bodies[1].radius.powi(3);

        let collisions = sim.check_collisions();
        assert_eq!(collisions.len(), 1);
        assert_eq!(sim.bodies.len(), 1);
        assert_ne!(collisions[0].absorbed_id, collisions[0].survivor_id);

        let survivor = &sim.bodies[0];
        let final_mass = survivor.mass;
        let final_momentum_x = survivor.mass * survivor.velocity.x;
        let final_momentum_y = survivor.mass * survivor.velocity.y;
        let final_momentum_z = survivor.mass * survivor.velocity.z;
        assert!((final_mass - initial_mass).abs() < 1e-9);
        assert!((final_momentum_x - initial_momentum_x).abs() < 1e-9);
        assert!((final_momentum_y - initial_momentum_y).abs() < 1e-9);
        assert!((final_momentum_z - initial_momentum_z).abs() < 1e-9);
        assert!((survivor.radius.powi(3) - initial_volume).abs() < 1e-9);
        assert!(survivor.is_fixed);
    }

    #[test]
    fn remove_body_recomputes_acceleration_for_remaining_bodies() {
        let mut sim = SimulationState::new();
        sim.add_body(make_body(
            0,
            "Sun",
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::zero(),
            1000.0,
            10.0,
            true,
        ));
        sim.add_body(make_body(
            1,
            "Planet",
            Vec3::new(100.0, 0.0, 0.0),
            Vec3::new(0.0, 10.0, 0.0),
            1.0,
            3.0,
            false,
        ));

        let accel_before = sim.bodies[1].acceleration.magnitude();
        assert!(accel_before > 0.0);

        sim.remove_body(0);
        assert_eq!(sim.bodies.len(), 1);
        assert!(sim.bodies[0].acceleration.magnitude() < 1e-9);
    }

    #[test]
    fn predict_orbit_returns_finite_positions() {
        let mut sim = SimulationState::new();
        scenarios::load_sun_earth(&mut sim);
        let earth_id = sim
            .bodies
            .iter()
            .find(|b| b.name == "Earth")
            .map(|b| b.id)
            .expect("Earth should exist");

        let path = sim.predict_orbit(earth_id, 300);
        assert_eq!(path.len(), 300);
        assert!(path.iter().all(|p| p.x.is_finite() && p.y.is_finite() && p.z.is_finite()));
    }

    #[test]
    fn barnes_hut_and_bruteforce_are_reasonably_consistent() {
        let mut sim_bh = SimulationState::new();
        sim_bh.g = 50.0;
        sim_bh.softening = 5.0;
        sim_bh.theta = 0.5;

        for i in 0..80 {
            let angle = (i as f64) * 0.3;
            let radius = 40.0 + (i % 9) as f64 * 4.0;
            sim_bh.bodies.push(make_body(
                i as u32,
                "P",
                Vec3::new(radius * angle.cos(), radius * angle.sin(), (i % 5) as f64 * 2.0),
                Vec3::zero(),
                1.0 + (i % 7) as f64 * 0.2,
                1.0,
                false,
            ));
        }

        let mut sim_brute = SimulationState {
            bodies: sim_bh.bodies.clone(),
            tick: sim_bh.tick,
            dt: sim_bh.dt,
            g: sim_bh.g,
            softening: sim_bh.softening,
            paused: sim_bh.paused,
            speed_multiplier: sim_bh.speed_multiplier,
            next_id: sim_bh.next_id,
            theta: sim_bh.theta,
            gpu: None,
        };

        sim_bh.compute_accelerations_barneshut();
        sim_brute.compute_accelerations_brute();

        for (a, b) in sim_bh.bodies.iter().zip(sim_brute.bodies.iter()) {
            let dx = (a.acceleration.x - b.acceleration.x).abs();
            let dy = (a.acceleration.y - b.acceleration.y).abs();
            let dz = (a.acceleration.z - b.acceleration.z).abs();
            assert!(dx < 0.12 && dy < 0.12 && dz < 0.12, "solver mismatch for body {}", a.id);
        }
    }

    #[test]
    fn validate_rejects_duplicate_ids_and_invalid_fuel_bounds() {
        let mut sim = SimulationState::new();
        let mut a = make_body(7, "A", Vec3::zero(), Vec3::zero(), 1.0, 1.0, false);
        let mut b = make_body(7, "B", Vec3::new(10.0, 0.0, 0.0), Vec3::zero(), 1.0, 1.0, false);
        a.fuel = 10.0;
        a.max_fuel = 5.0;
        b.fuel = 1.0;
        b.max_fuel = 5.0;
        sim.bodies.push(a);
        sim.bodies.push(b);

        let err = sim.validate().expect_err("state should fail validation");
        assert!(err.contains("duplicate body id") || err.contains("fuel greater than max_fuel"));
    }
}
