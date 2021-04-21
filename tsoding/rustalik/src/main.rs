mod board {
    use std::cmp::{max, min};
    use std::ops::Add;
    use std::ops::{Index, IndexMut, Range};

    pub type Col = usize;
    pub type Row = usize;

    #[derive(Clone, Copy)]
    pub struct Point(pub Col, pub Row);

    #[derive(Clone, Copy)]
    pub struct Delta(pub isize, pub isize);

    impl Add<Delta> for Point {
        type Output = Point;
        fn add(self, Delta(dx, dy): Delta) -> Self::Output {
            Point(
                (self.0 as isize + dx) as usize,
                (self.1 as isize + dy) as usize,
            )
        }
    }

    #[derive(Clone, Copy)]
    pub struct Rect(Point, Point);

    impl Rect {
        pub fn new(Point(col1, row1): Point, Point(col2, row2): Point) -> Self {
            Self(
                Point(min(col1, col2), min(row1, row2)),
                Point(max(col1, col2), max(row1, row2)),
            )
        }
        pub fn topLeft(&self) -> &Point {
            &self.0
        }
        pub fn bottomRight(&self) -> &Point {
            &self.1
        }
    }

    #[derive(Debug)]
    pub struct Board<T: Copy> {
        pub cols: usize,
        pub rows: usize,
        elems: Vec<T>,
    }

    impl<T: Copy> Board<T> {
        pub fn cols_range(&self) -> Range<Col> {
            0..self.cols
        }
        pub fn rows_range(&self) -> Range<Row> {
            0..self.rows
        }
        pub fn fill_rect(&mut self, rect: Rect, x: T) {
            let Rect(Point(col1, row1), Point(col2, row2)) = rect;
            for row in row1..=row2 {
                for col in col1..=col2 {
                    self[Point(col, row)] = x;
                }
            }
        }
        pub fn rect(&self) -> Rect {
            Rect(Point(0, 0), Point(self.cols - 1, self.rows - 1))
        }
        pub fn contains(&self, Point(col, row): Point) -> bool {
            (0..self.cols).contains(&col) && (0..self.rows).contains(&row)
        }
        pub fn get(&self, point: Point) -> Option<&T> {
            if self.contains(point) {
                Some(&self[point])
            } else {
                None
            }
        }
    }

    impl<T: Copy> Board<T> {
        pub fn new(cols: usize, rows: usize, x: T) -> Self {
            Self {
                cols: cols,
                rows: rows,
                elems: vec![x; cols * rows],
            }
        }
    }

    impl<T: Copy> Index<Point> for Board<T> {
        type Output = T;
        fn index(&self, Point(col, row): Point) -> &Self::Output {
            &self.elems[row * self.cols + col]
        }
    }

    impl<T: Copy> IndexMut<Point> for Board<T> {
        fn index_mut(&mut self, Point(col, row): Point) -> &mut Self::Output {
            &mut self.elems[row * self.cols + col]
        }
    }
}

#[derive(Clone, Copy)]
pub enum Cell {
    Empty,
    RoomFloor,
    VertWall,
    HorzWall,
    Passage,
    Door,
}

impl Cell {
    pub fn is_walkable(&self) -> bool {
        match self {
            Cell::Empty => false,
            Cell::RoomFloor => true,
            Cell::VertWall => false,
            Cell::HorzWall => false,
            Cell::Passage => true,
            Cell::Door => true,
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::VertWall => '|',
            Cell::RoomFloor => '.',
            Cell::HorzWall => '-',
            Cell::Passage => '#',
            Cell::Door => '+',
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Empty
    }
}

use crate::board::*;

#[derive(Copy, Clone)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl Direction {
    fn to_delta(self) -> Delta {
        match self {
            Self::N => Delta(0, -1),
            Self::S => Delta(0, 1),
            Self::E => Delta(1, 0),
            Self::W => Delta(-1, 0),
        }
    }
    fn from_key(key: char) -> Option<Self> {
        match key {
            'w' => Some(Self::N),
            's' => Some(Self::S),
            'a' => Some(Self::W),
            'd' => Some(Self::E),
            _ => None,
        }
    }
}

struct Rogalik {
    board: Board<Cell>,
    player_pos: Point,
    quit: bool,
}

impl Rogalik {
    fn new(cols: usize, rows: usize) -> Self {
        Self {
            board: Board::new(cols, rows, Cell::RoomFloor),
            player_pos: Point(0, 0),
            quit: false,
        }
    }
    fn render(&self, display: &mut Board<char>) {
        display.fill_rect(display.rect(), ' ');
        for row in self.board.rows_range() {
            for col in self.board.cols_range() {
                let point = Point(col, row);
                if display.contains(point) {
                    display[Point(col, row)] = self.board[point].to_char()
                }
            }
        }
        if display.contains(self.player_pos) {
            display[self.player_pos] = '@';
        }
    }
    fn move_to(&mut self, dir: Direction) {
        let next_pos = self.player_pos + dir.to_delta();
        if let Some(cell) = self.board.get(next_pos) {
            if cell.is_walkable() {
                self.player_pos = next_pos;
            }
        }
    }
    fn quit(&mut self) {
        self.quit = true;
    }
}

fn print_display(display: &Board<char>) {
    for row in display.rows_range() {
        for col in display.cols_range() {
            print!("{}", display[Point(col, row)])
        }
        println!("")
    }
}

fn main() {
    use std::io::{stdin, stdout, Write};

    const WIDTH: usize = 30;
    const HEIGHT: usize = 10;

    let mut display = Board::new(WIDTH, HEIGHT, ' ');
    let mut rogalik = Rogalik::new(WIDTH, HEIGHT);
    let mut line = String::new();

    rogalik.render(&mut display);
    print_display(&display);
    while !rogalik.quit {
        print!("> ");
        stdout().flush().unwrap();
        line.clear();
        stdin().read_line(&mut line).unwrap();
        for key in line.chars() {
            if let Some(dir) = Direction::from_key(key) {
                rogalik.move_to(dir)
            }
        }
        rogalik.render(&mut display);
        print_display(&display);
    }
}
