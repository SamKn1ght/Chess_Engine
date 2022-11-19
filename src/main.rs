fn main() {
    // Create board
    let mut board: [[Tile; 8]; 8] = read_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

    initialise_window(&mut board);
}

fn read_fen_string(string : &str) -> [[Tile; 8]; 8] {
    let white_piece_codes: [u8; 6] = [75, 81, 82, 78, 66, 80]; // ascii codes for "KQRNBP"
    let black_piece_codes: [u8; 6] = [107, 113, 114, 110, 98, 112]; // ascii codes for "kqrnbp"
    let mut board : [[Tile; 8]; 8] = [[Tile {color : None, piece : None}; 8]; 8];
    let mut x_index : usize = 0;
    let mut y_index : usize = 0;
    let characters: &[u8] = string.as_bytes();
    for i in 0..characters.len() {
        // If character is a number
        if (characters[i] >= 48) & (characters[i] <= 57) {
            // In ASCII, 48 is 0, so use a -48 offset
            // In ASCII, 57 is 9, which will never be used
            x_index += (characters[i] - 48u8) as usize;
            continue;
        } else
        if characters[i] == 47 {
            y_index += 1;
            x_index  = 0;
            continue;
        } else
        if white_piece_codes.contains(&characters[i]) {
            board[x_index][y_index].color = Some(Colors::White);

            // Compares the character utf-8 code to the correct piece
            match characters[i] {
                75 => board[x_index][y_index].piece = Some(Pieces::King {has_moved : false}),
                81 => board[x_index][y_index].piece = Some(Pieces::Queen),
                82 => board[x_index][y_index].piece = Some(Pieces::Rook {has_moved : false}),
                78 => board[x_index][y_index].piece = Some(Pieces::Knight),
                66 => board[x_index][y_index].piece = Some(Pieces::Bishop),
                80 => board[x_index][y_index].piece = Some(Pieces::Pawn {has_moved : false, double_moved : false}),
                _ => () // This will never be reached
            }

            x_index += 1;
        } else
        if black_piece_codes.contains(&characters[i]) {
            board[x_index][y_index].color = Some(Colors::Black);

            // Compares the character utf-8 code to the correct piece
            match characters[i] {
                107 => board[x_index][y_index].piece = Some(Pieces::King {has_moved : false}),
                113 => board[x_index][y_index].piece = Some(Pieces::Queen),
                114 => board[x_index][y_index].piece = Some(Pieces::Rook {has_moved : false}),
                110 => board[x_index][y_index].piece = Some(Pieces::Knight),
                98  => board[x_index][y_index].piece = Some(Pieces::Bishop),
                112 => board[x_index][y_index].piece = Some(Pieces::Pawn {has_moved : false, double_moved : false}),
                _ => () // This will never be reached
            }

            x_index += 1;
        }
    }

    return board;
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    color : Option<Colors>,
    piece : Option<Pieces>
}
impl Tile {
    fn get_piece_image_index(&self) -> Option<usize> {
        let mut index : usize = 0;

        match self.color {
            Some(Colors::Black) => index += 6, // Uses the black piece indexes (2nd row)
            Some(Colors::White) => index += 0, // Uses the white piece indexes (1st row)
            None => return None, // No piece is in this Tile
        }

        match self.piece {
            Some(Pieces::King {..}) => index += 0, // 1st image in the row
            Some(Pieces::Queen) => index += 1, // 2nd image in the row
            Some(Pieces::Bishop) => index += 2, // 3rd image in the row
            Some(Pieces::Knight) => index += 3, // 4th image in the row
            Some(Pieces::Rook {..}) => index += 4, // 5th image in the row
            Some(Pieces::Pawn {..}) => index += 5, // 6th image in the row
            None => ()
        }

        return Some(index);
    } 
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Colors {
    White,
    Black
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Pieces {
    King {has_moved : bool},
    Queen,
    Rook {has_moved : bool},
    Knight,
    Bishop,
    Pawn {has_moved : bool, double_moved : bool}
}

fn generate_legal_tile_movements(board: &[[Tile; 8]; 8], x_index: usize, y_index: usize) -> Option<Vec<[usize; 2]>> {
    let tile: Tile = board[x_index][y_index];
    let mut legal_moves: Vec<[usize; 2]> = vec![];
    match tile.piece {
        None => return None,
        Some(_) => {
            // Allows for safe unwrapping of the tile
            let piece = tile.piece.unwrap();
            let color = tile.color.unwrap();
            match piece {

                Pieces::King { .. } => {
                    // This syntax allows for the king piece variants to be done
                    // inside of the general king piece
                    if let Pieces::King {has_moved : false} = piece {
                        // Evaluate castling moves
                        match color {
                            Colors::White => {
                                // King side castle
                                // Check for a rook on the king side, must be of the
                                // same colour as it has to have not moved
                                if let Some(Pieces::Rook { has_moved : false }) = board[7][7].piece {
                                    // Squares in the middle of the two pieces must be empty
                                    if board[5][7].piece == None && board[6][7].piece == None {
                                        legal_moves.push([x_index + 2, y_index + 0]);
                                    }
                                }
                                if let Some(Pieces::Rook { has_moved : false }) = board[0][7].piece {
                                    // Squares in the middle of the two pieces must be empty
                                    if board[1][7].piece == None && board[2][7].piece == None && board[3][7].piece == None {
                                        legal_moves.push([x_index - 2, y_index + 0]);
                                    }
                                }
                            },
                            Colors::Black => {
                                // King side castle

                            }
                        }
                        if x_index > 0 && board[x_index - 1][y_index].color != Some(color) {
                            legal_moves.push([x_index - 1, y_index + 0]);
                        }
                        if x_index < 7 && board[x_index + 1][y_index].color != Some(color) {
                            legal_moves.push([x_index + 1, y_index + 0]);
                        }
                        if y_index > 0  {
                            if board[x_index][y_index - 1].color != Some(color) {
                                legal_moves.push([x_index, y_index - 1]);
                            }
                            if x_index > 0 && board[x_index - 1][y_index - 1].color != Some(color) {
                                legal_moves.push([x_index - 1, y_index - 1]);
                            }
                            if x_index < 7 && board[x_index + 1][y_index - 1].color != Some(color) {
                                legal_moves.push([x_index + 1, y_index - 1]);
                            }
                        }
                        if y_index < 7 {
                            if board[x_index][y_index + 1].color != Some(color) {
                                legal_moves.push([x_index, y_index + 1]);
                            }
                            if x_index > 0 && board[x_index - 1][y_index + 1].color != Some(color) {
                                legal_moves.push([x_index - 1, y_index + 1]);
                            }
                            if x_index < 7 && board[x_index + 1][y_index + 1].color != Some(color) {
                                legal_moves.push([x_index + 1, y_index + 1]);
                            }
                        }
                    }
                },

                Pieces::Queen => {

                },

                Pieces::Rook { .. } => {

                },

                Pieces::Bishop => {

                },

                Pieces::Knight => {

                },

                Pieces::Pawn { .. } => {

                }
            }
        }
    }

    return Some(legal_moves);
}

// GUI Stuff

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::path::Path;

use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};

use glutin_window::GlutinWindow as Window;

use piston::input::{RenderArgs, RenderEvent};
use piston::{MouseCursorEvent, Button, MouseButton, PressEvent, Key};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;

use graphics::rectangle::square;
use graphics::Image;

fn initialise_window(board: &mut[[Tile; 8]; 8]) {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("Chess", [800, 800])
        .graphics_api(opengl)
        .build()
        .unwrap();

    let texture_settings: TextureSettings = TextureSettings::new();
    let piece_images: [Texture; 12] = [
        Texture::from_path(Path::new("./resources/WhiteKing.png"), &texture_settings).unwrap(),
        Texture::from_path(Path::new("./resources/WhiteQueen.png"), &texture_settings).unwrap(),
        Texture::from_path(Path::new("./resources/WhiteBishop.png"), &texture_settings).unwrap(),
        Texture::from_path(Path::new("./resources/WhiteKnight.png"), &texture_settings).unwrap(),
        Texture::from_path(Path::new("./resources/WhiteRook.png"), &texture_settings).unwrap(),
        Texture::from_path(Path::new("./resources/WhitePawn.png"), &texture_settings).unwrap(),
        Texture::from_path(Path::new("./resources/BlackKing.png"), &texture_settings).unwrap(),
        Texture::from_path(Path::new("./resources/BlackQueen.png"), &texture_settings).unwrap(),
        Texture::from_path(Path::new("./resources/BlackBishop.png"), &texture_settings).unwrap(),
        Texture::from_path(Path::new("./resources/BlackKnight.png"), &texture_settings).unwrap(),
        Texture::from_path(Path::new("./resources/BlackRook.png"), &texture_settings).unwrap(),
        Texture::from_path(Path::new("./resources/BlackPawn.png"), &texture_settings).unwrap()
    ];
    let mut image_locations : [[Image; 8]; 8] = [[Image::new(); 8]; 8];
    // Creates the places where images can be drawn
    for y in 0..8 {
        for x in 0..8 {
            image_locations[x][y] = Image::new()
                    .rect(square((x * 200) as f64, (y * 200) as f64, 200f64));
        }
    }
    
    let mut app : App = App {
        gl : GlGraphics::new(opengl),
        piece_images,
        image_locations,
        selected_tile : None
    };

    let mut mouse_position : [f64; 2] = [0f64, 0f64];
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args, &board, &mouse_position);
        }

        if let Some(mouse_rel) = e.mouse_cursor_args() {
            mouse_position = mouse_rel;
        }

        if let Some(button) = e.press_args() {
            if let Button::Mouse(MouseButton::Left) = button {
                let x_index = (mouse_position[0] / 100f64).floor();
                let y_index = (mouse_position[1] / 100f64).floor();
                app.update_selected_tile(x_index, y_index, board);
            }
            match button {
                Button::Mouse(MouseButton::Left) => {

                },
                Button::Keyboard(Key::Escape) => {
                    app.clear_selected_tile();
                },
                _ => ()
            }
        }
    }
}

