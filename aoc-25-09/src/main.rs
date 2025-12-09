use std::collections::{HashSet};
use std::hash::Hash;
use itertools::Itertools;
use rayon::prelude::*;

fn main() {
    let input = include_str!("input");
    let (part1, part2) = dont_be_a_square(&input);
    println!("{}", part1);
    println!("{}", part2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn rect(&self, other: &Point) -> i64 {
        ((other.x - self.x).abs() + 1) as i64 * ((other.y - self.y).abs() + 1) as i64
    }
}

fn dont_be_a_square(input: &str) -> (i64, i64) {
    let points: Vec<Point> = input.lines().map(|line| {
        let (x, y) = line.split_once(',').unwrap();
        Point {
            x: x.parse::<i32>().unwrap(),
            y: y.parse::<i32>().unwrap()
        }
    }).collect();

    // let point_set: HashSet<Point> = points.iter().cloned().collect();
    // let bounding_box: (Point, Point) = create_bounding_box(&points);
    // let mut max_green_rect = 0;

    let mut max_rect = 0;

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let p1 = &points[i];
            let p2 = &points[j];

            let rect = p1.rect(&p2);
            if rect > max_rect {
                max_rect = rect;
            }

            // let p3 = Point { x: p1.x, y: p2.y };
            // let p4 = Point { x: p2.x, y: p1.y };

            // I thought this would work...
            // just take any two points, create a rect and check if all four points
            // plus their edges and diagonals are inside the polygon.
            // max rect found by this is a bit too small, works on test though...
            // additionally it's slow as sin, way too many in_polygon checks.
            // if (point_set.contains(&p3) || is_in_polygon(&p3, &points, &bounding_box))
            //     && (point_set.contains(&p4) || is_in_polygon(&p4, &points, &bounding_box)) {
            //     if check_edges_and_diagonals(&p1, &p2, &p3, &p4, &points, &point_set, &bounding_box) {
            //         if rect > max_green_rect {
            //             max_green_rect = rect;
            //         }
            //     }
            // }
        }
    }

    let actual_green_rect = points
        .iter()
        .combinations(2)
        .par_bridge()
        .filter_map(|potential_rect| {
            let (p1, p2) = (potential_rect[0], potential_rect[1]);
            let size = p1.rect(&p2);
            let (x, y, u, v) = (
                p1.x.min(p2.x),
                p1.y.min(p2.y),
                p1.x.max(p2.x),
                p1.y.max(p2.y),
            );

            let mut overlaps = false;
            for i in 0..points.len() {
                // a polygon edge is a 1 wide rectangle between two adjacent points
                // an inside rectangle is not allowed to overlap/intersect with any of them
                // if one overlaps => left the polygon => break
                // if none break => rectangle overlaps no edge => inside => add size
                let polygon_edge = (points[i], points[(i + 1) % points.len()]);

                let (a, b, c, d) = (polygon_edge.0.x, polygon_edge.0.y, polygon_edge.1.x, polygon_edge.1.y);
                let (p, r) = (a.min(c), a.max(c));
                let (q, s) = (b.min(d), b.max(d));

                if x < r && u > p && y < s && v > q {
                    overlaps = true;
                    break;
                }
            }

            if !overlaps {
                Some(size)
            } else {
                None
            }
    }).max().unwrap_or(0);

    (max_rect, actual_green_rect)
}

#[allow(unused)]
fn check_edges_and_diagonals(p1: &Point, p2: &Point, p3: &Point, p4: &Point, polygon: &Vec<Point>, point_set: &HashSet<Point>, bbox: &(Point, Point)) -> bool {
    let min_x = p1.x.min(p2.x).min(p3.x).min(p4.x);
    let min_y = p1.y.min(p2.y).min(p3.y).min(p4.y);
    let max_x = p1.x.max(p2.x).max(p3.x).max(p4.x);
    let max_y = p1.y.max(p2.y).max(p3.y).max(p4.y);

    let mut x = min_x + 1;
    let mut y = min_y + 1;

    loop {
        if x == max_x || y == max_y {
            break;
        }
        let p = Point {x, y};
        if !is_in_polygon(&p, &polygon, &bbox) {
            return false;
        }
        x += 1;
        y += 1;
    }

    let mut x = min_x + 1;
    let mut y = max_y - 1;

    loop {
        if x == max_x || y == min_y {
            break;
        }
        let p = Point {x, y};
        if !is_in_polygon(&p, &polygon, &bbox) {
            return false;
        }
        x += 1;
        y -= 1;
    }

    // Check top edge
    for x in min_x + 1..max_x {
        let p = Point { x, y: min_y };
        if !point_set.contains(&p) && !is_in_polygon(&p, &polygon, &bbox) {
            return false;
        }
    }

    // Check bottom edge
    for x in min_x + 1..max_x {
        let p = Point { x, y: max_y };
        if !point_set.contains(&p) && !is_in_polygon(&p, &polygon, &bbox) {
            return false;
        }
    }

    // Check left edge
    for y in min_y + 1..max_y {
        let p = Point { x: min_x, y };
        if !point_set.contains(&p) && !is_in_polygon(&p, &polygon, &bbox) {
            return false;
        }
    }

    // Check right edge
    for y in min_y + 1..max_y {
        let p = Point { x: max_x, y };
        if !point_set.contains(&p) && !is_in_polygon(&p, &polygon, &bbox) {
            return false;
        }
    }

    true
}

#[allow(unused)]
fn create_bounding_box(points: &Vec<Point>) -> (Point, Point) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for v in points {
        min_x = min_x.min(v.x);
        min_y = min_y.min(v.y);
        max_x = max_x.max(v.x);
        max_y = max_y.max(v.y);
    }

    (Point{ x: min_x, y: min_y}, Point{x: max_x, y: max_y})
}

fn is_in_polygon(point: &Point, polygon: &Vec<Point>, bbox: &(Point, Point)) -> bool {
    if point.x < bbox.0.x || point.x > bbox.1.x ||
        point.y < bbox.0.y || point.y > bbox.1.y {
        return false;
    }
    let edge_count = polygon.len();
    let mut inside = false;

    // start with first and last point.
    let mut j = edge_count - 1;

    for i in 0..edge_count {
        let start = polygon[i];
        let end = polygon[j];
        if crosses_edge(point, start, end) {
            // odd crossings => inside, even crossings => outside, start with 1 = false
            inside = !inside;
        }
        j = i;
    }

    inside
}

fn crosses_edge(point: &Point, start: Point, end: Point) -> bool {
    ((start.y > point.y) != (end.y > point.y)) &&
        (point.x <= (end.x - start.x) * (point.y - start.y) / (end.y - start.y) + start.x)
}

#[cfg(test)]
mod tests {
    use crate::dont_be_a_square;

    #[test]
    fn test_input() {
        let input = include_str!("testinput");
        assert_eq!(dont_be_a_square(&input), (50, 24));
    }

    #[test]
    fn real_input() {
        let input = include_str!("input");
        assert_eq!(dont_be_a_square(&input), (4761736832, 1452422268));
    }
}
