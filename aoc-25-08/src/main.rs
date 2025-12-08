use std::collections::{HashMap};

fn main() {
    let input = include_str!("input");
    let (part1, part2) = doing_circuitry(input, 1000);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Circuit {
    x: i64,
    y: i64,
    z: i64,
}

impl Circuit {
    fn distance(&self, other: &Circuit) -> i64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).isqrt()
    }
}

fn doing_circuitry(input: &str, runs: i64) -> (usize, usize) {
    let circuits: Vec<Circuit> = input.lines().map(|line| {
        let [x, y, z] = line.split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
            .try_into()
            .unwrap();
        Circuit { x, y, z }
    }).collect();

    let len = circuits.len();

    // initial boxes, each circuit its own box, will combine later.
    // will put a "root" (lowest circuit of box) at index for each circuit that circuit belongs to.
    // so a box of 0,1,2 means [0, 0, 0, 3, 4 ...] because the first three are a box with "root" 0.
    let mut boxes: Vec<usize> = (0..len).collect();

    // pre-calc all distances, to just pop the shortest ones, instead of doing double loops each run
    let distances = calc_distances(&circuits, len);

    for idx in 0..(runs as usize) {
        let (_distance, i, j) = distances[idx];
        let _connected = union(&mut boxes, i, j);
        // if connected {
        //     println!("connected at run {}", idx + 1);
        // }
    }

    let mut circuit_sizes = HashMap::new();
    for i in 0..len {
        let root = find(&mut boxes, i);
        *circuit_sizes.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<usize> = circuit_sizes.values().copied().collect();
    sizes.sort_by_key(|&s| std::cmp::Reverse(s));
    let part1 = sizes.iter().take(3).product();

    // Part 2, keep going until only one circuit.
    let mut num_circuits = sizes.len();
    let mut idx = runs as usize;
    let last_connected = loop {
        let (_distance, i, j) = distances[idx];

        if union(&mut boxes, i, j) {
            num_circuits -= 1;

            if num_circuits == 1 {
                break (i, j);
            }
        }
        idx += 1;
    };

    let part2 = circuits[last_connected.0].x * circuits[last_connected.1].x;
    (part1, part2.try_into().unwrap())
}

fn calc_distances(circuits: &Vec<Circuit>, len: usize) -> Vec<(i64, usize, usize)> {
    let mut distances = Vec::new();
    for i in 0..len {
        for j in (i + 1)..len {
            let distance = circuits[i].distance(&circuits[j]);
            distances.push((distance, i, j));
        }
    }
    distances.sort_by_key(|&(d, _, _)| d);
    distances
}

fn find(circuit_box: &Vec<usize>, x: usize) -> usize {
    if circuit_box[x] == x {
        return x;
    }
    find(circuit_box, circuit_box[x])
}

fn union(circuit_box: &mut Vec<usize>, x: usize, y: usize) -> bool {
    let root_x = find(circuit_box, x);
    let root_y = find(circuit_box, y);

    if root_x == root_y {
        return false;
    }

    // always attach to smaller root, doesn't matter.
    if root_y > root_x {
        circuit_box[root_y] = root_x;
    } else {
        circuit_box[root_x] = root_y;
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("testinput");
        assert_eq!(doing_circuitry(input, 10), (40, 25272));
    }
    #[test]
    fn real_input() {
        let input = include_str!("input");
        assert_eq!(doing_circuitry(input, 1000), (54180, 25325968));
    }
}