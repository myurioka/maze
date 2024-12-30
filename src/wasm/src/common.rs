/* <-- CONSTANT VALUE */
pub const MAZE_SIZE: usize = 21;
pub const FRAME_TERM:i32 = 10; //ACTION TERM
pub const WIDTH: usize = 450;  //HTML CANVAS WIDTH
pub const HEIGHT: usize = 600; //HTML CANVASA HEIGHT
pub const OPENING_TITLE_X: f32 = 225.0;
pub const OPENING_TITLE_Y: f32 = 320.0;
pub const OPENING_TITLE: &str = "M A Z E";
pub const OPENING_MESSAGE_X: f32 = 225.0;
pub const OPENING_MESSAGE_Y: f32 = 120.0;
pub const OPENING_MESSAGE: &str = "HIT SPACE KEY!";
pub const GAMEOVER_MESSAGE_X: f32 = 225.0;
pub const GAMEOVER_MESSAGE_Y: f32 = 220.0;
pub const GAMEOVER_MESSAGE: &str = "GAME OVER!";
pub const GAMECLEAR_MESSAGE: &str = "GAME CLEAR!!";
pub const MAP_X: usize = 50;
pub const MAP_Y: usize = 30;
pub const MAP_WIDTH: usize = 8;
pub const MAZE_MAP: [usize; MAZE_SIZE * MAZE_SIZE] = [
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,0,0,0,0,0,0,1,1,0,0,0,0,0,0,0,1,1,1,
                1,1,1,0,3,0,0,0,0,1,1,0,0,0,0,0,1,0,1,1,1,
                1,1,1,0,0,1,1,1,0,1,1,0,0,3,0,0,1,0,1,1,1,
                1,1,1,0,0,1,1,1,0,0,0,0,1,1,0,0,1,0,1,1,1,
                1,1,1,0,0,1,1,1,1,1,1,0,1,1,1,1,1,0,1,1,1,
                1,1,1,0,0,1,1,1,1,1,1,0,0,0,0,0,0,0,1,1,1,
                1,1,1,0,0,1,0,0,0,0,0,0,1,1,1,0,0,0,1,1,1,
                1,1,1,1,0,0,0,1,0,1,1,0,1,1,1,1,1,0,1,1,1,
                1,1,1,1,1,0,1,1,0,1,1,0,0,0,0,0,1,0,1,1,1,
                1,1,1,1,1,0,1,1,2,1,1,0,1,1,1,0,1,0,1,1,1,
                1,1,1,1,1,0,1,1,0,1,1,0,1,1,1,0,1,0,1,1,1,
                1,1,1,1,1,0,1,1,0,1,1,0,1,1,1,0,1,0,1,1,1,
                1,1,1,0,0,3,0,0,0,0,0,0,0,0,0,3,0,0,1,1,1,
                1,1,1,0,0,0,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,8,0,0,1,1,1,0,0,0,0,0,0,0,0,0,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
            ];
/*  0: none
    1: wall
    2: start/goal
    3: ghost start position
    8: item(box)
*/
pub const GROUND_MAP: [usize; MAZE_SIZE * MAZE_SIZE] = [
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,0,0,1,1,1,0,0,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,0,0,1,9,1,0,0,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,0,0,1,0,1,0,0,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,0,0,1,0,1,0,0,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,0,0,1,0,1,0,0,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,0,0,1,2,1,0,0,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,0,0,1,8,1,0,0,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,0,0,1,3,1,0,0,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,0,0,1,1,1,0,0,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,0,0,0,0,0,0,0,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
            ];
/* CONSTANT VALUE --> */
