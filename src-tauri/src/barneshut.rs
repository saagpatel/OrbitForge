use crate::physics::Vec3;

const MAX_DEPTH: usize = 20;

pub struct OctreeNode {
    center: Vec3,
    half_size: f64,
    total_mass: f64,
    center_of_mass: Vec3,
    body_index: Option<usize>,
    children: [Option<Box<OctreeNode>>; 8],
}

impl OctreeNode {
    pub fn new(center: Vec3, half_size: f64) -> Self {
        Self {
            center,
            half_size,
            total_mass: 0.0,
            center_of_mass: Vec3::zero(),
            body_index: None,
            children: Default::default(),
        }
    }

    fn octant(&self, pos: &Vec3) -> usize {
        let mut idx = 0;
        if pos.x >= self.center.x { idx |= 1; }
        if pos.y >= self.center.y { idx |= 2; }
        if pos.z >= self.center.z { idx |= 4; }
        idx
    }

    fn child_center(&self, octant: usize) -> Vec3 {
        let q = self.half_size * 0.5;
        Vec3::new(
            self.center.x + if octant & 1 != 0 { q } else { -q },
            self.center.y + if octant & 2 != 0 { q } else { -q },
            self.center.z + if octant & 4 != 0 { q } else { -q },
        )
    }

    pub fn insert(&mut self, idx: usize, pos: &Vec3, mass: f64, depth: usize) {
        if depth >= MAX_DEPTH {
            // Just accumulate mass at this node
            let new_mass = self.total_mass + mass;
            if new_mass > 0.0 {
                self.center_of_mass = Vec3::new(
                    (self.center_of_mass.x * self.total_mass + pos.x * mass) / new_mass,
                    (self.center_of_mass.y * self.total_mass + pos.y * mass) / new_mass,
                    (self.center_of_mass.z * self.total_mass + pos.z * mass) / new_mass,
                );
            }
            self.total_mass = new_mass;
            return;
        }

        if self.total_mass == 0.0 && self.body_index.is_none() {
            // Empty leaf: store this body
            self.body_index = Some(idx);
            self.total_mass = mass;
            self.center_of_mass = *pos;
            return;
        }

        if let Some(existing_idx) = self.body_index.take() {
            // Single-body leaf: subdivide
            let existing_pos = self.center_of_mass;
            let existing_mass = self.total_mass;

            // Reset this node
            self.total_mass = 0.0;
            self.center_of_mass = Vec3::zero();

            // Re-insert existing body
            self.insert_into_child(existing_idx, &existing_pos, existing_mass, depth);
            self.accumulate_mass(&existing_pos, existing_mass);
        }

        // Insert new body into appropriate child
        self.insert_into_child(idx, pos, mass, depth);
        self.accumulate_mass(pos, mass);
    }

    fn accumulate_mass(&mut self, pos: &Vec3, mass: f64) {
        let new_mass = self.total_mass + mass;
        if new_mass > 0.0 {
            self.center_of_mass = Vec3::new(
                (self.center_of_mass.x * self.total_mass + pos.x * mass) / new_mass,
                (self.center_of_mass.y * self.total_mass + pos.y * mass) / new_mass,
                (self.center_of_mass.z * self.total_mass + pos.z * mass) / new_mass,
            );
        }
        self.total_mass = new_mass;
    }

    fn insert_into_child(&mut self, idx: usize, pos: &Vec3, mass: f64, depth: usize) {
        let octant = self.octant(pos);
        let center = self.child_center(octant);
        let hs = self.half_size * 0.5;
        let child = self.children[octant].get_or_insert_with(|| {
            Box::new(OctreeNode::new(center, hs))
        });
        child.insert(idx, pos, mass, depth + 1);
    }

    pub fn compute_acceleration(
        &self,
        pos: &Vec3,
        body_index: usize,
        g: f64,
        softening_sq: f64,
        theta: f64,
    ) -> Vec3 {
        if self.total_mass == 0.0 {
            return Vec3::zero();
        }

        // Single body leaf: direct interaction
        if let Some(leaf_idx) = self.body_index {
            if leaf_idx == body_index {
                return Vec3::zero();
            }
            return direct_accel(pos, &self.center_of_mass, self.total_mass, g, softening_sq);
        }

        // Check Barnes-Hut criterion: s/d < theta
        let diff = self.center_of_mass - *pos;
        let dist_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z + softening_sq;
        let s = self.half_size * 2.0;

        if s * s < theta * theta * dist_sq {
            // Far enough: treat as single body
            return direct_accel(pos, &self.center_of_mass, self.total_mass, g, softening_sq);
        }

        // Recurse into children
        let mut accel = Vec3::zero();
        for child in &self.children {
            if let Some(c) = child {
                accel += c.compute_acceleration(pos, body_index, g, softening_sq, theta);
            }
        }
        accel
    }
}

fn direct_accel(pos: &Vec3, other_pos: &Vec3, other_mass: f64, g: f64, softening_sq: f64) -> Vec3 {
    let diff = *other_pos - *pos;
    let dist_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z + softening_sq;
    let dist = dist_sq.sqrt();
    let force_mag = g * other_mass / dist_sq;
    diff.scale(force_mag / dist)
}

