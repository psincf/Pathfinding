use crate::Map;
use crate::Point;
use crate::Unit;
use crate::FlowField;

use crate::X_LEN;
use crate::Y_LEN;

pub fn update(map: &Map, unit: &mut Unit, flow_field: &mut FlowField) {
    if !flow_field.computed {
        compute_path(map, unit, flow_field);
    }
    advance_path(unit, flow_field);
    
}

fn advance_path(unit: &mut Unit, flow_field: &mut FlowField) {
    if unit.location == unit.direction { return }
    let location = unit.location.clone();
    let mut best_length = f64::MAX;
    let mut target = None;
    for x in (location.x - 1)..=(location.x + 1) {
        for y in (location.y - 1)..=(location.y + 1) {
            if x == 0 && y == 0 { continue }
            if x < 0 || x >= X_LEN as i32 || y < 0 || y >= Y_LEN as i32 { continue }
            let test_length = flow_field.inner[x as usize][y as usize];
            if test_length < best_length {
                target = Some(Point { x, y });
                best_length = test_length;
            }
        }
    }

    if let Some(target) = target {
        unit.location = target;
    }
}

fn compute_path(map: &Map, unit: &mut Unit, flow_field: &mut FlowField) {
    flow_field.reset();

    let mut vec_points = Vec::new();
    vec_points.push(unit.direction.clone());
    flow_field.inner[unit.direction.x as usize][unit.direction.y as usize] = 0.0;

    loop {
        let mut new_points = Vec::new();
        for point in vec_points {
            for x in (point.x - 1)..=(point.x + 1) {
                for y in (point.y - 1)..=(point.y + 1) {
                    if x == 0 && y == 0 { continue }
                    if x < 0 || x >= X_LEN as i32 || y < 0 || y >= Y_LEN as i32 { continue }
                    if !map.inner[x as usize][y as usize] { continue }

                    let length_to_add = if point.x == x || point.y == y {
                        1.0
                    } else {
                        2.0f64.sqrt()
                    };

                    let length = flow_field.inner[point.x as usize][point.y as usize] + length_to_add;

                    if length < flow_field.inner[x as usize][y as usize] {
                        flow_field.inner[x as usize][y as usize] = length;
                        new_points.push(Point { x, y });
                    }
                }
            }
        }

        vec_points = new_points;
        if vec_points.is_empty() { break }
    }

    flow_field.computed = true;
}