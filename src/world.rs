use glam::{Vec3, Mat4};
use super::BoidInstance;

#[derive(Clone, Copy, Debug)]
struct AABB {
    min: Vec3,
    max: Vec3,
}

struct AABBIter {
    positions: [Vec3; 8],
    i: usize,
}

#[derive(Clone, Copy, Debug)]
struct Boid {
    position: Vec3,
    aabb: AABB,
}

struct BoidCellsIter {
    cells: [u64; 8],
}

struct Cell {
    min: Vec3,
    max: Vec3,
    boids_inside: Vec<Boid>,
}

// For now, the world is a cube
pub struct World {
    length: f32,
    width: f32,
    height: f32,
    cell_size: f32,
    boids: Vec<Boid>,
    hash_table: Vec<Cell>,
}

impl AABB {
    fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    fn points(&self) -> AABBIter {
        let points = [
            self.min,
            self.max,
            Vec3::new(self.min.x, self.min.y, self.max.z),
            Vec3::new(self.min.x, self.max.y, self.min.z),
            Vec3::new(self.min.x, self.max.y, self.max.z),
            Vec3::new(self.max.x, self.min.y, self.max.z),
            Vec3::new(self.max.x, self.max.y, self.min.z),
            Vec3::new(self.max.x, self.min.y, self.min.z),
        ];
        AABBIter {
            positions: points,
            i: 0,
        }
    }
}

impl Iterator for AABBIter {
    type Item = Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < 8 {
            self.i += 1;
            return Some(self.positions[self.i - 1]);
        }
        None
    }
}

impl Boid {
    fn new(position: Vec3) -> Self {
        Self {
            position,
            aabb: AABB::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0)),
        }
    }

    fn grid_cells(&self, length: f32, width: f32, conv_factor: f32) -> BoidCellsIter {
        let conv = |p: Vec3| (conv_factor * (p.x + p.y * width + p.z * length * width)) as u64;
        let mut points = self.aabb.points();
        let first = conv(points.next().unwrap());
        let second = conv(points.next().unwrap());

        if first == second {
            return BoidCellsIter {
                cells: [first, first, first, first, first, first, first, first],
            };
        }

        BoidCellsIter {
            cells: [
                first,
                second,
                conv(points.next().unwrap()),
                conv(points.next().unwrap()),
                conv(points.next().unwrap()),
                conv(points.next().unwrap()),
                conv(points.next().unwrap()),
                conv(points.next().unwrap()),
            ],
        }
    }
}

impl Cell {
    fn new(min: Vec3, max: Vec3) -> Self {
        Self {
            min,
            max,
            boids_inside: Vec::new(),
        }
    }
}

impl World {
    pub fn new(side_len: f32, cells_per_side: usize) -> Self {
        let num_cells = cells_per_side * cells_per_side * cells_per_side;

        // TODO: actually use the hash table
        let hash_table = Vec::with_capacity(num_cells);
        // hash_table.resize(num_cells, Cell::new());

        Self {
            length: side_len,
            width: side_len,
            height: side_len,
            cell_size: 0.0,
            boids: Vec::new(),
            hash_table,
        }
    }

    pub fn add_boid(&mut self, pos: Vec3) {
        self.boids.push(Boid::new(pos));
    }

    pub fn update(&mut self) {
        // for boid in self.boids.iter_mut() {
        //     boid.position += 0.01;
        // }
    }

    pub(crate) fn fill_instance_buffer(&self, buff: &mut Vec<BoidInstance>, view: Mat4, proj: Mat4) {
        buff.clear();
        for boid in self.boids.iter() {
            let trans = Mat4::from_translation(boid.position);
            let inst = BoidInstance {
                mvp: proj * view * trans,
                // mvp: Mat4::IDENTITY,
            };
            buff.push(inst);
        }
    }
}

#[cfg(test)]
mod tests {

    use glam::Vec3;

    use super::AABB;

    #[test]
    fn the_aabb_iter_works() {
        let aabb = AABB {
            min: Vec3::new(0.0, 0.0, 0.0),
            max: Vec3::new(1.0, 1.0, 1.0),
        };
        let mut iter = aabb.points();

        assert_eq!(iter.next(), Some(Vec3::new(0.0, 0.0, 0.0)));
        assert_eq!(iter.next(), Some(Vec3::new(1.0, 1.0, 1.0)));
        assert_eq!(iter.next(), Some(Vec3::new(0.0, 0.0, 1.0)));
        assert_eq!(iter.next(), Some(Vec3::new(0.0, 1.0, 0.0)));
        assert_eq!(iter.next(), Some(Vec3::new(0.0, 1.0, 1.0)));
        assert_eq!(iter.next(), Some(Vec3::new(1.0, 0.0, 1.0)));
        assert_eq!(iter.next(), Some(Vec3::new(1.0, 1.0, 0.0)));
        assert_eq!(iter.next(), Some(Vec3::new(1.0, 0.0, 0.0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn it_computes_sane_grid_cells() {}
}
