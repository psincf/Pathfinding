use crate::Algorithm;
use crate::App;
use crate::AStar;
use crate::FlowField;
use crate::GameState;
use crate::Map;

use crate::X_LEN;
use crate::Y_LEN;
use crate::DENSITY;

pub fn render(app: &mut App, map: &Map, game_state: &GameState) {
    app.renderer.set_draw_color((125, 125, 125, 255));
    app.renderer.clear();
    app.renderer.set_draw_color((200, 200, 200, 255));
    app.renderer.fill_rect(
        sdl2::rect::Rect::new(0, 0, X_LEN as u32 * DENSITY as u32, Y_LEN as u32 * DENSITY as u32)
    ).unwrap();
    app.renderer.set_draw_color((0, 0, 0, 255));
    let mut vec_rect:Vec<sdl2::rect::Rect> = Vec::new();
    let mut index_x:u32 = 0;
    for matrix_y in map.inner.iter() {                    
        let mut index_y:u32 = 0;
        for wall in matrix_y {
            match wall {
                true => { } ,
                false => { vec_rect.push(sdl2::rect::Rect::new(DENSITY as i32 * index_x as i32, DENSITY as i32 * index_y as i32, DENSITY as u32, DENSITY as u32 )) } ,
            };
            index_y = index_y + 1
        }
        index_x = index_x + 1;
    }
    if vec_rect.len() !=0 {
        app.renderer.fill_rects(&vec_rect).unwrap();
    }
    
    match &game_state.algorithm {
        Algorithm::AStar(astar) => {
            render_astar(app, &astar);
        },
        Algorithm::FlowField(flow_field) => {
            render_flow_field(app, &flow_field);
        }
    }

    let unit = &game_state.unit;
    app.renderer.set_draw_color((255, 0, 0, 255));
    app.renderer.fill_rect(
        sdl2::rect::Rect::new(DENSITY as i32 * unit.direction.x as i32, DENSITY as i32 * unit.direction.y as i32, DENSITY as u32, DENSITY as u32)
    ).unwrap();
    app.renderer.set_draw_color((0, 0, 0, 255));
    app.renderer.fill_rect(
        sdl2::rect::Rect::new(DENSITY as i32 * unit.location.x as i32, DENSITY as i32 * unit.location.y as i32, DENSITY as u32, DENSITY as u32)
    ).unwrap();


    app.renderer.present();

}

fn render_astar(app: &mut App, astar: &AStar) {
    if let Some(path) = astar.path.as_ref() {
        for point in path.iter() {
            app.renderer.set_draw_color((255, 0, 255, 255));
            app.renderer.fill_rect(
                sdl2::rect::Rect::new(DENSITY as i32 * point.x as i32, DENSITY as i32 * point.y as i32, DENSITY as u32, DENSITY as u32)
            ).unwrap();
        }
    }
}

fn render_flow_field(app: &mut App, flow_field: &FlowField) {
    for (x, row) in flow_field.inner.iter().enumerate() {
        for (y, field) in row.iter().enumerate() {
            if *field == f64::MAX { continue }
            let color_intensity = ((*field as i32) % 512).min(255) - (((*field as i32) % 512) - 256).max(0);
            app.renderer.set_draw_color((100, color_intensity as u8, 100, 255));
            app.renderer.fill_rect(
                sdl2::rect::Rect::new(DENSITY as i32 * x as i32, DENSITY as i32 * y as i32, DENSITY as u32, DENSITY as u32)
            ).unwrap();
        }
    }
}