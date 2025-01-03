pub mod map {
    use crate::game::{Point, Renderer, MAP_WIDTH, MAZE_SIZE, MAP_X, MAP_Y};

    pub struct Map {
        pub map: [usize; MAZE_SIZE * MAZE_SIZE],
    }
    impl Map {
        pub fn mapping(&mut self, _p:usize) {
            self.map[_p] = 9;
        }

        pub fn draw(&self, renderer: &Renderer, p: &usize ) {
            let _width: usize = MAP_WIDTH;
            let _p: usize = *p;

            for i in 0..self.map.len() {
                match self.map[i] { 
                    9 => {
                        renderer.polygon(
                            &Point{x: (MAP_X + i % MAZE_SIZE * _width) as f32, y: (MAP_Y + i / MAZE_SIZE * _width) as f32},
                            &Point{x: (MAP_X + (i % MAZE_SIZE + 1 ) * _width) as f32, y: (MAP_Y + i / MAZE_SIZE * _width) as f32},
                            &Point{x: (MAP_X + (i % MAZE_SIZE + 1 ) * _width) as f32, y: (MAP_Y + (i / MAZE_SIZE + 1) * _width) as f32},
                            &Point{x: (MAP_X + i % MAZE_SIZE * _width) as f32, y: (MAP_Y + (i / MAZE_SIZE + 1) * _width) as f32},
                            &'d'
                        );
                    },
                    _ => {}
                }
            }
            // current location
            renderer.polygon(
                &Point{x: (MAP_X + *p % MAZE_SIZE * _width) as f32, y: (MAP_Y + *p / MAZE_SIZE * _width) as f32},
                &Point{x: (MAP_X + (*p % MAZE_SIZE + 1 ) * _width) as f32, y: (MAP_Y + *p / MAZE_SIZE * _width) as f32},
                &Point{x: (MAP_X + (*p % MAZE_SIZE + 1 ) * _width) as f32, y: (MAP_Y + (*p / MAZE_SIZE + 1) * _width) as f32},
                &Point{x: (MAP_X + *p % MAZE_SIZE * _width) as f32, y: (MAP_Y + (*p / MAZE_SIZE +1)* _width) as f32},
                &'d'
            );
        }
    }
}