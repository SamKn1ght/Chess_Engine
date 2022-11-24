use std::cmp;

fn main() {
    // Create board
    let mut board: [Tile; 64] = read_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

    initialise_window(&mut board);
}

#[inline]
fn initialise_board() -> [Tile; 64] {
    let mut board: [Tile; 64] =
        [
            Tile {
                color: None,
                piece: None,
                tiles_left: 0,
                tiles_right: 0,
                tiles_down: 0,
                tiles_up: 0
            }
            ; 64
        ];
    for x in 0..8 {
        for y in 0..8 {
            board[get_array_index(x, y)].tiles_left = x;
            board[get_array_index(x, y)].tiles_right = 7 - x;
            board[get_array_index(x, y)].tiles_down = 7 - y;
            board[get_array_index(x, y)].tiles_up = y;
        }
    }
    return board;
}

#[inline]
fn read_fen_string(string : &str) -> [Tile; 64] {
    let white_piece_codes: [u8; 6] = [75, 81, 82, 78, 66, 80]; // ascii codes for "KQRNBP"
    let black_piece_codes: [u8; 6] = [107, 113, 114, 110, 98, 112]; // ascii codes for "kqrnbp"
    let mut board : [Tile; 64] = initialise_board();
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
            continue;
        } else
        if white_piece_codes.contains(&characters[i]) {
            board[index].color = Some(Colors::White);

            // Compares the character utf-8 code to the correct piece
            match characters[i] {
                75 => board[index].piece = Some(Pieces::King {has_moved : false}),
                81 => board[index].piece = Some(Pieces::Queen),
                82 => board[index].piece = Some(Pieces::Rook {has_moved : false}),
                78 => board[index].piece = Some(Pieces::Knight),
                66 => board[index].piece = Some(Pieces::Bishop),
                80 => board[index].piece = Some(Pieces::Pawn {has_moved : false, en_passantable : false}),
                _ => () // This will never be reached
            }

            index += 1;
        } else
        if black_piece_codes.contains(&characters[i]) {
            board[index].color = Some(Colors::Black);

            // Compares the character utf-8 code to the correct piece
            match characters[i] {
                107 => board[index].piece = Some(Pieces::King {has_moved : false}),
                113 => board[index].piece = Some(Pieces::Queen),
                114 => board[index].piece = Some(Pieces::Rook {has_moved : false}),
                110 => board[index].piece = Some(Pieces::Knight),
                98  => board[index].piece = Some(Pieces::Bishop),
                112 => board[index].piece = Some(Pieces::Pawn {has_moved : false, en_passantable : false}),
                _ => () // This will never be reached
            }

            index += 1;
        }
    }

    return board;
}

