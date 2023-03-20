#![allow(unused)]
//use std::collections::HashSet;

trait PrintElement {
    fn print_element(&self) -> String;
}

impl PrintElement for u8 {
    fn print_element(&self) -> String {
        if *self != 0 {
            format!(" {:1x} ", *self)
        } else {
            format!("   ")
        }
    }
}

struct Field {
    data: [[u8; 9]; 9],
}

impl Field {
    pub fn new() -> Self {
        let mut temp = [[0; 9]; 9];
        for x in 0..9 {
            for y in 0..9 {
                temp[x][y] = (x + y * 9) as u8 % 16;
            }
        }

        Self { data: temp }
    }

    pub fn get_x_line(&self, x_line: usize) -> [u8; 9] {
        let mut temp: [u8; 9] = [0; 9];
        for x in 0..9 {
            temp[x] = self.data[x][x_line];
        }
        temp
    }
    pub fn get_y_line(&self, y_line: usize) -> [u8; 9] {
        self.data[y_line]
    }

    // Get cell. Cell started from 0 to 8 and from row/col = 0
    // 0, 1, 2  [0..3][0..3], [3..6][0..3], [6..8][0..3]
    // 3, 4, 5  [0..3][3..6], [3..6][3..6], [6..8][3..6]
    // 6, 7, 8  [0..3][6..9], [3..6][6..9], [6..8][6..9]
    pub fn get_cell(&self, cell: usize) -> [u8; 9] {
        let mut temp: [u8; 9] = [0; 9];
        let x_line = (cell % 3) * 3;
        let y_line = (cell / 3) * 3;

        for y in 0..3 {
            for x in 0..3 {
                temp[y * 3 + x] = self.data[x_line + x][y_line + y];
            }
        }
        temp
    }

    pub fn print(&self) {
        println!("┌───┬───┬───┰───┬───┬───┰───┬───┬───┐");
        for y in 0..9 {
            for x in 0..9 {
                if x == 3 || x == 6 {
                    print!("┃{}", self.data[x][y].print_element());
                } else {
                    print!("│{}", self.data[x][y].print_element());
                }
            }
            println!("│");
            if y < 8 {
                if y == 2 || y == 5 {
                    println!("┝━━━┿━━━┿━━━╋━━━┿━━━┿━━━╋━━━┿━━━┿━━━┥");
                } else {
                    println!("├───┼───┼───╂───┼───┼───╂───┼───┼───┤");
                }
            }
        }
        println!("└───┴───┴───┸───┴───┴───┸───┴───┴───┘");
    }
}

fn main() {
    let fd = Field::new();
    println!("Sudoku Solver!");

    fd.print();
    println!("{:?}", fd.get_cell(3));
}
