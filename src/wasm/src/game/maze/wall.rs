pub mod wall {
    use crate::game::{Point, Renderer};

    pub struct Wall{}
    impl Wall {
        pub fn draw(renderer: &Renderer, num:i16) {
            match num {
                11 => {
                    renderer.polygon(
                        &Point{x: 400.0, y:125.0},
                        &Point{x:525.0, y:0.0},
                        &Point{x: 525.0, y: 600.0},
                        &Point{x: 400.0, y: 475.0},
                        &'l'
                    );
                }
                9 => {
                    renderer.polygon(
                        &Point{x: -75.0, y:0.0},
                        &Point{x:50.0, y:125.0},
                        &Point{x: 50.0, y: 475.0},
                        &Point{x: -75.0, y: 600.0},
                        &'l'
                    );
                }
                8 => {
                    renderer.polygon(
                        &Point{x: 325.0, y:200.0},
                        &Point{x:400.0, y:125.0},
                        &Point{x: 400.00, y: 475.0},
                        &Point{x: 325.0, y: 400.0},
                        &'m'
                    );
                    renderer.polygon(
                        &Point{x: 400.0, y:125.0},
                        &Point{x:575.0, y:125.0},
                        &Point{x: 575.00, y: 475.0},
                        &Point{x: 400.0, y: 475.0},
                        &'m'
                    );
                }
                7 => {
                    renderer.polygon(
                        &Point{x: 50.0, y:125.0},
                        &Point{x:400.0, y:125.0},
                        &Point{x: 400.0, y: 475.0},
                        &Point{x: 50.0, y: 475.0},
                        &'m'
                    );
                }
                6 => {
                    renderer.polygon(
                        &Point{x: -75.0, y:125.0},
                        &Point{x:50.0, y:125.0},
                        &Point{x: 50.00, y: 475.0},
                        &Point{x: -75.0, y: 475.0},
                        &'m'
                    );
                    renderer.polygon(
                        &Point{x: 50.0, y:125.0},
                        &Point{x:125.0, y:200.0},
                        &Point{x: 125.0, y: 400.0},
                        &Point{x: 50.0, y: 475.0},
                        &'m'
                    );
                }
                5 => {
                    renderer.polygon(
                        &Point{x: 275.0, y:250.0},
                        &Point{x:325.0, y:200.0},
                        &Point{x: 325.00, y: 400.0},
                        &Point{x: 275.0, y: 350.0},
                        &'h'
                    );
                    renderer.polygon(
                        &Point{x: 325.0, y:200.0},
                        &Point{x:475.0, y:200.0},
                        &Point{x: 475.00, y: 400.0},
                        &Point{x: 325.0, y: 400.0},
                        &'h'
                    );
                }
                4 => {
                    renderer.polygon(
                        &Point{x: 125.0, y:200.0},
                        &Point{x:325.0, y:200.0},
                        &Point{x: 325.00, y: 400.0},
                        &Point{x: 125.0, y: 400.0},
                        &'h'
                    );
                }
                3 => {
                    renderer.polygon(
                        &Point{x: -75.0, y:200.0},
                        &Point{x:125.0, y:200.0},
                        &Point{x: 125.00, y: 400.0},
                        &Point{x: -75.0, y: 400.0},
                        &'h'
                    );
                    renderer.polygon(
                        &Point{x: 125.0, y:200.0},
                        &Point{x:175.0, y:250.0},
                        &Point{x: 175.00, y: 350.0},
                        &Point{x: 125.0, y: 400.0},
                        &'h'
                    );
                }
                2 => {
                    renderer.polygon(
                        &Point{x: 275.0, y:250.0},
                        &Point{x:375.0, y:250.0},
                        &Point{x: 375.00, y: 350.0},
                        &Point{x: 275.0, y: 350.0},
                        &'b'
                    );
                }
                1 => {
                    renderer.polygon(
                        &Point{x: 175.0, y:250.0},
                        &Point{x:275.0, y:250.0},
                        &Point{x: 275.00, y: 350.0},
                        &Point{x: 175.0, y: 350.0},
                        &'b'
                    );
                }
                0 => {
                    renderer.polygon(
                        &Point{x: 75.0, y:250.0},
                        &Point{x:175.0, y:250.0},
                        &Point{x: 175.00, y: 350.0},
                        &Point{x: 75.0, y: 350.0},
                        &'b'
                    );
                }
                _ => {
                }
            }
        }
    }
}