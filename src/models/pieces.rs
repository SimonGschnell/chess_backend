use super::*;
#[derive(Clone, Debug)]
pub struct Pawn {
    pub did_move: bool,
    pub range: u8,
    pub color: Color,
}

impl Pawn {
    pub fn new(color: Color) -> Self {
        Pawn {
            did_move: false,
            range: 2,
            color,
        }
    }
}

impl Piece for Pawn {
    fn symbol(&self) -> &'static str {
        if let Color::Black = self.color {
            return chess_backend::BLACK_PAWN_SYMBOL;
        }
        chess_backend::WHITE_PAWN_SYMBOL
    }
    fn get_moves<'a>(
        &mut self,
        pos: &Position,
        db: &Board,
        lock: &'a MutexGuard<Matrix>,
    ) -> Vec<Position> {
        if self.did_move {
            self.range = 1;
        }
        let range = self.range;

        //? pawn moves differently based on its color
        let mut positions = Vec::new();
        //todo CHANGE BOUND TO 8 AFTER INCLUDING OTHER PIECES
        let rank_bound_max = 6;
        let rank_bound_min = 1;

        let files = pos.file..='h';
        let mut files = files.skip(1);
        let rev_files = 'a'..=pos.file;
        let mut rev_files = rev_files.rev().skip(1);
        match self.color {
            Color::Black => {
                let mut foreward = Vec::new();
                for i in 1..=range {
                    let rank = pos.rank - i;
                    if rank >= rank_bound_min {
                        let p = Position {
                            file: pos.file,
                            rank,
                        };
                        foreward.push(p);
                    }
                }
                if let Some(first) = foreward.get(0) {
                    if let None = db.is_piece_in_position(first, lock) {
                        positions.push(first.clone());
                        if let Some(second) = foreward.get(1) {
                            if let None = db.is_piece_in_position(second, lock) {
                                positions.push(second.clone());
                            }
                        }
                    }
                }
                let rank = pos.rank - 1;
                if rank >= rank_bound_min {
                    if let Some(positive_file) = files.next() {
                        let p = Position {
                            file: positive_file,
                            rank: rank,
                        };
                        if let Some(color) = db.is_piece_in_position(&p, lock) {
                            if self.color != color {
                                positions.push(p);
                            }
                        }
                    }
                    if let Some(negative_file) = rev_files.next() {
                        let p = Position {
                            file: negative_file,
                            rank: rank,
                        };
                        if let Some(color) = db.is_piece_in_position(&p, lock) {
                            if self.color != color {
                                positions.push(p);
                            }
                        }
                    }
                }
            }
            Color::White => {
                let mut foreward = Vec::new();
                for i in 1..=range {
                    let rank = pos.rank + i;
                    if rank <= rank_bound_max {
                        let p = Position {
                            file: pos.file,
                            rank,
                        };
                        foreward.push(p);
                    }
                }
                //? check if a piece stands in front of a pawn

                if let Some(first) = foreward.get(0) {
                    if let None = db.is_piece_in_position(first, lock) {
                        positions.push(first.clone());
                        if let Some(second) = foreward.get(1) {
                            if let None = db.is_piece_in_position(second, lock) {
                                positions.push(second.clone());
                            }
                        }
                    }
                }

                let rank = pos.rank + 1;
                if rank <= rank_bound_max {
                    if let Some(positive_file) = files.next() {
                        let p = Position {
                            file: positive_file,
                            rank: rank,
                        };
                        if let Some(color) = db.is_piece_in_position(&p, lock) {
                            if self.color != color {
                                positions.push(p);
                            }
                        }
                    }
                    if let Some(negative_file) = rev_files.next() {
                        let p = Position {
                            file: negative_file,
                            rank: rank,
                        };
                        if let Some(color) = db.is_piece_in_position(&p, lock) {
                            if self.color != color {
                                positions.push(p);
                            }
                        }
                    }
                }
            }
        }

        positions
    }
    fn get_color(&self) -> Color {
        self.color.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Rook {
    range: u8,
    color: Color,
}

impl Rook {
    pub fn new(color: Color) -> Self {
        Rook { color, range: 8 }
    }
}

impl Piece for Rook {
    fn symbol(&self) -> &'static str {
        if self.get_color() == Color::Black {
            chess_backend::BLACK_ROOK_SYMBOL
        } else {
            chess_backend::WHITE_ROOK_SYMBOL
        }
    }
    fn get_color(&self) -> Color {
        self.color.clone()
    }

    fn get_moves<'a>(
        &mut self,
        pos: &Position,
        db: &Board,
        lock: &'a MutexGuard<Matrix>,
    ) -> Vec<Position> {
        let range = self.range;
        //*example orthogonals of b6
        //*diagonals if range =1 are
        //? b5 & a6 & 6c

        let mut positions = Vec::new();
        //todo CHANGE BOUND TO 8 AFTER INCLUDING OTHER PIECES
        let rank_bound_max = 6;
        let rank_bound_min = 1;

        let files = pos.file..='h';
        let mut files = files.skip(1);
        let rev_files = 'a'..=pos.file;
        let mut rev_files = rev_files.rev().skip(1);
        for i in 1..=range {
            //?same rank
            if let Some(positive_file) = files.next() {
                positions.push(Position {
                    file: positive_file,
                    rank: pos.rank,
                });
            }
            if let Some(negative_file) = rev_files.next() {
                positions.push(Position {
                    file: negative_file,
                    rank: pos.rank,
                });
            }
            //?same file
            let positive_rank = pos.rank + i;
            //?u8 was panicing because it went lower than 0 when subtracting
            //?temporary fix with i8 conversion
            let negative_rank: i8 = (pos.rank as i8) - i as i8;
            if positive_rank <= rank_bound_max {
                positions.push(Position {
                    file: pos.file,
                    rank: positive_rank,
                });
            }
            if negative_rank >= rank_bound_min {
                positions.push(Position {
                    file: pos.file,
                    rank: negative_rank as u8,
                });
            }
        }
        positions
            .into_iter()
            .filter(|pos| !db.is_piece_in_position_of_same_color(pos, &self.color, lock))
            .collect()
    }
}

