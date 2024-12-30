pub mod ghost {
    use crate::game::{MAZE_SIZE, Maze};

    #[derive(Clone, Copy)]
    pub struct Ghost {
        p: usize, // position
        d: char,  // direction
        t: char,  // type a or b
        maze: Maze, // Maze
    }
    impl Ghost {
        pub fn new(_p:usize, _t:char, maze: &Maze) -> Ghost {
            Ghost {
                p: _p,
                d: 'n',
                t: _t,
                maze: *maze,
            }
        }

        pub fn get_position(&self) -> usize {
            self.p
        }
        pub fn get_type(&self) -> char {
            self.t
        }

        pub fn update(self) -> Ghost{
            match self.d {
                'n' => {
                    if self.maze.check_wall(&self.p - MAZE_SIZE) {
                        Ghost {
                            p: self.p - MAZE_SIZE,
                            d: self.d,
                            t: self.t,
                            maze: self.maze
                        }
                    } else {
                        Ghost {
                            p: self.p,
                            d: 'w',
                            t: self.t,
                            maze: self.maze
                        }
                    }
                },
                's' => {
                    if self.maze.check_wall(&self.p + MAZE_SIZE) {
                        Ghost {
                            p: self.p + MAZE_SIZE,
                            d: self.d,
                            t: self.t,
                            maze: self.maze
                        }
                    } else {
                        Ghost {
                            p: self.p,
                            d: 'e',
                            t: self.t,
                            maze: self.maze
                        }
                    }
                },
                'e' => {
                    if self.maze.check_wall(&self.p + 1) {
                        Ghost {
                            p: self.p + 1,
                            d: self.d,
                            t: self.t,
                            maze: self.maze
                        }
                    } else {
                        Ghost {
                            p: self.p,
                            d: 'n',
                            t: self.t,
                            maze: self.maze
                        }
                    }
                },
                'w' => {
                    if self.maze.check_wall(&self.p - 1) {
                        Ghost {
                            p: self.p - 1,
                            d: self.d,
                            t: self.t,
                            maze: self.maze
                        }
                    } else {
                        Ghost {
                            p: self.p,
                            d: 's',
                            t: self.t,
                            maze: self.maze
                        }
                    }
                },
                _ => {
                    self
                }
            }
        }
    }
}