pub fn build_octree(positions: &[Vec3], masses: &[f64]) -> OctreeNode {
    // Find bounding box
    let mut min_x = f64::MAX;
    let mut min_y = f64::MAX;
    let mut min_z = f64::MAX;
    let mut max_x = f64::MIN;
    let mut max_y = f64::MIN;
    let mut max_z = f64::MIN;

    for p in positions {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        min_z = min_z.min(p.z);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
        max_z = max_z.max(p.z);
    }

    let cx = (min_x + max_x) * 0.5;
    let cy = (min_y + max_y) * 0.5;
    let cz = (min_z + max_z) * 0.5;
    let half_size = ((max_x - min_x).max(max_y - min_y).max(max_z - min_z)) * 0.5 + 1.0;

    let mut root = OctreeNode::new(Vec3::new(cx, cy, cz), half_size);

    for (i, (pos, &mass)) in positions.iter().zip(masses.iter()).enumerate() {
        root.insert(i, pos, mass, 0);
    }

    root
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute_accel(
        positions: &[Vec3],
        masses: &[f64],
        i: usize,
        g: f64,
        softening_sq: f64,
    ) -> Vec3 {
        let mut accel = Vec3::zero();
        let pos = positions[i];
        for (j, other) in positions.iter().enumerate() {
            if i == j {
                continue;
            }
            let diff = *other - pos;
            let dist_sq = diff.x * diff.x + diff.y * diff.y + diff.z * diff.z + softening_sq;
            let dist = dist_sq.sqrt();
            let force_mag = g * masses[j] / dist_sq;
            accel += diff.scale(force_mag / dist);
        }
        accel
    }

    #[test]
    fn octree_aggregates_mass_and_center_of_mass() {
        let positions = vec![
            Vec3::new(-2.0, 0.0, 0.0),
            Vec3::new(2.0, 0.0, 0.0),
            Vec3::new(0.0, 4.0, 0.0),
        ];
        let masses = vec![2.0, 3.0, 5.0];
        let tree = build_octree(&positions, &masses);

        let total_mass: f64 = masses.iter().sum();
        let expected_com = Vec3::new(
            (-2.0 * 2.0 + 2.0 * 3.0 + 0.0 * 5.0) / total_mass,
            (0.0 * 2.0 + 0.0 * 3.0 + 4.0 * 5.0) / total_mass,
            0.0,
        );

        assert!((tree.total_mass - total_mass).abs() < 1e-10);
        assert!((tree.center_of_mass.x - expected_com.x).abs() < 1e-10);
        assert!((tree.center_of_mass.y - expected_com.y).abs() < 1e-10);
        assert!((tree.center_of_mass.z - expected_com.z).abs() < 1e-10);
    }

    #[test]
    fn barnes_hut_matches_bruteforce_with_tight_theta() {
        let mut positions = Vec::new();
        let mut masses = Vec::new();
        for i in 0..64 {
            positions.push(Vec3::new(
                (i as f64) * 2.0 - 50.0,
                ((i % 7) as f64) * 3.0 - 8.0,
                ((i % 5) as f64) * 1.5 - 3.0,
            ));
            masses.push(1.0 + (i % 11) as f64 * 0.2);
        }

        let tree = build_octree(&positions, &masses);
        let g = 100.0;
        let softening_sq = 100.0;
        let theta = 0.01;

        for i in 0..positions.len() {
            let bh = tree.compute_acceleration(&positions[i], i, g, softening_sq, theta);
            let brute = brute_accel(&positions, &masses, i, g, softening_sq);
            assert!((bh.x - brute.x).abs() < 5e-6, "x mismatch at index {i}");
            assert!((bh.y - brute.y).abs() < 5e-6, "y mismatch at index {i}");
            assert!((bh.z - brute.z).abs() < 5e-6, "z mismatch at index {i}");
        }
    }

    #[test]
    fn tighter_theta_reduces_approximation_error() {
        let mut positions = Vec::new();
        let mut masses = Vec::new();
        for i in 0..72 {
            positions.push(Vec3::new(
                (i as f64) * 1.7 - 40.0,
                ((i % 9) as f64) * 2.5 - 9.0,
                ((i % 6) as f64) * 1.2 - 3.0,
            ));
            masses.push(0.8 + (i % 13) as f64 * 0.15);
        }

        let tree = build_octree(&positions, &masses);
        let g = 80.0;
        let softening_sq = 64.0;
        let loose_theta = 1.2;
        let tight_theta = 0.2;

        let mut loose_error = 0.0;
        let mut tight_error = 0.0;

        for i in 0..positions.len() {
            let brute = brute_accel(&positions, &masses, i, g, softening_sq);
            let loose = tree.compute_acceleration(&positions[i], i, g, softening_sq, loose_theta);
            let tight = tree.compute_acceleration(&positions[i], i, g, softening_sq, tight_theta);

            loose_error += (loose.x - brute.x).abs() + (loose.y - brute.y).abs() + (loose.z - brute.z).abs();
            tight_error += (tight.x - brute.x).abs() + (tight.y - brute.y).abs() + (tight.z - brute.z).abs();
        }

        assert!(tight_error < loose_error, "tight theta should reduce aggregate error");
    }
}
