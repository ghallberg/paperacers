pub struct GamePos {
}

// move convertion from GridPos here
// Handle converting to DrawingPos in view
//

pub struct TrackEdge {
    pub xs: Vec<i16>,
    pub ys: Vec<i16>
}


pub struct Track {
    pub in_edge: TrackEdge,
    pub out_edge: TrackEdge
}

#[derive(PartialEq, Debug, Eq, Copy, Clone)]
pub struct GridPos {
    pub x: i32,
    pub y: i32
}

impl GridPos {
    fn is_adjacent(&self, other: GridPos) -> bool {
        let delta = (self.x-other.x, self.y-other.y);
        match delta.0 {
            1 | -1 => {
                match delta.1 {
                    1 | -1 | 0 => true,
                    _ => false
                }
            },
            0 => {
                match delta.1 {
                    1 | -1 => true,
                    _ => false
                }
            },
            _ => false
        }
    }

    fn is_adjacent_or_equal(&self, other: GridPos) -> bool {
        *self == other || self.is_adjacent(other)
    }

}


#[derive(Debug)]
pub struct GameState {
    pub path: Vec<GridPos>,
}


impl GameState {
    pub fn new() -> GameState {
        GameState { path: Vec::new() }
    }

    pub fn update_state (&mut self, new_pos: GridPos) -> () {
        if self.valid_move(new_pos) {
            self.path.push(new_pos);
        }
    }

    pub fn valid_move (&self, new_pos: GridPos) -> bool{
        let mut last_move = self.path.iter().rev().take(2);

        match last_move.next() {
            Some(end) => {
                match last_move.next() {
                    Some(start) => new_pos.is_adjacent_or_equal(GameState::extrapolate_trajectory(*start, *end)),
                    None => end.is_adjacent(new_pos)
                }
            },
            None => GameState::is_on_starting_line(new_pos)
        }
    }

    fn is_on_starting_line(pos: GridPos) -> bool {
        true
    }

    fn extrapolate_trajectory(start_pos: GridPos, end_pos: GridPos) -> GridPos {
        let delta = (end_pos.x-start_pos.x, end_pos.y-start_pos.y);
        GridPos { x: end_pos.x+delta.0, y: end_pos.y+delta.1 }
    }
}

