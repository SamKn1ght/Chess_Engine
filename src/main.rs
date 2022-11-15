fn main() {
    // Create board
    let mut board: [Tile; 64] = read_fen_string("RNBQKBNR/PPPPPPPP/8/8/8/8/pppppppp/rbnkqbnr");
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
struct Tile {
    color : Option<Colors>,
    piece : Option<Pieces>
}

#[derive(Debug, Copy, Clone)]
enum Colors {
    White,
    Black
}

#[derive(Debug, Copy, Clone)]
enum Pieces {
    King,
    Queen,
    Rook,
    Knight,
    Bishop,
    Pawn
}