struct App {
    gl : GlGraphics,
    piece_images : [Texture; 12],
    image_locations : [[Image; 8]; 8],
    selected_tile : Option<[usize; 2]>
}
impl App {
    fn render(&mut self, args: &RenderArgs, board : &[[Tile; 8]; 8], mouse_position : &[f64; 2]) {
        use graphics::*;

        // Color constants
        const BLACK  : [f32; 4] = [0.484f32, 0.582f32, 0.363f32, 1.00f32]; // (actually green)
        const WHITE  : [f32; 4] = [0.929f32, 0.929f32, 0.832f32, 1.00f32]; // (actually cream)
        const YELLOW : [f32; 4] = [0.871f32, 0.896f32, 0.375f32, 1.00f32];
        const PALE_YELLOW : [f32; 4] = [0.871f32, 0.896f32, 0.375f32, 0.50f32];
        const ORANGE : [f32; 4] = [0.770f32, 0.602f32, 0.426f32, 0.75f32];

        // Draw all needed elements
        self.gl.draw(args.viewport(), |c, gl| {
            // Fills the board black
            clear([0f32, 0f32, 0f32, 1f32], gl);

            // Draws the board squares
            let mut square : [f64; 4];
            for x in 0..8 {
                for y in 0..8 {
                    square = rectangle::square((x * 100) as f64, (y * 100) as f64, 100f64);
                    if y % 2 == x % 2 {
                        // Adds in the black squares
                        rectangle(WHITE, square, c.transform, gl);
                        continue;
                    } else {
                        rectangle(BLACK, square, c.transform, gl);
                    }
                }
            }

            // Highlights the selected tile
            if let Some(_) = self.selected_tile {
                // Get coordinates of the tile
                let x_position : f64 = (self.selected_tile.unwrap()[0] * 100) as f64;
                let y_position : f64 = (self.selected_tile.unwrap()[1] * 100) as f64;
                // Draws the square selected as yellow
                rectangle(YELLOW, rectangle::square(x_position, y_position, 100f64), c.transform, gl);
            }

            // Highlights the square under the mouse cursor
            rectangle(PALE_YELLOW, rectangle::square((mouse_position[0] / 100f64).floor() * 100f64, (mouse_position[1] / 100f64).floor() * 100f64, 100f64), c.transform, gl);

            for x in 0..8 {
                for y in 0..8 {
                    // Get the legal moves
                    let legal_moves = generate_legal_tile_movements(board, x, y);
                    if let Some(_) = legal_moves {
                        // Highlight the legal moves available
                        for i in legal_moves.unwrap() {
                            rectangle(ORANGE, rectangle::square((i[0] * 100) as f64, (i[1] * 100) as f64, 100f64), c.transform, gl);
                        }
                    }
                }
            }

            let mut image_index: Option<usize>;
            let draw_state : DrawState = DrawState::new_alpha();
            // Creates a matrix transformation to scale down the images
            let piece_transform = c.transform.scale(0.5, 0.5);
            for x in 0..8 {
                for y in 0..8 {
                    // Draw the piece images
                    image_index = board[x][y].get_piece_image_index();
                    if let Some(_) = image_index {
                        self.image_locations[x][y].draw(&self.piece_images[image_index.unwrap()], &draw_state, piece_transform, gl)
                    }
                }
            }
        });
    }

    fn update_selected_tile(&mut self, x_index : f64, y_index : f64, board : &mut[[Tile; 8]; 8]) {
        let y_index: usize = y_index as usize;
        let x_index: usize = x_index as usize;
        match self.selected_tile {
            Some(_) => {
                let current_tile = self.selected_tile.unwrap();
                match board[current_tile[0]][current_tile[1]].piece {
                    Some(_) => {
                        if board[current_tile[0]][current_tile[1]].color != board[x_index][y_index].color {
                            board[x_index][y_index] = board[current_tile[0]][current_tile[1]];
                            board[current_tile[0]][current_tile[1]].piece = None;
                            board[current_tile[0]][current_tile[1]].color = None;
                        }
                        self.clear_selected_tile();
                    },
                    None => {
                        self.selected_tile = Some([x_index, y_index]);
                    }
                }
            },
            None => {
                self.selected_tile = Some([x_index, y_index]);
            }
        }
    }

    fn clear_selected_tile(&mut self) {
        self.selected_tile = None;
    }

}