#[derive(Debug, Copy, Clone)]
struct Tile {
    color : Option<Colors>,
    piece : Option<Pieces>,
    tiles_left : usize,
    tiles_right : usize,
    tiles_up : usize,
    tiles_down : usize
}
impl Tile {
    fn get_piece_image_index(&self) -> Option<usize> {
        let mut index : usize = 0;

        match self.color {
            Some(Colors::Black) => index += 6, // Uses the black piece indexes (2nd row)
            Some(Colors::White) => index += 0, // Uses the white piece indexes (1st row)
            None => return None                // No piece is in this Tile
        }

        match self.piece {
            Some(Pieces::King { .. }) => index += 0, // 1st image in the row
            Some(Pieces::Queen) => index += 1,     // 2nd image in the row
            Some(Pieces::Bishop) => index += 2,    // 3rd image in the row
            Some(Pieces::Knight) => index += 3,    // 4th image in the row
            Some(Pieces::Rook { .. }) => index += 4, // 5th image in the row
            Some(Pieces::Pawn { .. }) => index += 5, // 6th image in the row
            None => return None
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
    Pawn {has_moved : bool, en_passantable : bool}
}

fn generate_legal_tile_movements(board: &[Tile; 64], index: usize) -> Option<Vec<usize>> {
    let tile: Tile = board[index];
    let mut legal_moves: Vec<usize> = vec![];
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
                    if let Pieces::King { has_moved : false } = piece {
                        // Evaluate castling moves
                        match color {
                            Colors::White => {
                                // King side castle
                                // Check for a rook on the king side, must be of the
                                // same colour as it has to have not moved
                                if let Some(Pieces::Rook { has_moved : false }) = board[0o77].piece {
                                    // Squares in the middle of the two pieces must be empty
                                    if board[0o76].piece == None && board[0o75].piece == None {
                                        legal_moves.push(index + 0o02);
                                    }
                                }
                                if let Some(Pieces::Rook { has_moved : false }) = board[0o70].piece {
                                    // Squares in the middle of the two pieces must be empty
                                    if board[0o71].piece == None && board[0o72].piece == None && board[0o73].piece == None {
                                        legal_moves.push(index - 0o02);
                                    }
                                }
                            },
                            Colors::Black => {
                                // King side castle
                                // Check for a rook on the king side, must be of the
                                // same colour as it has to have not moved
                                if let Some(Pieces::Rook { has_moved : false }) = board[0o07].piece {
                                    // Squares in the middle of the two pieces must be empty
                                    if board[0o05].piece == None && board[0o06].piece == None {
                                        legal_moves.push(index + 0o02);
                                    }
                                }
                                if let Some(Pieces::Rook { has_moved : false }) = board[0o00].piece {
                                    // Squares in the middle of the two pieces must be empty
                                    if board[0o01].piece == None && board[0o02].piece == None && board[0o03].piece == None {
                                        legal_moves.push(index - 0o02);
                                    }
                                }
                            }
                        }

                        if tile.tiles_left > 0 && board[index - 0o01].color != Some(color) {
                            legal_moves.push(index - 1);
                        }
                        if tile.tiles_right > 0 && board[index + 0o01].color != Some(color) {
                            legal_moves.push(index + 0o01);
                        }
                        if tile.tiles_up > 0 {
                            if board[index - 8].color != Some(color) {
                                legal_moves.push(index - 0o10);
                            }
                            if tile.tiles_left > 0 && board[index - 9].color != Some(color) {
                                legal_moves.push(index - 0o11);
                            }
                            if tile.tiles_right > 0 && board[index - 7].color != Some(color) {
                                legal_moves.push(index - 0o07);
                            }
                        }
                        if tile.tiles_down > 0 {
                            if board[index + 0o10].color != Some(color) {
                                legal_moves.push(index + 8);
                            }
                            if tile.tiles_left > 0 && board[index + 0o07].color != Some(color) {
                                legal_moves.push(index + 0o07);
                            }
                            if tile.tiles_right > 0 && board[index + 0o11].color != Some(color) {
                                legal_moves.push(index + 0o11);
                            }
                        }
                    }
                },

                Pieces::Queen => {
                    // Calculate moves right
                    for x in 0o01..tile.tiles_right + 0o01 {
                        match board[index + x].color {
                            None => { legal_moves.push(index + x); }
                            Some(_) => {
                                if board[index + x].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index + x);
                                }
                                break;
                            }
                        }
                    }
                    // Calculate moves left
                    for x in 0o01..tile.tiles_left + 0o01 {
                        match board[index - x].color {
                            None => { legal_moves.push(index - x); }
                            Some(_) => {
                                if board[index - x].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index - x);
                                }
                                break;
                            }
                        }
                    }
                    // Calculate moves down
                    for y in (0o10..tile.tiles_down * 0o10 + 0o10).step_by(0o10) {
                        match board[index + y].color {
                            None => { legal_moves.push(index + y); }
                            Some(_) => {
                                if board[index + y].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index + y);
                                }
                                break;
                            }
                        }
                    }
                    // Calculate moves up
                    for y in (0o10..tile.tiles_up * 0o10 + 0o10).step_by(0o10) {
                        match board[index - y].color {
                            None => { legal_moves.push(index - y); }
                            Some(_) => {
                                if board[index - y].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index - y);
                                }
                                break;
                            }
                        }
                    }
                    let max_up_left = cmp::min(tile.tiles_up, tile.tiles_left);
                    let max_down_left = cmp::min(tile.tiles_down, tile.tiles_left);
                    let max_up_right = cmp::min(tile.tiles_up, tile.tiles_right);
                    let max_down_right = cmp::min(tile.tiles_down, tile.tiles_right);
                    // Calculate moves up and left
                    for z in (0o11..max_up_left * 0o11 + 0o11).step_by(0o11) {
                        match board[index - z].color {
                            None => { legal_moves.push(index - z); }
                            Some(_) => {
                                if board[index - z].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index - z);
                                }
                                break;
                            }
                        }
                    }
                    // Calculates moves down and right
                    for z in (0o11..max_down_right * 0o11 + 0o11).step_by(0o11) {
                        match board[index + z].color {
                            None => { legal_moves.push(index + z); }
                            Some(_) => {
                                if board[index + z].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index + z);
                                }
                                break;
                            }
                        }
                    }
                    // Calculate moves up and right
                    for z in (0o07..max_up_right * 0o07 + 0o07).step_by(0o07) {
                        match board[index - z].color {
                            None => { legal_moves.push(index - z); }
                            Some(_) => {
                                if board[index - z].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index - z);
                                }
                                break;
                            }
                        }
                    }
                    // Calculates moves down and left
                    for z in (0o07..max_down_left * 0o07 + 0o07).step_by(0o07) {
                        match board[index + z].color {
                            None => { legal_moves.push(index + z); }
                            Some(_) => {
                                if board[index + z].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index + z);
                                }
                                break;
                            }
                        }
                    }
                },

                Pieces::Rook { .. } => {
                    // All castling mechanics are handled by the King piece
                    // Calculate moves right
                    for x in 0o01..tile.tiles_right + 0o01 {
                        match board[index + x].color {
                            None => { legal_moves.push(index + x); }
                            Some(_) => {
                                if board[index + x].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index + x);
                                }
                                break;
                            }
                        }
                    }
                    // Calculate moves left
                    for x in 0o01..tile.tiles_left + 0o01 {
                        match board[index - x].color {
                            None => { legal_moves.push(index - x); }
                            Some(_) => {
                                if board[index - x].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index - x);
                                }
                                break;
                            }
                        }
                    }
                    // Calculate moves down
                    for y in (0o10..tile.tiles_down * 0o10 + 0o10).step_by(0o10) {
                        match board[index + y].color {
                            None => { legal_moves.push(index + y); }
                            Some(_) => {
                                if board[index + y].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index + y);
                                }
                                break;
                            }
                        }
                    }
                    // Calculate moves up
                    for y in (0o10..tile.tiles_up * 0o10 + 0o10).step_by(0o10) {
                        match board[index - y].color {
                            None => { legal_moves.push(index - y); }
                            Some(_) => {
                                if board[index - y].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index - y);
                                }
                                break;
                            }
                        }
                    }
                },

                Pieces::Bishop => {
                    let max_up_left = cmp::min(tile.tiles_up, tile.tiles_left);
                    let max_down_left = cmp::min(tile.tiles_down, tile.tiles_left);
                    let max_up_right = cmp::min(tile.tiles_up, tile.tiles_right);
                    let max_down_right = cmp::min(tile.tiles_down, tile.tiles_right);
                    // Calculate moves up and left
                    for z in (0o11..max_up_left * 0o11 + 0o11).step_by(0o11) {
                        match board[index - z].color {
                            None => { legal_moves.push(index - z); }
                            Some(_) => {
                                if board[index - z].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index - z);
                                }
                                break;
                            }
                        }
                    }
                    // Calculates moves down and right
                    for z in (0o11..max_down_right * 0o11 + 0o11).step_by(0o11) {
                        match board[index + z].color {
                            None => { legal_moves.push(index + z); }
                            Some(_) => {
                                if board[index + z].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index + z);
                                }
                                break;
                            }
                        }
                    }
                    // Calculate moves up and right
                    for z in (0o07..max_up_right * 0o07 + 0o07).step_by(0o07) {
                        match board[index - z].color {
                            None => { legal_moves.push(index - z); }
                            Some(_) => {
                                if board[index - z].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index - z);
                                }
                                break;
                            }
                        }
                    }
                    // Calculates moves down and left
                    for z in (0o07..max_down_left * 0o07 + 0o07).step_by(0o07) {
                        match board[index + z].color {
                            None => { legal_moves.push(index + z); }
                            Some(_) => {
                                if board[index + z].color != Some(color) {
                                    // If opposing color add the take to legal moves
                                    legal_moves.push(index + z);
                                }
                                break;
                            }
                        }
                    }
                },

                Pieces::Knight => {
                    const OFFSETS: [usize; 4] = [0o06, 0o12, 0o17, 0o21];
                    let positive_condition_evals: [bool; 4] = [
                        tile.tiles_left >= 2 && tile.tiles_down >= 1,
                        tile.tiles_right >= 2 && tile.tiles_down >= 1,
                        tile.tiles_left >= 1 && tile.tiles_down >= 2,
                        tile.tiles_right >= 1 && tile.tiles_down >= 2
                    ];
                    let negative_condition_evals: [bool; 4] = [
                        tile.tiles_right >= 2 && tile.tiles_up >= 1,
                        tile.tiles_left >= 2 && tile.tiles_up >= 1,
                        tile.tiles_right >= 1 && tile.tiles_up >= 2,
                        tile.tiles_left >= 1 && tile.tiles_up >= 2
                    ];
                    for i in 0..4 {
                        if positive_condition_evals[i] {
                            match board[index + OFFSETS[i]].color {
                                None => { legal_moves.push(index + OFFSETS[i]); }
                                Some(_) => {
                                    if board[index + OFFSETS[i]].color != Some(color) {
                                        // If opposing color add the take to legal moves
                                        legal_moves.push(index + OFFSETS[i]);
                                    }
                                }
                            }
                        }
                        if negative_condition_evals[i] {
                            match board[index - OFFSETS[i]].color {
                                None => { legal_moves.push(index - OFFSETS[i]); }
                                Some(_) => {
                                    if board[index - OFFSETS[i]].color != Some(color) {
                                        // If opposing color add the take to legal moves
                                        legal_moves.push(index - OFFSETS[i]);
                                    }
                                }
                            }
                        }
                    }
                },

                Pieces::Pawn { .. } => {
                    let opposing_color = match color {
                        Colors::White => Colors::Black,
                        Colors::Black => Colors::White
                    };
                    // Pawn movements rely on the color for direction of movement
                    match color {
                        Colors::White => {
                            if tile.tiles_up >= 1 {
                                // Checks for a single move forwards
                                if board[index - 0o10].piece == None {
                                    legal_moves.push(index - 0o10);
                                    // Check eligibility for double move
                                    // This is enclosed here as a double move can only occur
                                    // If a single move can also occur
                                    if let Pieces::Pawn { has_moved: false, .. } = piece {
                                        // Pawns can only move forward into an empty space
                                        if tile.tiles_up >= 2 {
                                            if board[index - 0o20].piece == None {
                                                legal_moves.push(index - 0o20);
                                            }
                                        }
                                    }
                                }
                                if tile.tiles_left >= 1 && board[index - 0o01].piece == Some(Pieces::Pawn { has_moved: true, en_passantable: true }) {
                                    legal_moves.push(index - 0o11);
                                }
                                if tile.tiles_right >= 1 && board[index + 0o01].piece == Some(Pieces::Pawn { has_moved: true, en_passantable: true }) {
                                    legal_moves.push(index - 0o07)
                                }
                                if tile.tiles_left >= 1 && board[index - 0o11].color == Some(opposing_color) {
                                    legal_moves.push(index - 0o11);
                                }
                                if tile.tiles_right >= 1 && board[index - 0o07].color == Some(opposing_color) {
                                    legal_moves.push(index - 0o07);
                                }
                            }
                        },
                        Colors::Black => {
                            if tile.tiles_down >= 1 {
                                // Checks for a single move forwards
                                if board[index + 0o10].piece == None {
                                    legal_moves.push(index + 0o10);
                                    // Check eligibility for double move
                                    // This is enclosed here as a double move can only occur
                                    // If a single move can also occur
                                    if let Pieces::Pawn { has_moved: false, .. } = piece {
                                        // Pawns can only move forward into an empty space
                                        if tile.tiles_down >= 2 {
                                            if board[index + 16].piece == None {
                                                legal_moves.push(index + 0o20);
                                            }
                                            if tile.tiles_left >= 1 && board[index - 0o01].piece == Some(Pieces::Pawn { has_moved: true, en_passantable: true }) {
                                                legal_moves.push(index + 0o07);
                                            }
                                            if tile.tiles_right >= 1 && board[index + 0o01].piece == Some(Pieces::Pawn { has_moved: true, en_passantable: true }) {
                                                legal_moves.push(index + 0o11)
                                            }
                                        }
                                    }
                                }
                                if tile.tiles_left >= 1 && board[index + 0o07].color == Some(opposing_color) {
                                    legal_moves.push(index + 0o07);
                                }
                                if tile.tiles_right >= 1 && board[index + 0o11].color == Some(opposing_color) {
                                    legal_moves.push(index + 0o11);
                                }
                            }
                        }
                    }

                }
            }
        }
    }

    return Some(legal_moves);
}

