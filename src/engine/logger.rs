use crate::engine::pieces::{Piece, PieceKind};

#[derive(Clone)]
pub struct Logger {
    /// The log that contains all actions in a game.
    pub log: Vec<LogItem>,

    /// A boolean that is checked by the game to see if the logger has
    /// new action logs to be added to the game log UI.
    pub needs_update: bool
}

impl Logger {
    pub fn new() -> Self {
        Self {
            log: Vec::new(),
            needs_update: false
        }
    }

    pub fn add(&mut self, log: LogItem) {
        self.log.push(log);
        self.needs_update = true;
    }
}

#[derive(Clone)]
pub struct LogItem {
    pub capture: bool,
    pub piece: Piece,
    pub target_piece: Option<Piece>,
    pub start: (usize, usize),
    pub target: (usize, usize)
}

impl LogItem {
    pub fn is_castle(&self) -> bool {
        if self.piece.kind != PieceKind::King {
            return false
        }

        let (prev_x, prev_y) = self.start;
        let (target_x, target_y) = self.target;

        (prev_x == 4 && target_x == 2 )
        && (
            prev_y == 0 && target_y == 0
            || prev_y == 7 && target_y == 7
        )
    }
}