extern crate sdl2;
extern crate itertools;

mod game;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Point;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::gfx::primitives::DrawRenderer;

use itertools::Itertools;

use game::GameState;
use game::GridPos;
use game::Track;
use game::TrackEdge;

use std::time::Duration;


static LINE_DIST : u32 = 25;
static LINE_WEIGHT : u32 = 1;


fn main() {
    let grid_color = Color::RGB(130, 130, 130);
    let bg_color_light = Color::RGB(230, 230, 230);
    let bg_color = Color::RGB(199, 199, 199);
    let path_color = Color::RGB(0,0,150);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let mut game_state = GameState::new();
    sdl_context.mouse().show_cursor(false);

    let window = video_subsystem.window("Paperacers", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..  } => {
                    break 'running
                },
                Event::MouseButtonDown { x, y, mouse_btn: MouseButton::Left, .. } => {
                    game_state.update_state(nearest_game_pos(x, y));
                },
                _ => {}
            }
        }

        canvas.set_draw_color(bg_color);
        canvas.clear();

        let track = Track {
            out_edge: TrackEdge {
                xs: vec![200, 300, 350, 300, 200],
                ys: vec![200, 200, 350, 400, 400]
            },
            in_edge: TrackEdge {
                xs: vec![225, 250, 250, 225],
                ys: vec![225, 225, 300, 300]
            }
        };

        draw_track(&canvas, track, bg_color, bg_color_light);

        draw_grid(&canvas, grid_color);

        draw_path(&canvas, game_state.path.iter(), path_color);

        draw_next_move(&event_pump.mouse_state(), &canvas, &game_state, path_color);

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn draw_track(canvas: &sdl2::render::Canvas<sdl2::video::Window>, track: Track, center_color: Color, track_color: Color) -> () {
    canvas.filled_polygon(
        track.out_edge.xs.as_slice(),
        track.out_edge.ys.as_slice(),
        track_color
        );
    canvas.filled_polygon(
        track.in_edge.xs.as_slice(),
        track.in_edge.ys.as_slice(),
        center_color
        );
}

fn draw_path(canvas: &sdl2::render::Canvas<sdl2::video::Window>, path_iter: std::slice::Iter<GridPos>, color: Color) -> () {
    let drawing_points: Vec<Point> = path_iter.map(|game_point| {to_drawing_point(game_point)}).collect();
    for (start, end) in drawing_points.iter().tuple_windows() {
        canvas.thick_line(start.x as i16, start.y as i16, end.x as i16, end.y as i16, 2, color);
    }

    let cur_pos = drawing_points.last();
    match cur_pos {
        Some(pos) => {canvas.filled_circle(pos.x as i16, pos.y as i16, 3, color);},
        None => {}
    }
}

fn draw_grid(canvas: &sdl2::render::Canvas<sdl2::video::Window>, color: Color) -> () {
        let size = canvas.window().size();
        let height = size.0;
        let width = size.1;

        for n in 1..(height/LINE_DIST) {
            let x1 = (LINE_DIST*n) as i16;
            let x2 = x1 + LINE_WEIGHT as i16;
            let y1 = 0;
            let y2 = width as i16;
            canvas.rectangle(x1, y1, x2, y2, color);
        }

        for n in 1..(width/LINE_DIST) {
            let x1 = 0;
            let x2 = height as i16;
            let y1 = (LINE_DIST*n) as i16;
            let y2 = y1 + LINE_WEIGHT as i16;
            canvas.rectangle(x1, y1, x2, y2, color);
        }
}

fn draw_next_move(mouse: &sdl2::mouse::MouseState, canvas: &sdl2::render::Canvas<sdl2::video::Window>, game_state: &game::GameState, color: Color) -> () {
        let mouse_pos = nearest_game_pos(mouse.x(), mouse.y());
        let mdp = to_drawing_point(&mouse_pos);

        match game_state.valid_move(mouse_pos) {
            true => {
                canvas.circle(mdp.x as i16, mdp.y as i16, 3, color);
            },
            false => {
                canvas.circle(mdp.x as i16, mdp.y as i16, 3, Color::RGB(150,0,0));
            }
        }

}

fn nearest_game_pos(draw_x: i32, draw_y: i32) -> GridPos {
    GridPos {
        x: (draw_x as f32/LINE_DIST as f32).round() as i32,
        y: (draw_y as f32/LINE_DIST as f32).round() as i32
    }
}

fn to_drawing_point(game_point: &GridPos) -> Point {
    let drawing_x = game_point.x * LINE_DIST as i32;
    let drawing_y = game_point.y * LINE_DIST as i32;
    Point::new(drawing_x, drawing_y)
}
