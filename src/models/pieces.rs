use super::*;

const RANK_BOUND_MAX: u8 = 8;
const RANK_BOUND_MIN: u8 = 1;

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

        let files = pos.file..='h';
        let mut files = files.skip(1);
        let rev_files = 'a'..=pos.file;
        let mut rev_files = rev_files.rev().skip(1);
        match self.color {
            Color::Black => {
                for i in 1..=range {
                    let rank = pos.rank - i;
                    if rank >= RANK_BOUND_MIN {
                        let p = Position::new(pos.file, rank);
                        match db.is_piece_in_position(&p, lock) {
                            None => positions.push(p),
                            Some(_) => break,
                        }
                    }
                }

                let rank = pos.rank - 1;
                if rank >= RANK_BOUND_MIN {
                    if let Some(positive_file) = files.next() {
                        let p = Position::new(positive_file, rank);
                        if let Some(color) = db.is_piece_in_position(&p, lock) {
                            if self.color != color {
                                positions.push(p);
                            }
                        }
                    }
                    if let Some(negative_file) = rev_files.next() {
                        let p = Position::new(negative_file, rank);
                        if let Some(color) = db.is_piece_in_position(&p, lock) {
                            if self.color != color {
                                positions.push(p);
                            }
                        }
                    }
                }
            }
            Color::White => {
                for i in 1..=range {
                    let rank = pos.rank + i;
                    if rank <= RANK_BOUND_MAX {
                        let p = Position::new(pos.file, rank);
                        match db.is_piece_in_position(&p, lock) {
                            None => positions.push(p),
                            Some(_) => break,
                        }
                    }
                }

                let rank = pos.rank + 1;
                if rank <= RANK_BOUND_MAX {
                    if let Some(positive_file) = files.next() {
                        let p = Position::new(positive_file, rank);
                        if let Some(color) = db.is_piece_in_position(&p, lock) {
                            if self.color != color {
                                positions.push(p);
                            }
                        }
                    }
                    if let Some(negative_file) = rev_files.next() {
                        let p = Position::new(negative_file, rank);
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
    fn set_color(&mut self, color: Color) {
        self.color = color;
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
    fn set_color(&mut self, color: Color) {
        self.color = color;
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

        let files = pos.file..='h';
        let mut files = files.skip(1);
        let rev_files = 'a'..=pos.file;
        let mut rev_files = rev_files.rev().skip(1);
        for _ in 1..=range {
            //?same rank

            match files.next() {
                Some(positive_file) => {
                    let p = Position::new(positive_file, pos.rank);

                    if db.is_piece_in_position_of_same_color(&p, &self.color, lock) {
                        break;
                    }
                    positions.push(p);
                }
                None => break,
            }
        }
        for _ in 1..=range {
            match rev_files.next() {
                Some(negative_file) => {
                    let p = Position::new(negative_file, pos.rank);

                    if db.is_piece_in_position_of_same_color(&p, &self.color, lock) {
                        break;
                    }
                    positions.push(p);
                }
                None => break,
            }
        }
        for i in 1..=range {
            //?same file
            let positive_rank = pos.rank + i;
            //?u8 was panicing because it went lower than 0 when subtracting
            //?temporary fix with i8 conversion
            if positive_rank <= RANK_BOUND_MAX {
                let p = Position::new(pos.file, positive_rank);
                if db.is_piece_in_position_of_same_color(&p, &self.color, lock) {
                    break;
                }
                positions.push(p);
            }
        }
        for i in 1..=range {
            let negative_rank: i8 = (pos.rank as i8) - i as i8;
            if negative_rank >= RANK_BOUND_MIN as i8 {
                let p = Position::new(pos.file, negative_rank as u8);
                if db.is_piece_in_position_of_same_color(&p, &self.color, lock) {
                    break;
                }
                positions.push(p);
            }
        }

        positions
            .into_iter()
            .filter(|pos| !db.is_piece_in_position_of_same_color(pos, &self.color, lock))
            .collect()
    }
}

#[derive(Clone, Debug)]
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
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    fn get_moves<'a>(
        &mut self,
        pos: &Position,
        db: &Board,
        lock: &'a MutexGuard<Matrix>,
    ) -> Vec<Position> {
        let mut positions = Vec::new();

        let files = pos.file..='h';
        let mut files = files.skip(1);
        let rev_files = 'a'..=pos.file;
        let mut rev_files = rev_files.rev().skip(1);

        let highest_rank = pos.rank + 2;
        let lowest_rank = pos.rank as i8 - 2 as i8;

        if let Some(positive_file) = files.next() {
            if highest_rank <= RANK_BOUND_MAX {
                positions.push(Position::new(positive_file, highest_rank));
            }
            if lowest_rank >= RANK_BOUND_MIN as i8 {
                positions.push(Position::new(positive_file, lowest_rank as u8));
            }
        }

        if let Some(negative_file) = rev_files.next() {
            if highest_rank <= RANK_BOUND_MAX {
                positions.push(Position::new(negative_file, highest_rank));
            }
            if lowest_rank >= RANK_BOUND_MIN as i8 {
                positions.push(Position::new(negative_file, lowest_rank as u8));
            }
        }
        let highest_rank = pos.rank + 1;
        let lowest_rank = pos.rank as i8 - 1;
        if let Some(highest_file) = files.next() {
            if lowest_rank >= RANK_BOUND_MIN as i8 {
                positions.push(Position::new(highest_file, lowest_rank as u8));
            }
            if highest_rank <= RANK_BOUND_MAX {
                positions.push(Position::new(highest_file, highest_rank));
            }
        }

        if let Some(lowest_file) = rev_files.next() {
            if lowest_rank >= RANK_BOUND_MIN as i8 {
                positions.push(Position::new(lowest_file, lowest_rank as u8));
            }
            if highest_rank <= RANK_BOUND_MAX {
                positions.push(Position::new(lowest_file, highest_rank));
            }
        }

        positions
            .into_iter()
            .filter(|pos| !db.is_piece_in_position_of_same_color(&pos, &self.color, lock))
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct Bishop {
    color: Color,
    range: u8,
}

impl Bishop {
    pub fn new(color: Color) -> Self {
        Bishop { color, range: 8 }
    }
}

impl Piece for Bishop {
    fn symbol(&self) -> &'static str {
        match self.color {
            Color::Black => chess_backend::BLACK_BISHOP_SYMBOL,
            Color::White => chess_backend::WHITE_BISHOP_SYMBOL,
        }
    }
    fn get_color(&self) -> Color {
        self.color.clone()
    }
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    fn get_moves<'a>(
        &mut self,
        pos: &Position,
        db: &Board,
        lock: &'a MutexGuard<Matrix>,
    ) -> Vec<Position> {
        let range = self.range;
        //*example diagonals of b6
        //*diagonals if range =1 are
        //? a5 & c5

        let mut positions: Vec<Position> = Vec::new();

        let files = pos.file..='h';
        let mut files = files.skip(1);

        for i in 1..=range {
            let negative_rank = pos.rank as i8 - i as i8;

            match files.next() {
                Some(positive_file) => {
                    if negative_rank >= RANK_BOUND_MIN as i8 {
                        let p = Position::new(positive_file, negative_rank as u8);
                        if db.is_piece_in_position_of_same_color(&p, &self.color, lock) {
                            break;
                        }
                        positions.push(p);
                    }
                }
                None => break,
            }
        }
        let files2 = pos.file..='h';
        let mut files2 = files2.skip(1);

        for i in 1..=range {
            let positive_rank = pos.rank + i;

            match files2.next() {
                Some(positive_file) => {
                    if positive_rank <= RANK_BOUND_MAX {
                        let p = Position::new(positive_file, positive_rank);
                        if db.is_piece_in_position_of_same_color(&p, &self.color, lock) {
                            break;
                        }
                        positions.push(p);
                    }
                }
                None => break,
            }
        }

        let rev_files = 'a'..=pos.file;
        let mut rev_files = rev_files.rev().skip(1);

        for i in 1..=range {
            let positive_rank = pos.rank + i;
            match rev_files.next() {
                Some(negative_file) => {
                    if positive_rank <= RANK_BOUND_MAX {
                        let p = Position::new(negative_file, positive_rank);

                        if db.is_piece_in_position_of_same_color(&p, &self.color, lock) {
                            break;
                        }
                        positions.push(p);
                    }
                }
                None => break,
            }
        }

        let rev_files2 = 'a'..=pos.file;
        let mut rev_files2 = rev_files2.rev().skip(1);

        for i in 1..=range {
            let negative_rank = pos.rank as i8 - i as i8;
            match rev_files2.next() {
                Some(negative_file) => {
                    if negative_rank >= RANK_BOUND_MIN as i8 {
                        let p = Position::new(negative_file, negative_rank as u8);
                        if db.is_piece_in_position_of_same_color(&p, &self.color, lock) {
                            break;
                        }
                        positions.push(p);
                    };
                }
                None => break,
            }
        }

        positions
    }
}

#[derive(Debug, Clone)]
pub struct Queen {
    range: u8,
    color: Color,
}

impl Queen {
    pub fn new(color: Color) -> Self {
        Queen { range: 8, color }
    }
}
impl Piece for Queen {
    fn symbol(&self) -> &'static str {
        match self.color {
            Color::Black => chess_backend::BLACK_QUEEN_SYMBOL,
            Color::White => chess_backend::WHITE_QUEEN_SYMBOL,
        }
    }
    fn get_color(&self) -> Color {
        self.color.clone()
    }
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    fn get_moves<'a>(
        &mut self,
        pos: &Position,
        db: &Board,
        lock: &'a MutexGuard<Matrix>,
    ) -> Vec<Position> {
        let range = self.range;

        let mut positions = Vec::new();

        let files = pos.file..='h';
        let mut files = files.skip(1);
        let rev_files = 'a'..=pos.file;
        let mut rev_files = rev_files.rev().skip(1);
        let (mut diag_top, mut horizontal_right, mut diag_bot) = (true, true, true);
        for i in 1..=range {
            //?same rank

            match files.next() {
                Some(positive_file) => {
                    let higher_rank = if i > pos.rank { 0 } else { pos.rank - i };
                    let lower_rank = pos.rank + i;

                    let p_hor = Position::new(positive_file, pos.rank);

                    let p_diag_top = Position::new(positive_file, higher_rank);

                    let p_diag_bot = Position::new(positive_file, lower_rank);

                    if db.is_piece_in_position_of_same_color(&p_hor, &self.color, lock) {
                        horizontal_right = false;
                    }
                    if lower_rank <= RANK_BOUND_MAX
                        && db.is_piece_in_position_of_same_color(&p_diag_bot, &self.color, lock)
                    {
                        diag_bot = false;
                    }
                    if higher_rank >= RANK_BOUND_MIN
                        && db.is_piece_in_position_of_same_color(&p_diag_top, &self.color, lock)
                    {
                        diag_top = false;
                    }
                    if horizontal_right {
                        positions.push(p_hor);
                    }
                    if diag_top && higher_rank >= RANK_BOUND_MIN {
                        positions.push(p_diag_top);
                    }
                    if diag_bot && lower_rank <= RANK_BOUND_MAX {
                        positions.push(p_diag_bot);
                    }
                }
                None => break,
            }
        }
        let (mut diag_top, mut horizontal_left, mut diag_bot) = (true, true, true);

        for i in 1..=range {
            let higher_rank = if i > pos.rank { 0 } else { pos.rank - i };
            let lower_rank = pos.rank + i;

            match rev_files.next() {
                Some(negative_file) => {
                    let p_hor = Position::new(negative_file, pos.rank);
                    let p_diag_top = Position::new(negative_file, higher_rank);
                    let p_diag_bot = Position::new(negative_file, lower_rank);

                    if db.is_piece_in_position_of_same_color(&p_hor, &self.color, lock) {
                        horizontal_left = false;
                    }
                    if lower_rank <= RANK_BOUND_MAX
                        && db.is_piece_in_position_of_same_color(&p_diag_bot, &self.color, lock)
                    {
                        diag_bot = false;
                    }
                    if higher_rank >= RANK_BOUND_MIN
                        && db.is_piece_in_position_of_same_color(&p_diag_top, &self.color, lock)
                    {
                        diag_top = false;
                    }
                    if horizontal_left {
                        positions.push(p_hor);
                    }
                    if diag_top && higher_rank >= RANK_BOUND_MIN {
                        positions.push(p_diag_top);
                    }
                    if diag_bot && lower_rank <= RANK_BOUND_MAX {
                        positions.push(p_diag_bot);
                    }
                }
                None => break,
            }
        }
        for i in 1..=range {
            //?same file
            let positive_rank = pos.rank + i;
            //?u8 was panicing because it went lower than 0 when subtracting
            //?temporary fix with i8 conversion
            if positive_rank <= RANK_BOUND_MAX {
                let p = Position::new(pos.file, positive_rank);
                if db.is_piece_in_position_of_same_color(&p, &self.color, lock) {
                    break;
                }
                positions.push(p);
            }
        }
        for i in 1..=range {
            let negative_rank: i8 = (pos.rank as i8) - i as i8;
            if negative_rank >= RANK_BOUND_MIN as i8 {
                let p = Position::new(pos.file, negative_rank as u8);

                if db.is_piece_in_position_of_same_color(&p, &self.color, lock) {
                    break;
                }
                positions.push(p);
            }
        }

        positions
            .into_iter()
            .filter(|pos| !db.is_piece_in_position_of_same_color(pos, &self.color, lock))
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct King {
    range: u8,
    color: Color,
}
impl King {
    pub fn new(color: Color) -> Self {
        King { range: 1, color }
    }
}
impl Piece for King {
    fn symbol(&self) -> &'static str {
        match self.color {
            Color::Black => chess_backend::BLACK_KING_SYMBOL,
            Color::White => chess_backend::WHITE_KING_SYMBOL,
        }
    }
    fn get_color(&self) -> Color {
        self.color.clone()
    }
    fn set_color(&mut self, color: Color) {
        self.color = color;
    }
    fn get_moves<'a>(
        &mut self,
        pos: &Position,
        db: &Board,
        lock: &'a MutexGuard<Matrix>,
    ) -> Vec<Position> {
        let files = pos.file..='h';
        let mut files = files.skip(1);
        let rev_files = 'a'..=pos.file;
        let mut rev_files = rev_files.rev().skip(1);
        let mut positions = Vec::new();

        let positive_rank = pos.rank + self.range;
        let negative_rank: i8 = (pos.rank as i8) - self.range as i8;
        if let Some(positive_file) = files.next() {
            let (p_hor, diag_top, diag_bot) = (
                Position::new(positive_file, pos.rank),
                Position::new(positive_file, negative_rank as u8),
                Position::new(positive_file, positive_rank),
            );
            positions.push(p_hor);
            if positive_rank <= RANK_BOUND_MAX {
                positions.push(diag_bot);
            }
            if negative_rank >= RANK_BOUND_MIN as i8 {
                positions.push(diag_top);
            }
        }
        if let Some(negative_file) = rev_files.next() {
            let (p_hor, diag_top, diag_bot) = (
                Position::new(negative_file, pos.rank),
                Position::new(negative_file, negative_rank as u8),
                Position::new(negative_file, positive_rank),
            );
            positions.push(p_hor);
            if positive_rank <= RANK_BOUND_MAX {
                positions.push(diag_bot);
            }
            if negative_rank >= RANK_BOUND_MIN as i8 {
                positions.push(diag_top);
            }
        }
        if positive_rank <= RANK_BOUND_MAX {
            positions.push(Position::new(pos.file, positive_rank));
        }
        if negative_rank >= RANK_BOUND_MIN as i8 {
            positions.push(Position::new(pos.file, negative_rank as u8));
        }

        let positions = positions
            .into_iter()
            .filter(|p| !db.is_piece_in_position_of_same_color(&p, &self.color, lock))
            .collect();

        positions
    }
}
