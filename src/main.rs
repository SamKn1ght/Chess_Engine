fn main() {
    // Create board
    let mut board: [Tile; 64] = read_fen_string("RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rbnkqbnr");

    initialise_window();
}

fn read_fen_string(string : &str) -> [Tile; 64] {
    let white_piece_codes: [u8; 6] = [75, 81, 82, 78, 66, 80]; // ascii codes for "KQRNBP"
    let black_piece_codes: [u8; 6] = [107, 113, 114, 110, 98, 112]; // ascii codes for "kqrnbp"
    let mut board : [Tile; 64] = [Tile {color : None, piece : None}; 64];
    let mut index : usize = 0;
    let characters: &[u8] = string.as_bytes();
    for i in 0..characters.len() {
        // If character is a number
        if (characters[i] >= 48) & (characters[i] <= 57) {
            // In ASCII, 48 is 0, so use a -48 offset
            // In ASCII, 57 is 9, which will never be used
            index += (characters[i] - 48u8) as usize;
            continue;
        } else
        if characters[i] == 47 {
            // Moves index to the next multiple of 8 when '/' character is found
            continue;
        } else
        if white_piece_codes.contains(&characters[i]) {
            board[index].color = Some(Colors::White);

            // Compares the character utf-8 code to the correct piece
            match characters[i] {
                75 => board[index].piece = Some(Pieces::King),
                81 => board[index].piece = Some(Pieces::Queen),
                82 => board[index].piece = Some(Pieces::Rook),
                78 => board[index].piece = Some(Pieces::Knight),
                66 => board[index].piece = Some(Pieces::Bishop),
                80 => board[index].piece = Some(Pieces::Pawn),
                _ => () // This will never be reached
            }

            index += 1;
        } else
        if black_piece_codes.contains(&characters[i]) {
            board[index].color = Some(Colors::Black);

            // Compares the character utf-8 code to the correct piece
            match characters[i] {
                107 => board[index].piece = Some(Pieces::King),
                113 => board[index].piece = Some(Pieces::Queen),
                114 => board[index].piece = Some(Pieces::Rook),
                110 => board[index].piece = Some(Pieces::Knight),
                98 => board[index].piece = Some(Pieces::Bishop),
                112 => board[index].piece = Some(Pieces::Pawn),
                _ => () // This will never be reached
            }

            index += 1;
        }
    }

    return board;
}

#[derive(Debug, Copy, Clone)]
pub struct Tile {
    color : Option<Colors>,
    piece : Option<Pieces>
}

#[derive(Debug, Copy, Clone)]
pub enum Colors {
    White,
    Black
}

#[derive(Debug, Copy, Clone)]
pub enum Pieces {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn
}

// GUI Stuff

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

pub fn initialise_window() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Chess", [800, 800])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();
    
    let mut app : App = App {
        gl : GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}

struct App {
    gl : GlGraphics
}
impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BLACK: [f32; 4] = [0.484f32, 0.582f32, 0.363f32, 1.00f32]; // Black square color (actually green)
        const WHITE: [f32; 4] = [0.929f32, 0.929f32, 0.832f32, 1.00f32]; // White square color (actually cream)

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);

            let mut square : [f64 ; 4];
            for x in 0..8 {
                for y in 0..8 {
                    if (y % 2 == x % 2) {continue;}
                    square = rectangle::square((x * 100) as f64, (y * 100) as f64, 100f64);
                    rectangle(BLACK, square, c.transform, gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        return;
    }

}