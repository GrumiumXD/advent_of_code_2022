use std::collections::{HashSet, VecDeque};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Voxel {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Voxel {
    pub fn neighbors(&self) -> [Voxel; 6] {
        let sides = [
            Voxel {
                x: self.x - 1,
                y: self.y,
                z: self.z,
            },
            Voxel {
                x: self.x + 1,
                y: self.y,
                z: self.z,
            },
            Voxel {
                x: self.x,
                y: self.y - 1,
                z: self.z,
            },
            Voxel {
                x: self.x,
                y: self.y + 1,
                z: self.z,
            },
            Voxel {
                x: self.x,
                y: self.y,
                z: self.z - 1,
            },
            Voxel {
                x: self.x,
                y: self.y,
                z: self.z + 1,
            },
        ];

        sides
    }
}

struct Droplet {
    voxels: HashSet<Voxel>,
    bounding_box: (Voxel, Voxel),
}

impl Droplet {
    pub fn new(voxels: HashSet<Voxel>) -> Self {
        let mut min = voxels.iter().next().unwrap().clone();
        let mut max = min.clone();

        for v in voxels.iter() {
            min.x = min.x.min(v.x);
            min.y = min.y.min(v.y);
            min.z = min.z.min(v.z);

            max.x = max.x.max(v.x);
            max.y = max.y.max(v.y);
            max.z = max.z.max(v.z);
        }

        Droplet {
            voxels,
            bounding_box: (min, max),
        }
    }

    pub fn surface(&self) -> usize {
        self.voxels
            .iter()
            .map(|v| {
                let sides = v.neighbors();

                sides.iter().filter(|&s| !self.voxels.contains(s)).count()
            })
            .sum::<usize>()
    }

    fn touches_bb(&self, v: &Voxel) -> bool {
        v.x == self.bounding_box.0.x
            || v.x == self.bounding_box.1.x
            || v.y == self.bounding_box.0.y
            || v.y == self.bounding_box.1.y
            || v.z == self.bounding_box.0.z
            || v.z == self.bounding_box.1.z
    }

    pub fn flood_fill(&self, start: &Voxel) -> Option<HashSet<Voxel>> {
        let mut queue = VecDeque::new();
        let mut filled = HashSet::new();

        queue.push_back(start.clone());

        while !queue.is_empty() {
            let v = queue.pop_front().unwrap();

            if !(filled.contains(&v) || self.voxels.contains(&v)) {
                // skip the ones that touch the bounding box
                // as the would count towards the outside
                if self.touches_bb(&v) {
                    return None;
                }

                for n in v.neighbors() {
                    queue.push_back(n);
                }

                filled.insert(v);
            }
        }

        if filled.is_empty() {
            return None;
        }
        Some(filled)
    }

    pub fn inner_cube(&self) -> CubeIterator {
        CubeIterator::new((
            Voxel {
                x: self.bounding_box.0.x + 1,
                y: self.bounding_box.0.y + 1,
                z: self.bounding_box.0.z + 1,
            },
            Voxel {
                x: self.bounding_box.1.x - 1,
                y: self.bounding_box.1.y - 1,
                z: self.bounding_box.1.z - 1,
            },
        ))
    }

    pub fn add_voxels(&mut self, voxels: HashSet<Voxel>) {
        for v in voxels {
            self.voxels.insert(v);
        }
    }
}

struct CubeIterator {
    bb: (Voxel, Voxel),
    current: Voxel,
}

impl CubeIterator {
    pub fn new(bb: (Voxel, Voxel)) -> Self {
        let current = Voxel {
            x: bb.0.x - 1,
            y: bb.0.y,
            z: bb.0.z,
        };

        CubeIterator { bb, current }
    }
}

impl Iterator for CubeIterator {
    type Item = Voxel;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.x += 1;
        if self.current.x > self.bb.1.x {
            self.current.x = self.bb.0.x;
            self.current.y += 1;
            if self.current.y > self.bb.1.y {
                self.current.y = self.bb.0.y;
                self.current.z += 1;
            }
        }

        if self.current.z <= self.bb.1.z {
            return Some(self.current.clone());
        }

        None
    }
}

fn parse_voxels(input: &str) -> HashSet<Voxel> {
    input
        .lines()
        .map(|l| {
            let coords = l
                .split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            Voxel {
                x: coords[0],
                y: coords[1],
                z: coords[2],
            }
        })
        .collect()
}

pub fn puzzle_1(input: &str) -> String {
    let voxels = parse_voxels(input);

    let droplet = Droplet::new(voxels);

    let open_sides = droplet.surface();

    open_sides.to_string()
}

pub fn puzzle_2(input: &str) -> String {
    let voxels = parse_voxels(input);

    let mut droplet = Droplet::new(voxels);

    // fill all holes
    for v in droplet.inner_cube() {
        if let Some(fill) = droplet.flood_fill(&v) {
            droplet.add_voxels(fill);
        }
    }

    let result = droplet.surface();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn p1() {
        let result = puzzle_1(INPUT);
        assert_eq!(result, "64");
    }

    #[test]
    fn p2() {
        let result = puzzle_2(INPUT);
        assert_eq!(result, "58");
    }
}