pub struct Knight {
    color: Color,
}

impl Knight {
    pub fn new(color: Color) -> Self {
        Knight { color }
    }
}
impl Piece for Knight {
    fn symbol(&self) -> &'static str {
        if self.get_color() == Color::Black {
            chess_backend::BLACK_KNIGHT_SYMBOL
        } else {
            chess_backend::WHITE_KNIGHT_SYMBOL
        }
    }
    fn get_color(&self) -> Color {
        self.color.clone()
    }
    fn get_moves<'a>(
        &mut self,
        pos: &Position,
        db: &Board,
        lock: &'a MutexGuard<Matrix>,
    ) -> Vec<Position> {
        let mut positions = Vec::new();
        //todo CHANGE BOUND TO 8 AFTER INCLUDING OTHER PIECES
        let rank_bound_max = 6;
        let rank_bound_min = 1;

        let files = pos.file..='h';
        let mut files = files.skip(1);
        let rev_files = 'a'..=pos.file;
        let mut rev_files = rev_files.rev().skip(1);

        let highest_rank = pos.rank + 2;
        let lowest_rank = pos.rank - 2;

        if let Some(positive_file) = files.next() {
            if highest_rank <= rank_bound_max {
                positions.push(Position {
                    file: positive_file,
                    rank: highest_rank,
                });
            }
            if lowest_rank >= rank_bound_min {
                positions.push(Position {
                    file: positive_file,
                    rank: lowest_rank,
                });
            }
        }

        if let Some(negative_file) = rev_files.next() {
            if highest_rank <= rank_bound_max {
                positions.push(Position {
                    file: negative_file,
                    rank: highest_rank,
                });
            }
            if lowest_rank >= rank_bound_min {
                positions.push(Position {
                    file: negative_file,
                    rank: lowest_rank,
                });
            }
        }
        let highest_rank = pos.rank + 1;
        let lowest_rank = pos.rank - 1;
        if let Some(highest_file) = files.next() {
            if lowest_rank >= rank_bound_min {
                positions.push(Position {
                    file: highest_file,
                    rank: lowest_rank,
                });
            }
            if highest_rank <= rank_bound_max {
                positions.push(Position {
                    file: highest_file,
                    rank: highest_rank,
                });
            }
        }

        if let Some(lowest_file) = rev_files.next() {
            if lowest_rank >= rank_bound_min {
                positions.push(Position {
                    file: lowest_file,
                    rank: lowest_rank,
                });
            }
            if highest_rank <= rank_bound_max {
                positions.push(Position {
                    file: lowest_file,
                    rank: highest_rank,
                });
            }
        }

        positions
            .into_iter()
            .filter(|pos| !db.is_piece_in_position_of_same_color(&pos, &self.color, lock))
            .collect()
    }
}
