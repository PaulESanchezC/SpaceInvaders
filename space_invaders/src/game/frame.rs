use super::{COLUMNS, ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut columns: Vec<Vec<&str>> = Vec::with_capacity(COLUMNS);
    for _ in 0..COLUMNS {
        let mut col: Vec<&str> = Vec::with_capacity(ROWS);
        for _ in 0..ROWS {
            col.push(" ");
        }
        columns.push(col);
    }
    columns
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
