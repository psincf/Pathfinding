use crate::AStar;
use crate::Map;
use crate::Point;
use crate::Unit;

use crate::X_LEN;
use crate::Y_LEN;

#[derive(Clone)]
struct PathPoint {
    previous_point : Point,
    length : f64,
    total_length : f64,
}

struct MatrixPoint {
    inner: Vec<Vec<PathPoint>>
}

impl MatrixPoint {
    fn new() -> MatrixPoint {
        let mut inner: Vec<Vec<PathPoint>> = Vec::new();
        for _x in 0..X_LEN {
            let mut matrix_y: Vec<PathPoint> = Vec::new();
            for _y in 0..Y_LEN {
                matrix_y.push(PathPoint { previous_point: Point{ x: 0, y: 0}, length: std::f64::MAX, total_length: std::f64::MAX });
            }
            inner.push(matrix_y);
        }

        MatrixPoint { inner }
    }
}

pub fn update(map: &Map, unit: &mut Unit, astar: &mut AStar) {
    if astar.path.is_none() {
        compute_path(map, unit, astar);
    }
    advance_path(unit, astar);
    
}

fn advance_path(unit: &mut Unit, astar: &mut AStar) {
    if let Some(path) = astar.path.as_mut() {
        unit.location = path.remove(0);
        if path.is_empty() { astar.path = None; }
    }
}

fn compute_path(map: &Map, unit: &mut Unit, astar: &mut AStar) {
    if unit.location == unit.direction { return; }
    let length = length_between(&unit.location, &unit.direction);
    let mut matrix_point = MatrixPoint::new();
    matrix_point.inner[unit.location.x as usize][unit.location.y as usize] = PathPoint { previous_point: Point{ x:0, y:0}, length: 0.0, total_length: length };

    let mut possibilities:Vec<Point> = Vec::new();
    possibilities.push(unit.location.clone());

    loop {
        if possibilities.len() == 0 { return; }
        let point:Point = possibilities.last().unwrap().clone();
        possibilities.pop();


        for x in -1..=1 {
            'outer: for y in -1..=1 {
                if x == 0 && y == 0 {continue}
                let new_point:Point = Point{x: point.x + x, y: point.y + y};
                if new_point.x < 0 || new_point.x >= X_LEN as i32 || new_point.y < 0 || new_point.y >= Y_LEN as i32 { continue }

                if map.inner[new_point.x as usize][new_point.y as usize] == false { continue }
                
                let mut length:f64;
                if x == 0 || y == 0 { length = 1.0f64; }
                else { length = 2.0f64.sqrt(); }

                length = matrix_point.inner[point.x as usize][point.y as usize].length + length;
                let total_length = length_between(&new_point, &unit.direction) + length;

                if matrix_point.inner[new_point.x as usize][new_point.y as usize].total_length <= total_length { continue; }

                matrix_point.inner[new_point.x as usize][new_point.y as usize] = PathPoint{previous_point: point.clone(), length: length, total_length: total_length };

                if new_point == unit.direction {
                    let mut temp_point:Point = new_point.clone();
                    let mut path = Vec::new();
                    loop {
                        path.insert(0, temp_point.clone());
                        temp_point = matrix_point.inner[temp_point.x as usize][temp_point.y as usize].previous_point.clone();
                        if temp_point == unit.location { astar.path = Some(path); return; }
                        
                    }
                }

                if possibilities.len() == 0 {
                    possibilities.push(new_point.clone());
                } else {
                    for index in (0..possibilities.len()).rev() {                 
                        if matrix_point.inner[new_point.x as usize][new_point.y as usize].total_length <= matrix_point.inner[possibilities[index].x as usize][possibilities[index].y as usize].total_length { possibilities.insert(index + 1, new_point.clone()); continue 'outer; }
                        if index == 0 { possibilities.insert(index, new_point.clone()); continue; }                    
                    }
                }
            }
        } 
    }
}

fn length_between(ref point1: &Point, ref point2: &Point) -> f64 {
    let delta_x:f64 = point1.x as f64 - point2.x as f64;
    let delta_y:f64 = point1.y as f64 - point2.y as f64;
    let length:f64 = (delta_x * delta_x + delta_y * delta_y).sqrt();
    return length;
}