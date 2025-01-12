pub mod message {
    use crate::game::{Point, Renderer, HtmlImageElement};

    pub struct Message {
    }
    impl Message {
        pub fn new() -> Self{
            Message{}
        }
        pub fn draw(&self, renderer: &Renderer, _m: &char, image:HtmlImageElement ) {
            match &_m {
                'a' => {
                    let _ = renderer.clear();
                    let _ = renderer.polygon(
                        &Point{x: 400.0, y:275.0},
                        &Point{x:525.0, y:150.0},
                        &Point{x: 525.0, y: 750.0},
                        &Point{x: 400.0, y: 625.0},
                        &'l'
                    );
                    renderer.polygon(
                        &Point{x: -75.0, y:150.0},
                        &Point{x:50.0, y:275.0},
                        &Point{x: 50.0, y: 625.0},
                        &Point{x: -75.0, y: 750.0},
                        &'l'
                    );
                    renderer.polygon(
                        &Point{x: 325.0, y:350.0},
                        &Point{x:400.0, y:275.0},
                        &Point{x: 400.00, y: 625.0},
                        &Point{x: 325.0, y: 550.0},
                        &'m'
                    );
                    renderer.polygon(
                        &Point{x: 50.0, y:275.0},
                        &Point{x:125.0, y:350.0},
                        &Point{x: 125.0, y: 550.0},
                        &Point{x: 50.0, y: 625.0},
                        &'m'
                    );
                    // ivy
                    let _ = renderer.draw_image(
                        &image, 0.0, 500.0, 100.0, 100.0, 120.0, 10.0, 200.0, 200.0,
                    );
                    // text box
                    let _ = renderer.draw_image(
                        &image, 0.0, 400.0, 100.0, 100.0, 5.0, 200.0, 430.0, 340.0,
                    );
                    let _ = renderer.text(&Point{x:50.0, y:360.0}, "I think I can escape from there.", 'l', 18, 'l');
                    let _ = renderer.text(&Point{x:50.0, y:395.0}, "If I can reach the ivy.", 'l', 18, 'l');
                },
                'b' => {
                    let _ = renderer.clear();
                    let _ = renderer.polygon(
                        &Point{x: 400.0, y: -25.0},
                        &Point{x:525.0, y: -100.0},
                        &Point{x: 525.0, y: 450.0},
                        &Point{x: 400.0, y: 325.0},
                        &'l'
                    );
                    renderer.polygon(
                        &Point{x: -75.0, y: -150.0},
                        &Point{x:50.0, y: -25.0},
                        &Point{x: 50.0, y: 325.0},
                        &Point{x: -75.0, y: 450.0},
                        &'l'
                    );
                    renderer.polygon(
                        &Point{x: 325.0, y: 50.0},
                        &Point{x:400.0, y: -25.0},
                        &Point{x: 400.00, y: 325.0},
                        &Point{x: 325.0, y: 250.0},
                        &'m'
                    );
                    renderer.polygon(
                        &Point{x: 50.0, y: -25.0},
                        &Point{x:125.0, y: 50.0},
                        &Point{x: 125.0, y: 250.0},
                        &Point{x: 50.0, y: 325.0},
                        &'m'
                    );
                    // box
                    let _ = renderer.draw_image(
                        &image, 0.0, 600.0, 100.0, 100.0, 90.0, 320.0, 300.0, 300.0,
                    );
                    // text box
                    let _ = renderer.draw_image(
                        &image, 0.0, 400.0, 100.0, 100.0, 5.0, 50.0, 430.0, 340.0,
                    );
                    let _ = renderer.text(&Point{x:50.0, y:210.0}, "This box is pretty sturdy. ", 'l', 18, 'l');
                    let _ = renderer.text(&Point{x:50.0, y:245.0}, "I might be able to reach high...", 'l', 18, 'l');
                }
                _=> {}
            }
        }
    }
}