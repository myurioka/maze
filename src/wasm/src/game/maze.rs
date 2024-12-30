mod wall;
use wall::wall::*;

pub mod maze {
    use crate::game::maze::Wall;
    use crate::game::Ghost;
    use crate::game::{Renderer, Point, MAZE_SIZE, GREEN_DARK_MIDDLE, HtmlImageElement};

    #[derive(Clone, Copy)]
    pub struct Maze {
        pub maze: [usize; MAZE_SIZE * MAZE_SIZE],
    }
    impl Maze {
        pub fn new(_maze:[usize; MAZE_SIZE * MAZE_SIZE]) -> Maze {
            Maze {
                maze: _maze,
            }
        }
        pub fn get_position_value(&self, _p:usize) -> usize{
            self.maze[_p]
        }
        pub fn get_start_position(&self) -> usize {
            self.maze.iter().position(|&x| x == 2).unwrap()
        }
        pub fn get_box_position(&self) -> usize {
            for i in 0..self.maze.len() {
                if self.maze[i] == 8 {
                    return i;
                }
            }
            return 0;
        }
        pub fn get_goal_position(&self) -> usize {
            for i in 0..self.maze.len() {
                if self.maze[i] == 9 {
                    return i;
                }
            }
            return 1;
        }
        pub fn get_ghosts_position(&self) -> Vec<usize> {
            let mut _index = Vec::new();
            for i in 0..self.maze.len() {
                if self.maze[i] == 3 {
                    _index.push(i);
                } 
            }
            return _index
        }
        pub fn get_maze(&self) -> [usize; MAZE_SIZE * MAZE_SIZE] {
            let _maze = self.maze.clone();
            _maze
        }
        pub fn set_maze(&mut self, _maze: [usize; MAZE_SIZE * MAZE_SIZE]){
            self.maze = _maze
        }
        pub fn check_wall(&self, _p:usize) -> bool {
            if self.maze[_p] == 1 { return false; }
            return true;
        }
        pub fn check_direction_for_ghost(&self, _p:usize) -> bool {
            if self.maze[_p] != 0 { return false; }
            return true;
        }
        pub fn check_goal(&self, _p:usize) -> bool {
            if self.maze[_p] == 8 { return true; }
            return false;
        }
        pub fn draw(&self, renderer: &Renderer, _p: &usize, _d: char, _out_box: &bool, image:HtmlImageElement, ghosts: &Vec<Ghost> ) {
            let mut _pt: usize = 0;
            match _d {
                'n' => {
                    for i in [0,2,1] {
                        _pt = _p - 3 * MAZE_SIZE - 1 + i;
                        match self.maze[_pt] {
                            1 => { let _= &Wall::draw(renderer,i as i16); }
                            2 => { Self::draw_start_1(renderer,&i,&image);}
                            8 => { Self::draw_box_1(renderer,&i,&image);}
                            _ => {}
                        }
                        ghosts.iter().for_each(|_g| {
                            if _g.get_position() == _pt {
                                Self::draw_image_1(
                                    renderer, &i, 
                                    _g.get_type(),
                                    &image
                                )
                            }
                        });
                    }
                    for i in [0,2,1] {
                        _pt = _p - 2 * MAZE_SIZE - 1 + i;
                        match self.maze[_pt] {
                            1 => { let _= &Wall::draw(renderer, (i + 3) as i16); }
                            2 => { Self::draw_start_2(renderer, &i, &image);}
                            8 => { Self::draw_box_2(renderer,&i,&image);}
                            _ => {}
                        }
                        ghosts.iter().for_each(|_g| {
                            if _g.get_position() == _pt {
                                Self::draw_image_2(
                                    renderer, &i, 
                                    _g.get_type(),
                                    &image
                                )
                            }
                        });
                    }
                    for i in [0,2,1] {
                        _pt = _p - 1 * MAZE_SIZE - 1 + i;
                        match self.maze[_pt] {
                            1 => { let _= &Wall::draw(renderer, (i + 6) as i16); }
                            2 => { Self::draw_start_3(renderer, &i, &image);}
                            8 => { Self::draw_box_3(renderer,&i,&image);},
                            _ => {}
                        }
                        ghosts.iter().for_each(|_g| {
                            if _g.get_position() == _pt {
                                Self::draw_image_3(
                                    renderer, &i, 
                                    _g.get_type(),
                                    &image
                                )
                            }
                        });
                    }
                    for i in [0,2,1] {
                        match self.maze[ _p - 1 + i] {
                            1 => { let _= &Wall::draw(renderer, (i + 9) as i16);}
                            2 => { Self::draw_start_4(renderer, &i, &image);}
                            _ => {}
                        }
                    }
                }
                's' => {
                    for i in [0,2,1] {
                        _pt = _p + 3 * MAZE_SIZE + 1 - i;
                        match self.maze[_pt] {
                            1 => { let _= &Wall::draw(renderer, i as i16); }
                            2 => { Self::draw_start_1(renderer, &i, &image);}
                            8 => { Self::draw_box_1(renderer,&i,&image);}
                            _ => {}
                        }
                        ghosts.iter().for_each(|_g| {
                            if _g.get_position() == _pt {
                                Self::draw_image_1(
                                    renderer, &i, 
                                    _g.get_type(),
                                    &image
                                )
                            }
                        });
                    }
                    for i in [0,2,1] {
                        _pt = _p + 2 * MAZE_SIZE + 1 - i;
                        match self.maze[_pt] {
                            1 => { let _= &Wall::draw(renderer, (i + 3) as i16); }
                            2 => { Self::draw_start_2(renderer, &i, &image);}
                            8 => { Self::draw_box_2(renderer,&i,&image);},
                            _ => {}
                        }
                        ghosts.iter().for_each(|_g| {
                            if _g.get_position() == _pt {
                                Self::draw_image_2(
                                    renderer, &i, 
                                    _g.get_type(),
                                    &image
                                )
                            }
                        });
                    }
                    for i in [0,2,1] {
                        _pt = _p + 1 * MAZE_SIZE + 1 - i;
                        match self.maze[_pt] {
                            1 => { let _= &Wall::draw(renderer, (i + 6) as i16); }
                            2 => { Self::draw_start_3(renderer, &i, &image);}
                            8 => { Self::draw_box_3(renderer,&i,&image);}
                            _ => {}
                        }
                        ghosts.iter().for_each(|_g| {
                            if _g.get_position() == _pt {
                                Self::draw_image_3(
                                    renderer, &i, 
                                    _g.get_type(),
                                    &image
                                )
                            }
                        });
                    }
                    for i in [0,2,1] {
                        match self.maze[_p + 1 - i] {
                            1 => { let _= &Wall::draw(renderer, (i + 9) as i16);}
                            2 => { Self::draw_start_4(renderer, &i, &image);}
                            _ => {}
                        }
                    }
                }
                'w' => {
                    for i in [0,2,1] {
                        _pt = _p - 3 - MAZE_SIZE * (i - 1);
                        match self.maze[_pt] {
                            1 => { let _= &Wall::draw(renderer, i as i16); },
                            2 => { Self::draw_start_1(renderer, &i, &image);}
                            8 => { Self::draw_box_1(renderer,&i,&image);}
                            _ => {}
                        }
                        ghosts.iter().for_each(|_g| {
                            if _g.get_position() == _pt {
                                Self::draw_image_1(
                                    renderer, &i, 
                                    _g.get_type(),
                                    &image
                                )
                            }
                        });
                    }
                    for i in [0,2,1] {
                        _pt = _p - 2 - MAZE_SIZE * (i - 1);
                        match self.maze[ _p - 2 - MAZE_SIZE * (i - 1) ] {
                            1 => { let _= &Wall::draw(renderer, (i + 3) as i16); },
                            2 => { Self::draw_start_2(renderer, &i, &image);}
                            8 => { Self::draw_box_2(renderer,&i,&image);}
                            _ => {}
                        }
                        ghosts.iter().for_each(|_g| {
                            if _g.get_position() == _pt {
                                Self::draw_image_2(
                                    renderer, &i, 
                                    _g.get_type(),
                                    &image
                                )
                            }
                        });
                    }
                    for i in [0,2,1] {
                        _pt = _p - 1 - MAZE_SIZE * (i - 1);
                        match self.maze[ _p - 1 - MAZE_SIZE * (i - 1) ] {
                            1 => { let _= &Wall::draw(renderer, (i + 6) as i16); },
                            2 => { Self::draw_start_3(renderer, &i, &image);}
                            8 => { Self::draw_box_3(renderer,&i,&image);}
                            _ => {}
                        }
                        ghosts.iter().for_each(|_g| {
                            if _g.get_position() == _pt {
                                Self::draw_image_3(
                                    renderer, &i, 
                                    _g.get_type(),
                                    &image
                                )
                            }
                        });
                    }
                    for i in [0,2,1] {
                        _pt = _p - MAZE_SIZE * (i - 1);
                        match self.maze[_pt] {
                            1 => { let _= &Wall::draw(renderer, (i + 9) as i16); },
                            2 => { Self::draw_start_4(renderer, &i, &image);}
                            _ => {}
                        }
                    }
                }
                'e' => {
                    for i in [0,2,1] {
                        _pt = _p + 3 + MAZE_SIZE * (i - 1);
                        match self.maze[_pt] {
                            1 => { let _= &Wall::draw(renderer, i as i16); },
                            2 => { Self::draw_start_1(renderer, &i, &image);}
                            8 => { Self::draw_box_1(renderer,&i,&image);}
                            9 => {},
                            _ => {}
                        }
                        ghosts.iter().for_each(|_g| {
                            if _g.get_position() == _pt {
                                Self::draw_image_1(
                                    renderer, &i, 
                                    _g.get_type(),
                                    &image
                                )
                            }
                        });
                    }
                    for i in [0,2,1] {
                        _pt = _p + 2 + MAZE_SIZE * (i - 1);
                        match self.maze[ _p + 2 + MAZE_SIZE * (i - 1) ] {
                            1 => { let _= &Wall::draw(renderer, (i + 3) as i16); },
                            2 => { Self::draw_start_2(renderer, &i, &image);}
                            8 => { Self::draw_box_2(renderer,&i,&image);}
                            _ => {}
                        }
                        ghosts.iter().for_each(|_g| {
                            if _g.get_position() == _pt {
                                Self::draw_image_2(
                                    renderer, &i, 
                                    _g.get_type(),
                                    &image
                                )
                            }
                        });
                    }
                    for i in [0,2,1] {
                        _pt = _p + 1 + MAZE_SIZE * (i - 1);
                        match self.maze[_pt] {
                            1 => { let _= &Wall::draw(renderer, (i + 6) as i16); },
                            2 => { Self::draw_start_3(renderer, &i, &image);}
                            8 => { Self::draw_box_3(renderer,&i,&image);}
                            _ => {}
                        }
                        ghosts.iter().for_each(|_g| {
                            if _g.get_position() == _pt {
                                Self::draw_image_3(
                                    renderer, &i, 
                                    _g.get_type(),
                                    &image
                                )
                            }
                        });
                    }
                    for i in [0,2,1] {
                        match self.maze[_p + MAZE_SIZE * (i - 1)] {
                            1 => { let _= &Wall::draw(renderer, (i + 9) as i16);},
                            2 => { Self::draw_start_4(renderer, &i, &image);}
                            _ => {}
                        }
                    }
                },
                _ => {},
            }
            if !*_out_box {
                Self::draw_in_box(renderer);
            }
        }
        fn draw_start_1(renderer: &Renderer, i: &usize, image: &HtmlImageElement) {
            let mut _p = 300.0;
            let _= renderer.draw_image(
                &image, 0.0, _p, 100.0,100.0,(40 + 150 * i) as f32, 20.0, 60.0, 60.0
            );
        }
        fn draw_start_2(renderer: &Renderer, i: &usize, image: &HtmlImageElement) {
            let mut _p = 300.0;
            let _= renderer.draw_image(
                &image, 0.0, _p, 100.0,100.0,(50 + 120 * i) as f32, 10.0, 100.0, 100.0
            );
        }
        fn draw_start_3(renderer: &Renderer, i: &usize, image: &HtmlImageElement) {
            let mut _p = 300.0;
            let _= renderer.draw_image(
                &image, 0.0, _p, 100.0,100.0,(20 + 110 * i) as f32, 0.0, 150.0, 150.0
            );
        }
        fn draw_start_4(renderer: &Renderer, i: &usize, image: &HtmlImageElement) {
            let mut _p = 300.0;
            let _= renderer.draw_image(
                &image, 0.0, _p, 100.0,100.0,(40 + 100 * i) as f32, -100.0, 200.0, 200.0
            );
        }
        fn draw_image_1(renderer: &Renderer, i: &usize, t: char, image: &HtmlImageElement) {
            let mut _p = 100.0;
            if t == 'b' { _p = 200.0;} 
            let _= renderer.draw_image(
                &image, -1.0, _p, 100.0,100.0,(100 + 100 * i) as f32, 250.0, 50.0, 50.0
            );
        }
        fn draw_image_2(renderer: &Renderer, i: &usize, t: char, image: &HtmlImageElement) {
            let mut _p = 100.0;
            if t == 'b' { _p = 200.0;} 
            let _= renderer.draw_image(
                &image, 0.0, _p, 100.0, 100.0, (90 + 100 * i) as f32, 230.0, 60.0, 60.0,
            );
        }
        fn draw_image_3(renderer: &Renderer, i: &usize, t: char, image: &HtmlImageElement) {
            let mut _p = 100.0;
            if t == 'b' { _p = 200.0;} 
            let _= renderer.draw_image(
                &image, 0.0, _p, 100.0, 100.0, (0 + 150 * i) as f32, 150.0, 180.0, 180.0,
            );
        }
        fn draw_box_1(renderer: &Renderer, i: &usize, image: &HtmlImageElement) {
            let _= renderer.draw_image(&image,0.0,0.0,100.0,100.0,(40 + 160 * i) as f32, 320.0, 50.0, 50.0,);
        }
        fn draw_box_2(renderer: &Renderer, i: &usize, image: &HtmlImageElement) {
            let _= renderer.draw_image(&image, 0.0,0.0,100.0,100.0,(60 + 120 * i) as f32, 340.0, 80.0, 80.0,);
        }
        fn draw_box_3(renderer: &Renderer, i: &usize, image: &HtmlImageElement) {
            let _= renderer.draw_image(&image, 0.0,0.0,100.0,100.0,(180 * i - 10) as f32, 340.0, 130.0, 130.0,);
        }

