mod events;
mod render;
mod update_astar;
mod update_flowfield;

const X_LEN:usize = 256;
const Y_LEN:usize = 144;

const DENSITY:f64 = 5.0;

pub struct App {
    renderer: sdl2::render::WindowCanvas,
    input: sdl2::EventPump,
}

#[derive(Default)]
pub struct Unit{
    location: Point,
    direction: Point,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}

pub struct AStar {
    path: Option<Vec<Point>>,
}

impl AStar {
    pub fn new() -> AStar {
        AStar {
            path: None,
        }
    }
}

pub struct FlowField {
    pub inner: Vec<Vec<f64>>,
    pub computed: bool,
}

impl FlowField {
    pub fn new() -> FlowField {
        FlowField {
            inner: vec![vec![f64::MAX; Y_LEN]; X_LEN],
            computed: false,
        }
    }

    pub fn reset(&mut self) {
        for row in self.inner.iter_mut() {
            for length in row.iter_mut() {
                *length = f64::MAX;
            }
        }
    }
}

pub struct Map {
    inner: Vec<Vec<bool>>,
}

impl Map {
    fn new(x: usize, y: usize) -> Map {
        Map { inner: vec![vec![true; y]; x] }
    }
    fn reset(&mut self) {
        for row in self.inner.iter_mut() {
            for cell in row.iter_mut() {
                *cell = true;
            }
        }
    }
}

pub enum Algorithm {
    AStar(AStar),
    FlowField(FlowField),
}

impl Algorithm {
    pub fn reset_compute(&mut self) {
        match self {
            Algorithm::AStar(a_star) => {
                a_star.path = None;
            }
            Algorithm::FlowField(flow_field) => {
                flow_field.computed = false;
            }
        }
    }
}

pub struct GameState {
    algorithm: Algorithm,
    unit: Unit,
    mouse_position: (i32, i32),
    mouse_clicked: bool,
    adding_wall_on_click: bool,
    size_wall: i32,
    quit: bool,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            algorithm: Algorithm::AStar(AStar::new()),
            unit: Unit {location:Point{x:5, y:35}, direction:Point{x:5, y:35}},
            mouse_position: (0, 0),
            mouse_clicked: false,
            adding_wall_on_click: true,
            size_wall: 1,
            quit: false
        }
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("pathfinding", X_LEN as u32 * DENSITY as u32, Y_LEN as u32 * DENSITY as u32).build().unwrap();
    let canvas = window.into_canvas().build().unwrap();
    let input = sdl_context.event_pump().unwrap();

    let app = App {
        renderer: canvas,
        input: input,
    };
    let game_state = GameState::new();
    let map  = Map::new(X_LEN, Y_LEN);
    game_loop(app, map, game_state);
}

fn game_loop(mut app: App, mut map: Map, mut game_state:GameState) {
    while game_state.quit == false {
        std::thread::sleep(std::time::Duration::from_millis(10));
        events::poll_events(&mut app, &mut map, &mut game_state);
        match &mut game_state.algorithm {
            Algorithm::AStar(astar) => {
                update_astar::update(&map, &mut game_state.unit, astar);
            }
            Algorithm::FlowField(flow_field) => {
                update_flowfield::update(&map, &mut game_state.unit, flow_field)
            }
        }
        render::render(&mut app, &map, &game_state);
    }
}