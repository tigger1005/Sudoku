#![allow(unused)]

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
    pub fn new(field: &str) -> Self {
        let mut temp = [[0; 9]; 9];
        for (i, c) in field.chars().enumerate() {
            temp[i % 9][i / 9] = 1 << ((c as u16 - '0' as u16) % 16);
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

    fn get_missing_set(&self, x: usize, y: usize) -> u16 {
        let inv_row = self.get_x_line(y);
        let inv_col = self.get_y_line(x);
        let inv_cel = self.get_cell(x, y);
        let sum = 0b0111111111 & inv_row & inv_col & inv_cel;
        sum & 0xfffe // Remove zero from list
    }

    fn solve_singles(&mut self) -> isize {
        let mut count: isize = 0;
        loop {
            count = 0;

            let mut changed = false;
            for x in 0..9 {
                for y in 0..9 {
                    if self.data[x][y] == 0x01 {
                        let set = self.get_missing_set(x, y);
                        let set_count = set.count_ones();
                        count += set_count as isize;
                        // Check if there is an error in the puzzle
                        if set_count == 0 {
                            return -1;
                        }
                        // Check if only one is missing
                        if set_count == 1 {
                            // Yes, solve
                            self.data[x][y] = set;
                            changed = true;
                        }
                    }
                }
            }
            if changed == false {
                break;
            }
        }
        // return count value : 0 sudoku solved / > 0 open fields / -1 wrong solution
        count
    }

    // Get best fitting set
    // if set is 0
    fn get_best_set(&mut self) -> (usize, usize, u16) {
        for x in 0..9 {
            for y in 0..9 {
                if self.data[x][y] == 0x01 {
                    let set = self.get_missing_set(x, y);
                    let set_count = set.count_ones();
                    // Check if there is an error in the puzzle
                    if set_count == 0 {
                        return (0, 0, 0);
                    }
                    // Check if only one is missing
                    if set_count == 1 {
                        // Yes, solve
                        return (x, y, set);
                    }
                }
            }
        }
        (0, 0, 0)
    }

    // Solve sudoku
    fn solve(&mut self) -> isize {
        // Solve all singles
        let ret = self.solve_singles();
        if ret > 0 {
            // Not all could be solved
        }
        ret
    }
}

fn main() {
    let game_str_2 =
        "605790301402180009907032080804325107090861042500907800079603400150470236006008975";

    let fd = Field::new(&game_str_2);
    println!("Sudoku Solver!");

    fd.print();
    println!("{:?}", fd.find_missing(1, 0));
}
