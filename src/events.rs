use crate::Algorithm;
use crate::App;
use crate::AStar;
use crate::FlowField;
use crate::GameState;
use crate::Map;

use crate::X_LEN;
use crate::Y_LEN;
use crate::DENSITY;

pub fn poll_events(app: &mut App, map: &mut Map, game_state: &mut GameState) {
    while let Some(event) = app.input.poll_event() {
        use sdl2::event::Event;
        match event {
            Event::Quit{ timestamp: _} => {
                game_state.quit = true;
            }
            Event::KeyDown{ keycode, ..} => {
                use sdl2::keyboard::Keycode;
                match keycode {
                    Some(Keycode::Escape) => {
                        game_state.quit = true;
                    }
                    /*
                    Some(Keycode::C) => {
                        game_state.adding_wall_on_click = !game_state.adding_wall_on_click;
                        let text = if !game_state.adding_wall_on_click { "Add" } else { "Remove" };
                        println!("Left click set to : {}", text);
                    }
                    */

                    Some(Keycode::R) => {
                        map.reset();
                        game_state.algorithm.reset_compute();
                    }
                    Some(Keycode::V) => {
                        let mut astar= false;
                        let mut flow_field= false;
                        match game_state.algorithm {
                            Algorithm::AStar(_) => {
                                astar = true;
                            },
                            Algorithm::FlowField(_) => {
                                flow_field = true;
                            }
                        }
                        if astar { game_state.algorithm = Algorithm::FlowField(FlowField::new()) }
                        if flow_field { game_state.algorithm = Algorithm::AStar(AStar::new()) }
                    }
                    Some(Keycode::KpPlus) => {
                        game_state.size_wall += 1;
                        println!("radius = {:?}", game_state.size_wall);
                    }
                    Some(Keycode::KpMinus) => {
                        game_state.size_wall -= 1;
                        if game_state.size_wall <= 0 { game_state.size_wall = 1; }
                        println!("radius = {:?}", game_state.size_wall);
                    }
                    _ => {}
                }
            }
            Event::MouseButtonDown{ mouse_btn, .. } => {
                use sdl2::mouse::MouseButton;
                if mouse_btn == MouseButton::Left {
                    game_state.mouse_clicked = true;
                    game_state.algorithm.reset_compute();
                    put_wall(map, &game_state);
                } else if mouse_btn == MouseButton::Right {
                    let x:usize = (game_state.mouse_position.0 as f64 / DENSITY) as usize;
                    let y:usize = (game_state.mouse_position.1 as f64 / DENSITY) as usize;
                    if x<X_LEN && y<Y_LEN {
                        game_state.unit.direction.x = x as i32;
                        game_state.unit.direction.y = y as i32;
                        
                        match &mut game_state.algorithm {
                            Algorithm::AStar(a_star) => {
                                a_star.path = None;
                            }
                            Algorithm::FlowField(flow_field) => {
                                flow_field.computed = false;
                            }
                        }
                    }
                }
            }
            Event::MouseButtonUp{ mouse_btn, .. } => {
                use sdl2::mouse::MouseButton;
                if mouse_btn == MouseButton::Left {
                    game_state.mouse_clicked = false;
                }
            }
            Event::MouseMotion{ x, y, .. } => {
                game_state.mouse_position = (x, y);
                if game_state.mouse_clicked {
                    game_state.algorithm.reset_compute();
                    put_wall(map, &game_state);
                }
            }
            Event::MouseWheel{y, ..} => {
                game_state.size_wall = game_state.size_wall + y;
                if game_state.size_wall <= 0 { game_state.size_wall = 1; }
                println!("radius = {:?}", game_state.size_wall);
            }
            _ => {}
        }
    }
}

fn put_wall(map: &mut Map, game_state: &GameState) {
    let x = (game_state.mouse_position.0 as f64 / DENSITY) as i32;
    let y = (game_state.mouse_position.1 as f64 / DENSITY) as i32;
    for x in (x - game_state.size_wall as i32)..(x + game_state.size_wall as i32) {
        for y in (y - game_state.size_wall as i32)..(y + game_state.size_wall as i32) {
            if x<X_LEN as i32 && y<Y_LEN as i32 && x>=0 && y>=0 { 
                map.inner[x as usize][y as usize] = !game_state.adding_wall_on_click;        
            }
        }
    }
}