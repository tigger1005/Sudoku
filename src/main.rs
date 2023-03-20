#![allow(unused)]
use std::collections::HashSet;

trait PrintElement {
    fn print_element(&self) -> String;
}

impl PrintElement for u16 {
    fn print_element(&self) -> String {
        if *self != 1 {
            format!(" {:1x} ", self.trailing_zeros())
        } else {
            format!("   ")
        }
    }
}

struct Field {
    data: [[u16; 9]; 9],
}

impl Field {
    pub fn new() -> Self {
        let mut temp = [[0; 9]; 9];
        for x in 0..9 {
            for y in 0..9 {
                temp[x][y] = 1 << ((x + y * 9) as u16 % 16);
            }
        }

        Self { data: temp }
    }

    pub fn get_x_line(&self, y_line: usize) -> u16 {
        let mut temp: u16 = 0;
        for x in 0..9 {
            temp |= self.data[x][y_line];
        }
        temp
    }
    pub fn get_y_line(&self, x_line: usize) -> u16 {
        let mut temp: u16 = 0;
        for y in 0..9 {
            temp |= self.data[x_line][y];
        }
        temp
    }

    // Get cell
    // 0, 1, 2  [0..3][0..3], [3..6][0..3], [6..8][0..3]
    // 3, 4, 5  [0..3][3..6], [3..6][3..6], [6..8][3..6]
    // 6, 7, 8  [0..3][6..9], [3..6][6..9], [6..8][6..9]
    pub fn get_cell(&self, x: usize, y: usize) -> u16 {
        let mut temp: u16 = 0;
        let x_line = (x % 3) * 3;
        let y_line = (y % 3) * 3;

        for y in 0..3 {
            for x in 0..3 {
                temp |= self.data[x_line + x][y_line + y];
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

    fn find_missing(&self, x: usize, y: usize) {
        let inv_row = !self.get_x_line(x);
        let inv_col = !self.get_y_line(y);
        let inv_cel = !self.get_cell(x, y);
        let sum = 0b0111111111 & inv_row & inv_col & inv_cel;
    }
}

fn main() {
    let fd = Field::new();
    println!("Sudoku Solver!");

    fd.print();
    //fd.find_missing(0, 0);
}