        fn draw_in_box(renderer: &Renderer) {
            renderer.rect(
                &Point{x: 0.0, y:50.0},
                450.0,
                200.0,
                GREEN_DARK_MIDDLE,
                1.0
            );

            renderer.rect(
                &Point{x: 0.0, y:300.0},
                450.0,
                200.0,
                GREEN_DARK_MIDDLE,
                1.0
            );
        }

        pub fn draw_ground(&self, renderer: &Renderer, _p: &usize, _d: char, _f: &i32, image:HtmlImageElement, ghosts: &Vec<Ghost> ) {
            let mut _pt: usize = 0;
            match _d {
                'n' => {
                    if self.maze[_p - MAZE_SIZE] == 1 {
                        let _ = renderer.draw_image(
                            &image, 120.0, 0.0, 110.0, 98.0, 20.0, 0.0, 450.0, 600.0,
                        );
                    } else if self.maze[_p - 2 * MAZE_SIZE] == 1 {
                        let _ = renderer.draw_image(
                            &image, 115.0, 0.0, 115.0, 98.0, 15.0, 0.0, 450.0, 550.0,
                        );
                    } else if self.maze[_p - 3 * MAZE_SIZE] == 1 {
                        let _ = renderer.draw_image(
                            &image, 110.0, 0.0, 120.0, 98.0, 10.0, 0.0, 450.0, 500.0,
                        );
                    } else if self.maze[_p - 4 * MAZE_SIZE] == 1 {
                        let _ = renderer.draw_image(
                            &image, 105.0, 0.0, 125.0, 98.0, 5.0, 0.0, 450.0, 450.0,
                        );
                    } else if self.maze[_p - 5 * MAZE_SIZE] == 1 {
                        let _ = renderer.draw_image(
                            &image, 100.0, 0.0, 130.0, 98.0, 0.0, 0.0, 450.0, 400.0,
                        );
                    }
                }
                's' => {
                    //if self.maze[_p + 3 * MAZE_SIZE] == 1 {
                        let _ = renderer.draw_image(
                            &image, 100.0, 100.0, 130.0, 100.0, 0.0, 0.0, 450.0, 400.0,
                        );
                    //}
                    //if self.maze[_p + 2 * MAZE_SIZE] == 1 {
                    //    let _ = renderer.draw_image(
                    //        &image, 100.0, 100.0, 130.0, 100.0, 0.0, 0.0, 450.0, 400.0,);
                    //}
                    if self.maze[_p + 2 * MAZE_SIZE] == 8 {
                        if *_f > 5 {
                            let _ = renderer.draw_image(
                                &image, 100.0, 600.0, 100.0, 200.0, 30.0, 0.0, 350.0, 650.0,
                            );
                        } else {
                            let _ = renderer.draw_image(
                                &image, 100.0, 400.0, 100.0, 200.0, 30.0, 0.0, 350.0, 650.0,
                            );
                        }
                    }
                    if self.maze[_p + MAZE_SIZE] == 8 {
                        if *_f > 5 {
                            let _ = renderer.draw_image(
                                &image, 100.0, 600.0, 100.0, 200.0, 30.0, -10.0, 350.0, 650.0,
                            );
                        } else {
                            let _ = renderer.draw_image(
                                &image, 100.0, 400.0, 100.0, 200.0, 30.0, -10.0, 350.0, 650.0,
                            );
                        }
                    }
                    /*
                    _pt = _p + 3 * MAZE_SIZE;
                    ghosts.iter().for_each(|_g| {
                        if _g.get_position() == _pt {
                            Self::draw_image_1(
                                renderer,
                                &1, 
                                _g.get_type(),
                                &image
                            )
                        }
                    });
                    */
                    /*
                    _pt = _p + 2 * MAZE_SIZE;
                    ghosts.iter().for_each(|_g| {
                        if _g.get_position() == _pt {
                            Self::draw_image_2(
                                renderer,
                                &1, 
                                _g.get_type(),
                                &image
                            )
                        }
                    });
                    */
                    _pt = _p + 1 * MAZE_SIZE;
                    ghosts.iter().for_each(|_g| {
                        if _g.get_position() == _pt {
                            Self::draw_image_3(
                                renderer,
                                &1, 
                                _g.get_type(),
                                &image
                            )
                        }
                    });
                }
                'w' => {
                    let _ = renderer.draw_image(
                        &image, 100.0, 200.0, 130.0, 100.0, 0.0, 0.0, 450.0, 600.0,
                    );
                }
                'e' => {
                    let _ = renderer.draw_image(
                        &image, 100.0, 300.0, 130.0, 100.0, 0.0, 0.0, 450.0, 600.0,
                    );
                }
                _ => {},
            }
        }
    }
}