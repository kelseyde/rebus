use crate::board::{Board, Hand};
use crate::consts::{Piece, Side, Square};

impl Board {

    // Parses a Shogi position from an SFEN string.
    pub fn from_sfen(sfen: String) -> Result<Board, &'static str> {

        let mut board = Board::new();

        let parts = sfen.split_whitespace().collect::<Vec<&str>>();
        if parts.len() < 3 || parts.len() > 4 {
            return Err("SFEN has invalid number of parts");
        }

        let ranks = parts[0].split('/').collect::<Vec<&str>>();
        if ranks.len() != 9 {
            return Err("SFEN has invalid number of ranks");
        }

        for mut rank_idx in 0..9 {
            let rank = ranks[rank_idx];
            let mut file_idx = 0;
            let chars = rank.chars().collect::<Vec<char>>();

            for char_idx in 0..chars.len() {
                let char = chars[char_idx];
                if char.is_numeric() {
                    file_idx += char.to_digit(10).unwrap() as u8;
                }
                else if char.eq(&'+') {
                    if char_idx == chars.len() - 1 {
                        return Err("SFEN has invalid promotion character");
                    }
                    let piece_str = &rank[char_idx..char_idx + 2];
                    let piece_char = piece_str.chars().nth(1).unwrap();
                    let side = if piece_char.is_uppercase() { Side::Sente } else { Side::Gote };
                    let piece = Piece::from_str(piece_str);
                    if piece.is_some() {
                        let sq = Square::of(8 - rank_idx as u8, file_idx);
                        board.add_piece(side, piece.unwrap(), sq);
                        file_idx += 1;
                        rank_idx += 1;
                    } else {
                        return Err("SFEN has invalid piece character");
                    }
                } else {
                    let piece = Piece::from_str(&char.to_string());
                    let side = if char.is_uppercase() { Side::Sente } else { Side::Gote };
                    if piece.is_some() {
                        let sq = Square::of(8 - rank_idx as u8, file_idx);
                        board.add_piece(side, piece.unwrap(), sq);
                        file_idx += 1;
                    } else {
                        return Err("SFEN has invalid piece character");
                    }
                }
            }

            if file_idx != 9 {
                return Err("SFEN has invalid number of files");
            }

        }

        let stm_part = parts[1];
        if stm_part.len() != 1 {
            return Err("SFEN has invalid side to move");
        }
        match stm_part {
            "b" => board.set_stm(Side::Gote),
            "w" => board.set_stm(Side::Sente),
            _ => return Err("SFEN has invalid side to move")
        };

        let hand_part = parts[2];
        if hand_part != "-" {
            let chars = hand_part.chars().collect::<Vec<char>>();
            board.set_hand(Side::Sente, Hand::from_chars(&chars, Side::Sente));
            board.set_hand(Side::Gote, Hand::from_chars(&chars, Side::Gote));
        }

        if parts.len() == 4 {
            let moves_part = parts[3];
            let moves = moves_part.parse::<u8>();
            if moves.is_err() {
                return Err("SFEN has invalid number of moves");
            }
            board.set_moves(moves.unwrap());
        }

        Ok(board)

    }

    pub fn to_sfen(&self) -> String {
        let mut sfen = String::new();

        for rank in (0..9).rev() {
            for file in 0..9 {
                let sq = Square::of(rank, file);
                match self.piece_at(sq) {
                    Some(piece) => {
                        let side = self.side_at(sq).expect("Square should be occupied");
                        sfen.push_str(&piece.to_str(side));
                    },
                    None => {
                        let mut empty_squares = 0;
                        for i in 1..9 {
                            let next_sq = Square::of(rank, file + i);
                            if self.piece_at(next_sq).is_none() {
                                empty_squares += 1;
                            } else {
                                break;
                            }
                        }
                        sfen.push_str(&empty_squares.to_string());
                    }
                }
            }
            if rank > 0 {
                sfen.push('/');
            }
        }

        let stm = if self.stm().is_sente() { " w " } else { " b " };
        sfen.push_str(stm);

        let sente_hand = self.hand(Side::Sente);
        let gote_hand = self.hand(Side::Gote);
        if (sente_hand.is_empty() && gote_hand.is_empty()) {
            sfen.push_str("-");
        } else {
            sfen.push_str(&sente_hand.to_sfen(Side::Sente));
            sfen.push_str(&gote_hand.to_sfen(Side::Gote));
        }

        sfen.push(' ');

        sfen.push_str(&self.moves().to_string());

        sfen
    }

}

