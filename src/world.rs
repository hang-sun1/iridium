use glam::Vec3;

#[derive(Clone, Copy)]
struct AABB {
    min: Vec3,
    max: Vec3,
}

struct AABBIter {
    positions: [Vec3; 8],
    i: usize,
}

#[derive(Clone, Copy)]
struct Boid {
    position: Vec3,
    aabb: AABB,
}

struct BoidCellsIter {
    positions: [Vec3; 8],
    length: f32,
    width: f32,
    conv_factor: f32,
}

struct Cell {
    min: Vec3,
    max: Vec3,
    boids_inside: Vec<Boid>,
}

// For now, the world is a cube
struct World {
    length: f32,
    width: f32,
    height: f32,
    cell_size: f32,
    boids: Vec<Boid>,
    hash_table: Vec<Cell>
}

impl AABB {
    fn new(min: Vec3, max: Vec3) -> Self {
        Self {
            min,
            max,
        }
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
        AABBIter { positions: points, i: 0 }
    }
}

impl Iterator for AABBIter {
    type Item = Vec3;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < 8 {
            self.i += 1;
            return Some(self.positions[self.i-1])
        }
        None
    }
}

impl Boid {
    fn new(position: Vec3, aabb: AABB) -> Self {
        Self {
            position,
            aabb,
        }
    }

    fn grid_cells(&self, length: f32, width: f32, conv_factor: f32) -> BoidCellsIter {

        // BoidCellsIter {
        //     length,
        //     width,
        //     conv_factor,

        // }
        todo!()
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


// impl World {
//     fn new(side_len: f32, cells_per_side: usize) -> Self {
//         let num_cells = cells_per_side * cells_per_side * cells_per_side;
//         let mut hash_table = Vec::with_capacity(num_cells);
//         hash_table.resize(num_cells, Cell::new())

//         Self {
//             length: side_len,
//             width: side_len,
//             height: side_len,
//             boids: Vec::new(),
//             hash_table
//         }
//     }
// }

#[cfg(test)]
mod tests {

    use glam::Vec3;

    use super::AABB;

    #[test]
    fn the_aabb_iter_works() {
        let aabb = AABB { min: Vec3::new(0.0, 0.0, 0.0), max: Vec3::new(1.0, 1.0, 1.0) };
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
}