/* <-- CONSTANT VALUE */
pub const MAZE_SIZE: usize = 21;
pub const FRAME_TERM:i32 = 10; //ACTION TERM
pub const OPENING_TITLE_X: f32 = 225.0;
pub const OPENING_TITLE_Y: f32 = 320.0;
pub const OPENING_TITLE: &str = "M A Z E";
pub const OPENING_MESSAGE_X: f32 = 225.0;
pub const OPENING_MESSAGE_Y: f32 = 120.0;
pub const OPENING_MESSAGE: &str = "HIT SPACE KEY!";
pub const HOWTO_MESSAGE_T_X: f32 = 225.0;
pub const HOWTO_MESSAGE_T_Y: f32 = 40.0;
pub const HOWTO_MESSAGE_T: &str = "Touch Area for Action";
pub const HOWTO_MESSAGE_F_X: f32 = 225.0;
pub const HOWTO_MESSAGE_F_Y: f32 = 120.0;
pub const HOWTO_MESSAGE_F: &str = "Forward";
pub const HOWTO_MESSAGE_L_X: f32 = 50.0;
pub const HOWTO_MESSAGE_L_Y: f32 = 300.0;
pub const HOWTO_MESSAGE_L: &str = "Left 🢀";
pub const HOWTO_MESSAGE_R_X: f32 = 400.0;
pub const HOWTO_MESSAGE_R_Y: f32 = 300.0;
pub const HOWTO_MESSAGE_R: &str = "🢂 Right";
pub const HOWTO_MESSAGE_B_X: f32 = 225.0;
pub const HOWTO_MESSAGE_B_Y: f32 = 500.0;
pub const HOWTO_MESSAGE_B: &str = "Back";
pub const HOWTO_MESSAGE_S_X: f32 = 225.0;
pub const HOWTO_MESSAGE_S_Y: f32 = 300.0;
pub const HOWTO_MESSAGE_S: &str = "Action";

pub const GAMEOVER_MESSAGE_X: f32 = 225.0;
pub const GAMEOVER_MESSAGE_Y: f32 = 220.0;
pub const GAMEOVER_MESSAGE: &str = "GAME OVER!";
pub const MAP_X: usize = 50;
pub const MAP_Y: usize = 30;
pub const MAP_WIDTH: usize = 8;
pub const GROUND_START_FRAME: i32 = 30;
pub const MAZE_MAP: [usize; MAZE_SIZE * MAZE_SIZE] = [
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,0,0,0,0,0,0,1,1,0,0,0,8,0,0,0,1,1,1,
                1,1,1,0,3,0,0,0,0,1,1,0,0,0,0,0,1,0,1,1,1,
                1,1,1,0,0,1,1,1,0,1,1,0,0,3,0,0,1,0,1,1,1,
                1,1,1,0,0,1,1,1,0,0,0,0,1,1,0,0,1,0,1,1,1,
                1,1,1,0,0,1,1,1,1,1,1,0,1,1,1,1,1,0,1,1,1,
                1,1,1,3,0,1,1,1,1,1,1,0,0,0,0,0,0,0,1,1,1,
                1,1,1,0,0,1,0,0,0,0,0,0,1,1,1,0,0,0,1,1,1,
                1,1,1,1,0,0,0,1,0,1,1,0,1,1,1,1,1,0,1,1,1,
                1,1,1,1,1,0,1,1,0,1,1,0,0,0,0,0,1,0,1,1,1,
                1,1,1,1,1,0,1,1,2,1,1,3,1,1,1,0,1,0,1,1,1,
                1,1,1,1,1,0,1,1,0,1,1,0,1,1,1,0,1,0,1,1,1,
                1,1,1,1,1,0,1,1,0,1,1,0,1,1,1,0,1,0,1,1,1,
                1,1,1,0,0,3,0,0,0,0,0,0,0,0,0,3,0,0,1,1,1,
                1,1,1,0,0,0,1,1,1,0,1,1,1,1,1,1,1,1,1,1,1,
                1,1,1,0,0,0,1,1,1,0,0,0,0,0,0,0,0,0,1,1,1,
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
                1,1,1,1,1,0,0,1,3,1,0,0,1,1,1,1,1,1,1,1,1,
                1,1,1,1,1,0,0,1,1,1,0,0,1,1,1,1,1,1,1,1,1,
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