impl Hand {

    pub fn from_chars(chars: &Vec<char>, side: Side) -> Hand {
        let mut hand = Hand::new();
        let is_ours = if side.is_sente() { char::is_uppercase } else { char::is_lowercase };
        let our_chars: Vec<&char> = chars.iter().filter(|c| is_ours(**c)).collect();
        let our_pieces = our_chars.iter().map(|c| Piece::from_str(c.to_string().as_str()));
        for piece in our_pieces {
            if piece.is_some() {
                hand.add(piece.unwrap());
            }
        }
        hand
    }

    pub fn to_sfen(&self, side: Side) -> String {
        let mut sfen = String::new();
        for idx in self.pieces.iter() {
            let piece = Piece::from(*idx);
            let count = self.pieces[piece.idx()];
            let piece_str = piece.to_str(side);
            for _ in 0..count {
                sfen.push_str(piece_str.as_str());
            }
        }
        sfen
    }

}

impl Piece {

    pub fn from_str(s: &str) -> Option<Piece> {
        match s.to_uppercase().as_str() {
            "P" => Some(Piece::Pawn),
            "L" => Some(Piece::Lance),
            "N" => Some(Piece::Knight),
            "S" => Some(Piece::Silver),
            "G" => Some(Piece::Gold),
            "B" => Some(Piece::Bishop),
            "R" => Some(Piece::Rook),
            "+P" => Some(Piece::PromotedPawn),
            "+L" => Some(Piece::PromotedLance),
            "+N" => Some(Piece::PromotedKnight),
            "+S" => Some(Piece::PromotedSilver),
            "+B" => Some(Piece::PromotedBishop),
            "+R" => Some(Piece::PromotedRook),
            "K" => Some(Piece::King),
            _ => None
        }
    }

    pub fn to_str(&self, side: Side) -> String {
        let mut piece = match self {
            Piece::Pawn => "P",
            Piece::Lance => "L",
            Piece::Knight => "N",
            Piece::Silver => "S",
            Piece::Gold => "G",
            Piece::Bishop => "B",
            Piece::Rook => "R",
            Piece::PromotedPawn => "+P",
            Piece::PromotedLance => "+L",
            Piece::PromotedKnight => "+N",
            Piece::PromotedSilver => "+S",
            Piece::PromotedBishop => "+B",
            Piece::PromotedRook => "+R",
            Piece::King => "K"
        };
        if side == Side::Gote {
            piece.to_lowercase()
        } else {
            piece.to_string()
        }
    }

}

#[cfg(test)]
mod test {
    use crate::bits;
    use crate::board::Board;
    use crate::consts::Side;

    #[test]
    pub fn test_startpos() {
        let sfens: [&str; 1] = [
            "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1",
            //"lnsgk2nl/1r4gs1/p1pppp1pp/1p4p2/7P1/2P6/PP1PPPP1P/1SG4R1/LN2KGSNL b Bb"
        ];
        for sfen in sfens.iter() {
            match Board::from_sfen(sfen.to_string()) {
                Ok(board) => {
                    println!("{}", board.to_sfen());
                    assert_eq!(*sfen, board.to_sfen().as_str());
                },
                Err(e) => panic!("Error parsing SFEN: {}", e)
            }
        }
    }

}