fn play_move(board: &mut [Tile; 64], current: usize, new: usize) {
    board[new].piece = match board[current].piece {
        Some(Pieces::King { .. }) => Some(Pieces::King { has_moved: true }),
        Some(Pieces::Rook { .. }) => Some(Pieces::Rook { has_moved: true }),
        Some(Pieces::Pawn { .. }) => {
            let bigger = cmp::max(new, current);
            let smaller = cmp::min(new, current);
            if bigger - smaller == 0o20 {
                Some(Pieces::Pawn { has_moved: true, en_passantable: true })
            } else {
                Some(Pieces::Pawn { has_moved: true, en_passantable: false })
            }
        },
        _ => board[current].piece
    };
    board[new].color = board[current].color;
    board[current].piece = None;
    board[current].color = None;
}

#[inline]
fn get_render_coords(index : usize) -> [usize; 2] {
    let x = index  % 8;
    let y = index >> 3;
    [x, y]
}

#[inline]
fn get_array_index(x: usize, y: usize) -> usize {
    y * 8 + x
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

#[inline]
fn initialise_window(board: &mut[Tile; 64]) {
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
    selected_tile : Option<usize>
}
impl App {
    #[inline]
    fn render(&mut self, args: &RenderArgs, board : &[Tile; 64], mouse_position : &[f64; 2]) {
        use graphics::*;

        // Color constants
        const BLACK  : [f32; 4] = [0.484f32, 0.582f32, 0.363f32, 1.00f32]; // (actually green)
        const WHITE  : [f32; 4] = [0.929f32, 0.929f32, 0.832f32, 1.00f32]; // (actually cream)
        const YELLOW : [f32; 4] = [0.871f32, 0.896f32, 0.375f32, 1.00f32];
        const ORANGE : [f32; 4] = [0.770f32, 0.602f32, 0.426f32, 0.75f32];
        const PALE_YELLOW : [f32; 4] = [0.871f32, 0.896f32, 0.375f32, 0.50f32];

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
                let render_coords = get_render_coords(self.selected_tile.unwrap());
                let x_position : f64 = (render_coords[0] * 100) as f64;
                let y_position : f64 = (render_coords[1] * 100) as f64;
                // Draws the square selected as yellow
                rectangle(YELLOW, rectangle::square(x_position, y_position, 100f64), c.transform, gl);
            }

            // Highlights the square under the mouse cursor
            rectangle(PALE_YELLOW, rectangle::square((mouse_position[0] / 100f64).floor() * 100f64, (mouse_position[1] / 100f64).floor() * 100f64, 100f64), c.transform, gl);

            if let Some(_) = self.selected_tile {
                // Get the legal moves
                let legal_moves = generate_legal_tile_movements(board, self.selected_tile.unwrap());
                let mut draw_position: [usize; 2];
                if let Some(_) = legal_moves {
                    // Highlight the legal moves available
                    for i in legal_moves.unwrap() {
                        draw_position = get_render_coords(i);
                        rectangle(ORANGE, rectangle::square((draw_position[0] * 100) as f64, (draw_position[1] * 100) as f64, 100f64), c.transform, gl);
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
                    image_index = board[get_array_index(x, y)].get_piece_image_index();
                    if let Some(_) = image_index {
                        self.image_locations[x][y].draw(&self.piece_images[image_index.unwrap()], &draw_state, piece_transform, gl)
                    }
                }
            }
        });
    }

    #[inline]
    // Rework this function
    fn update_selected_tile(&mut self, x_index : f64, y_index : f64, board : &mut[Tile; 64]) {
        let new: usize = get_array_index(x_index as usize, y_index as usize);
        // New
        if let Some(_) = self.selected_tile {
            let current = self.selected_tile.unwrap();
            if let Some(_) = board[current].piece {
                let legal_moves = generate_legal_tile_movements(board, current);
                if let Some(_) = legal_moves {
                    let legal_moves = legal_moves.unwrap();
                    if legal_moves.contains(&new) {
                        play_move(board, current, new);
                    }
                }
                self.clear_selected_tile()
            } else {
                self.selected_tile = Some(new);
            }
        } else {
            self.selected_tile = Some(new);
        }
    }

    #[inline]
    fn clear_selected_tile(&mut self) {
        self.selected_tile = None;
    }

}