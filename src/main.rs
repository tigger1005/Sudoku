//use std::collections::HashSet;

#[derive(Clone, Copy, Default)]
struct Element {
    el: u8,
}

impl Element {
    pub fn string(&self) -> String {
        if self.el != 0 {
            format!(" {:1x} ", self.el)
        } else {
            format!("   ")
        }
    }
}

struct Field {
    data: [[Element; 9]; 9],
}

impl Field {
    pub fn new() -> Self {
        Self {
            data: [[Element::default(); 9]; 9],
        }
    }

    // pub fn get_x_line(&self, x_line: usize) -> [u8; 9] {
    //     let mut temp: [u8; 9] = [0; 9];
    //     for x in 0..10 {
    //         temp[x] = self.data[x][x_line];
    //     }
    //     temp
    // }
    // pub fn get_y_line(&self, y_line: usize) -> [u8; 9] {
    //     self.data[y_line]
    // }

    // // Get cell. Cell started from 0 to 8 and from row/col = 0
    // pub fn get_cell(&self, cell: usize) -> [u8; 9] {
    //     let mut temp: [u8; 9] = [0; 9];
    //     let x_line = cell % 3;
    //     let y_line = cell / 3;

    //     for y in 0..4 {
    //         let slice = &self.data[y][y..y + 3];
    //         temp.copy_from_slice(slice);
    //     }
    //     temp
    // }

    pub fn print(&self) {
        println!("┌───┬───┬───┰───┬───┬───┰───┬───┬───┐");
        for x in 0..9 {
            for y in 0..9 {
                if y == 3 || y == 6 {
                    print!("┃{}", self.data[x][y].string());
                } else {
                    print!("│{}", self.data[x][y].string());
                }
            }
            println!("│");
            if x < 8 {
                if x == 2 || x == 5 {
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
}
