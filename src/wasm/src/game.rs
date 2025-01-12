mod maze;
mod map;
mod ghost;
mod message;
use anyhow::Result;
use async_trait::async_trait;
use maze::maze::*;
use map::map::*;
use message::message::*;
use ghost::ghost::*;
use crate::engine;
use crate::engine::{
    Game, KeyState, TouchState, Point, Renderer, KIMIDORI_COLOR, GREEN_DARK_MIDDLE,
};
//use crate::common;
use crate::common::{
    MAZE_SIZE,FRAME_TERM,OPENING_TITLE_X,OPENING_TITLE_Y,OPENING_TITLE,OPENING_MESSAGE_X,OPENING_MESSAGE_Y,OPENING_MESSAGE,GAMEOVER_MESSAGE_X,GAMEOVER_MESSAGE_Y,GAMEOVER_MESSAGE, MAP_X,MAP_Y,MAP_WIDTH, MAZE_MAP, GROUND_MAP, GROUND_START_FRAME,
};
use web_sys::HtmlImageElement;

pub struct GameStage {
    machine: Option<GameStageStateMachine>,
}
impl GameStage {
    pub fn new() -> Self {
        GameStage { machine: None }
    }
}
enum GameStageStateMachine {
    Ready(GameStageState<Ready>),
    Playing(GameStageState<Playing>),
    Focusing(GameStageState<Focusing>),
    GameOver(GameStageState<GameOver>),
    Escape(GameStageState<Escape>),
    GameClear(GameStageState<GameClear>),
}
impl GameStageStateMachine {
    fn new(material: Material) -> Self {
        GameStageStateMachine::Ready(GameStageState::new(material))
    }
    fn update(self, _keystate: &KeyState, _touchstate: &TouchState) -> Self {
        match self {
            GameStageStateMachine::Ready(state) => state.update(_keystate, _touchstate).into(),
            GameStageStateMachine::Playing(state) => state.update(_keystate, _touchstate).into(),
            GameStageStateMachine::Focusing(state) => state.update(_keystate, _touchstate).into(),
            GameStageStateMachine::GameOver(state) => state.update(_keystate, _touchstate).into(),
            GameStageStateMachine::Escape(state) => state.update(_keystate, _touchstate).into(),
            GameStageStateMachine::GameClear(state) => state.update(_keystate, _touchstate).into(),
        }
    }
    fn draw(&self, renderer: &Renderer) {
        match self {
            GameStageStateMachine::Ready(state) => state.material.draw(renderer),
            GameStageStateMachine::Playing(state) => state.material.draw(renderer),
            GameStageStateMachine::Focusing(state) => state.material.draw(renderer),
            GameStageStateMachine::GameOver(state) => state.material.draw(renderer),
            GameStageStateMachine::Escape(state) => state.material.draw(renderer),
            GameStageStateMachine::GameClear(state) => state.material.draw(renderer),
        };
    }
}
impl From<GameStageState<Ready>> for GameStageStateMachine {
    fn from(state: GameStageState<Ready>) -> Self {
        GameStageStateMachine::Ready(state)
    }
}
impl From<GameStageState<Playing>> for GameStageStateMachine {
    fn from(state: GameStageState<Playing>) -> Self {
        GameStageStateMachine::Playing(state)
    }
}
impl From<GameStageState<Focusing>> for GameStageStateMachine {
    fn from(state: GameStageState<Focusing>) -> Self {
        GameStageStateMachine::Focusing(state)
    }
}
impl From<GameStageState<GameOver>> for GameStageStateMachine {
    fn from(state: GameStageState<GameOver>) -> Self {
        GameStageStateMachine::GameOver(state)
    }
}
impl From<GameStageState<Escape>> for GameStageStateMachine {
    fn from(state: GameStageState<Escape>) -> Self {
        GameStageStateMachine::Escape(state)
    }
}
impl From<GameStageState<GameClear>> for GameStageStateMachine {
    fn from(state: GameStageState<GameClear>) -> Self {
        GameStageStateMachine::GameClear(state)
    }
}

struct GameStageState<T> {
    _state: T,
    material: Material,
}

struct Ready;
impl GameStageState<Ready> {
    fn new(material: Material) -> GameStageState<Ready> {
        GameStageState { _state: Ready, material,}
    }
    fn start_running(self) -> GameStageState<Playing> {
        GameStageState { _state: Playing, material: self.material,}
    }
    fn update(self, _keystate: &KeyState, _touchstate: &TouchState) -> ReadyEndState {
        if _keystate.is_pressed("Space") || _touchstate.touch_pressed() == "Space"{
            return ReadyEndState::Complete(self.start_running());
        }
        ReadyEndState::Continue(self)
    }
}
enum ReadyEndState {
    Complete(GameStageState<Playing>),
    Continue(GameStageState<Ready>),
}
impl From<ReadyEndState> for GameStageStateMachine {
    fn from(state: ReadyEndState) -> Self {
        match state {
            ReadyEndState::Complete(running) => running.into(),
            ReadyEndState::Continue(ready) => ready.into(),
        }
    }
}
struct Focusing;
impl GameStageState<Focusing> {
   fn start_running(mut self) -> GameStageState<Playing> {
        let _p = self.material.p;
        if self.material.maze.maze[_p] == 8 {
            self.material.maze.maze[_p] = 0;
        }
        GameStageState { _state: Playing, material: self.material,}
    }
    fn update(self, _keystate: &KeyState, _touchstate: &TouchState) -> FocusEndState {
        if _keystate.is_pressed("Space") || _touchstate.touch_pressed() == "Space"{
            return FocusEndState::Complete(self.start_running());
        }
        FocusEndState::Continue(self)
    }
}
enum FocusEndState {
    Complete(GameStageState<Playing>),
    Continue(GameStageState<Focusing>),
}
impl From<FocusEndState> for GameStageStateMachine {
    fn from(state: FocusEndState) -> Self {
        match state {
            FocusEndState::Complete(running) => running.into(),
            FocusEndState::Continue(focus) => focus.into(),
        }
    }
}
struct Playing;
impl GameStageState<Playing> {
    fn update(mut self, _keystate: &KeyState, _touchstate: &TouchState) -> RunningEndState {
        self.material.frame += 1;
        let mut _p = self.material.p;
        let mut _d = self.material.d;
        let mut _m = self.material.m;
        let mut _effect = self.material.effect;
        let mut _out_box = self.material.out_box;

        // INPUT KEY
        // camera moves forward
        if _out_box {
            if _keystate.is_pressed("ArrowUp") || _touchstate.touch_pressed() == "ArrowUp"{
                match self.material.d {
                    'n' => {
                        if self.material.maze.check_wall(_p - MAZE_SIZE ) {
                            _p -= MAZE_SIZE;
                        }
                    },
                    's' => {
                        if self.material.maze.check_wall(_p + MAZE_SIZE) {
                            _p += MAZE_SIZE;
                        }
                    },
                    'e' => {
                        if self.material.maze.check_wall(_p + 1) {
                            _p += 1;
                        }
                    },
                    'w' => {
                        if self.material.maze.check_wall(_p - 1) {
                            _p -= 1;
                        }
                    },
                    _ =>{}
                }
            }
            // camera moves back
            if _keystate.is_pressed("ArrowDown") || _touchstate.touch_pressed() == "ArrowDown"{
                match self.material.d {
                    'n' => {
                    if self.material.maze.check_wall(_p + MAZE_SIZE) {
                        _p += MAZE_SIZE
                    }
                    },
                    's' => {
                    if self.material.maze.check_wall(_p - MAZE_SIZE) {
                        _p -= MAZE_SIZE
                    }
                    },
                    'e' => {
                    if self.material.maze.check_wall(_p - 1) {
                        _p -= 1;
                    }
                    },
                    'w' => {
                    if self.material.maze.check_wall(_p + 1) {
                        _p += 1;
                    }
                    },
                    _ =>{}
                }
            }
            // camera moves left
            if _keystate.is_pressed("ArrowLeft") || _touchstate.touch_pressed() == "ArrowLeft" {
                match self.material.d {
                    'n' => { _d = 'w';},
                    'w' => { _d = 's';},
                    's' => { _d = 'e';},
                    'e' => { _d = 'n';},
                    _ => {}
                }
            }
            // camera moves right
            if _keystate.is_pressed("ArrowRight") || _touchstate.touch_pressed() == "ArrowRight"{
                match self.material.d {
                    'n' => { _d = 'e';},
                    'w' => { _d = 'n';},
                    's' => { _d = 'w';},
                    'e' => { _d = 's';},
                    _ => {}
                }
            }
        }
        // special action
        if _keystate.is_pressed("Space") || _touchstate.touch_pressed() == "Space"{
            let _start_p = self.material.maze.get_start_position();
            let _box_p = self.material.maze.get_box_position();
            if _p == _start_p {
                self.material.m = 'a';
                // GOAL!!
                if self.material.effect {
                    let _ground_maze = Maze::new(GROUND_MAP);
                    let _g_start_positions:Vec<usize> = _ground_maze.get_ghosts_position();
                    let mut _g = Vec::new();
                    for _g_start_position in _g_start_positions.iter(){
                        _g.push(Ghost::new(*_g_start_position,'a', &_ground_maze));
                    }
                    self.material.p = _ground_maze.get_start_position();
                    self.material.maze = Maze::new(GROUND_MAP);
                    self.material.ghosts = _g;
                    return RunningEndState::Escape(
                        GameStageState {
                            _state: Escape,
                            material: self.material,
                        }
                    );
                };
                return RunningEndState::Focusing(
                    GameStageState {
                        _state: Focusing,
                        material: self.material,
                    }
                );
            }
            if _p == _box_p {
                self.material.m = 'b';
                self.material.effect = true;
                return RunningEndState::Focusing(
                    GameStageState {
                        _state: Focusing,
                        material: self.material,
                    }
                );
            }
            if self.material.effect && self.material.out_box {
                self.material.out_box = false;
            } else if self.material.effect && !self.material.out_box {
                self.material.out_box = true;
            }
        }

        // RECORDING MAP
        self.material.map.mapping(_p);
        self.material.p = _p;
        self.material.d = _d;

        // Moving ghost 
        if self.material.frame > FRAME_TERM {
            for i in 0..self.material.ghosts.len() {
                self.material.ghosts[i] = self.material.ghosts[i].update();
            };
            self.material.frame -= FRAME_TERM;
        }
        // Colligion Check Ghost
        let mut _colligion: bool = false;
        self.material.ghosts.iter().for_each(|_ghost| {
            if self.material.p == _ghost.get_position() && self.material.out_box {
                _colligion = true;
            };
        });
        if _colligion {
            RunningEndState::GameOver(
                GameStageState {
                    _state: GameOver,
                    material: self.material,
                }
            )
        } else {
            RunningEndState::Continue(self)
        }
    }
}
impl From<RunningEndState> for GameStageStateMachine {
    fn from(state: RunningEndState) -> Self {
        match state {
            RunningEndState::Continue(running) => running.into(),
            RunningEndState::Focusing(focusing) => focusing.into(),
            RunningEndState::GameOver(gameover) => gameover.into(),
            RunningEndState::Escape(gameclear) => gameclear.into(),
        }
    }
}
enum RunningEndState {
    Continue(GameStageState<Playing>),
    Focusing(GameStageState<Focusing>),
    GameOver(GameStageState<GameOver>),
    Escape(GameStageState<Escape>),
}
struct GameOver;
impl GameStageState<GameOver> {
    fn update(self, _keystate: &KeyState, _touchstate: &TouchState) -> GameOverEndState {
        if _keystate.is_pressed("Space") || _touchstate.touch_pressed() == "Space" {
            GameOverEndState::Complete(self.new_game())
        } else {
            GameOverEndState::Continue(self)
        }
    }
    fn new_game(self) -> GameStageState<Ready> {
        GameStageState {
            _state: Ready,
            material: Material::reset(&self.material)
        }
    }
}
enum GameOverEndState {
    Continue(GameStageState<GameOver>),
    Complete(GameStageState<Ready>),
}
impl From<GameOverEndState> for GameStageStateMachine {
    fn from(state: GameOverEndState) -> Self {
        match state {
            GameOverEndState::Continue(game_over) => game_over.into(),
            GameOverEndState::Complete(ready) => ready.into(),
        }
    }
}
struct Escape;
impl GameStageState<Escape> {
    fn update(mut self, _keystate: &KeyState, _touchstate: &TouchState) -> EscapeEndState {
        self.material.frame += 1;
        let mut _p = self.material.p;
        let mut _d = self.material.d;
        let mut _m = self.material.m;
        let mut _effect = self.material.effect;
        let mut _out_box = self.material.out_box;


        if _keystate.is_pressed("ArrowUp") || _touchstate.touch_pressed() == "ArrowUp"{
            match self.material.d {
                'n' => {
                    _p -= MAZE_SIZE;
                },
                's' => {
                    if self.material.maze.check_goal(_p - MAZE_SIZE) {
                        _p += MAZE_SIZE;
                    }
                },
                _ =>{}
            }
        }
        if _keystate.is_pressed("ArrowDown") || _touchstate.touch_pressed() == "ArrowDown"{
            match self.material.d {
                's' => {
                    _p -= MAZE_SIZE;
                },
                _ =>{}
            }
        }
        if _keystate.is_pressed("ArrowLeftr") || _touchstate.touch_pressed() == "ArrowLeft"{
            match self.material.d {
                'n' => { _d = 'w';},
                'w' => { _d = 's';},
                's' => { _d = 'e';},
                'e' => { _d = 'n';},
                _ => {}
            }
        }
        // camera moves right
        if _keystate.is_pressed("ArrowRight") || _touchstate.touch_pressed() == "ArrowRight"{
            match self.material.d {
                'n' => { _d = 'e';},
                'w' => { _d = 'n';},
                's' => { _d = 'w';},
                'e' => { _d = 's';},
                _ => {}
            }
        }

        self.material.p = _p;
        self.material.d = _d;
        // GameClear
        let _g = self.material.maze.get_goal_position();
        if _p == _g {
            return EscapeEndState::GameClear(
                GameStageState {
                    _state: GameClear,
                    material: self.material,
                }
            );
        }
        // Moving ghost 
        if self.material.frame > 5 * FRAME_TERM {
            for i in 0..self.material.ghosts.len() {
                self.material.ghosts[i] = self.material.ghosts[i].update();
            };
            self.material.frame -= FRAME_TERM;
        }
        // Colligion Check Ghost
        let mut _colligion: bool = false;
        self.material.ghosts.iter().for_each(|_ghost| {
            if self.material.p == _ghost.get_position() {
                _colligion = true;
            };
        });
        if _colligion {
            self.material.stage = 'g';
            EscapeEndState::GameOver(
                GameStageState {
                    _state: GameOver,
                    material: self.material,
                }
            )
        } else {
            EscapeEndState::Continue(self)
        }
    }
}
enum EscapeEndState {
    GameClear(GameStageState<GameClear>),
    GameOver(GameStageState<GameOver>),
    Continue(GameStageState<Escape>),
}
impl From<EscapeEndState> for GameStageStateMachine {
    fn from(state: EscapeEndState) -> Self {
        match state {
            EscapeEndState::Continue(escape) => escape.into(),
            EscapeEndState::GameClear(gameclear) => gameclear.into(),
            EscapeEndState::GameOver(gameover) => gameover.into(),
        }
    }
}

struct GameClear;
impl GameStageState<GameClear> {
    fn update(self, _keystate: &KeyState, _touchstate: &TouchState) -> GameClearEndState {
        if _keystate.is_pressed("Space") || _touchstate.touch_pressed() == "Space" {
            GameClearEndState::Complete(self.new_game())
        } else {
            GameClearEndState::Continue(self)
        }
    }
    fn new_game(self) -> GameStageState<Ready> {
        GameStageState {
            _state: Ready,
            material: Material::reset(&self.material)
        }
    }
}
enum GameClearEndState {
    Continue(GameStageState<GameClear>),
    Complete(GameStageState<Ready>),
}
impl From<GameClearEndState> for GameStageStateMachine {
    fn from(state: GameClearEndState) -> Self {
        match state {
            GameClearEndState::Continue(game_clear) => game_clear.into(),
            GameClearEndState::Complete(ready) => ready.into(),
        }
    }
}

pub struct Material {
    frame: i32,         // Timer
    maze: Maze,         // maze
    p: usize,           // current camera position
    d: char,            // current camera direction
    m: char,            // messaage display flag
    stage: char,        // m: maze, g: ground
    effect: bool,       // belogings
    out_box: bool,      // hide in box
    map: Map,           // show map
    message: Message,   // message
    ghosts: Vec<Ghost>, // Ghost Character
    image: HtmlImageElement, // HTML OUTPUT IMAGE FOR CHARACTER
}
impl Material {
    fn new(_image: HtmlImageElement) -> Self {
        let _maze = Maze::new(MAZE_MAP);
        let _map = _maze.get_maze().clone();
        let _message = Message::new();
        let _start_position = _maze.get_start_position();
        let _ghosts_start_positions:Vec<usize> = _maze.get_ghosts_position();
        let mut _ghosts = Vec::new();
        let mut _cnt = 0;
        for _ghost_start_position in _ghosts_start_positions.iter() {
            if _cnt % 2 == 0 {
                _ghosts.push(Ghost::new(*_ghost_start_position, 'a', &_maze));
            } else {
                _ghosts.push(Ghost::new(*_ghost_start_position, 'b', &_maze));
            }
            _cnt += 1;
        }
        Material {
            frame: 0,
            maze: _maze,
            p: _start_position,
            d: 'n',
            m: 'a',
            stage: 'm',
            effect: false,
            out_box: true,
            map: Map{map: _map},
            message: _message,
            ghosts: _ghosts,
            image: _image,
        }
    }
    fn reset(&self) -> Material {
        Material::new(self.image.clone())
    }
    fn draw(&self, _renderer: &Renderer) {}
}

#[async_trait(?Send)]
impl Game for GameStage {
    async fn initialize(&self) -> Result<Box<dyn Game>> {
        log!("START");
        match &self.machine {
            _none => {
                let screen = engine::load_image("./static/screen.svg");
                let machine = GameStageStateMachine::new(
                    Material::new(screen.await.unwrap()),
                );
                Ok(Box::new(GameStage {
                    machine: Some(machine),
                }))
            }
        }
    }
    // Whole World UPDATE
    fn update(&mut self, _keystate: &KeyState, _touchstate: &TouchState) {
        if let Some(machine) = self.machine.take() {
            self.machine.replace(machine.update(_keystate, _touchstate));
        }
        assert!(self.machine.is_some());
    }
    fn draw(&self, renderer: &Renderer) {
        renderer.clear();
        match &self.machine {
            Some(GameStageStateMachine::Ready(_state)) => {
                renderer.polygon(
                    &Point{x: 400.0, y:125.0},
                    &Point{x:525.0, y:0.0},
                    &Point{x: 525.0, y: 600.0},
                    &Point{x: 400.0, y: 475.0},
                    &'l'
                );
                renderer.polygon(
                    &Point{x: -75.0, y:0.0},
                    &Point{x:50.0, y:125.0},
                    &Point{x: 50.0, y: 475.0},
                    &Point{x: -75.0, y: 600.0},
                    &'l'
                );
                renderer.polygon(
                    &Point{x: 325.0, y:200.0},
                    &Point{x:400.0, y:125.0},
                    &Point{x: 400.00, y: 475.0},
                    &Point{x: 325.0, y: 400.0},
                    &'m'
                );
                renderer.polygon(
                    &Point{x: 50.0, y:125.0},
                    &Point{x:125.0, y:200.0},
                    &Point{x: 125.0, y: 400.0},
                    &Point{x: 50.0, y: 475.0},
                    &'m'
                );
                renderer.text(
                    &Point {
                        x: OPENING_TITLE_X,
                        y: OPENING_TITLE_Y,
                    },
                    OPENING_TITLE,
                    'c',
                    24,
                    'd',
                );
                renderer.text(
                    &Point {
                        x: OPENING_MESSAGE_X,
                        y: OPENING_MESSAGE_Y,
                    },
                    OPENING_MESSAGE,
                    'c',
                    18,
                    'd',
                );
            }
            Some(GameStageStateMachine::Playing(_state)) => {
                let _p = _state.material.p;
                let _d = _state.material.d;
                let _ghosts = &_state.material.ghosts;
                let _image = &_state.material.image;
                let _out_box = &_state.material.out_box;
                _state.material.maze.draw(renderer, &_p, _d, &_out_box, _image.clone(), _ghosts);
                _state.material.map.draw(renderer, &_p);
                if _state.material.effect && _state.material.out_box {
                    let _= renderer.draw_image(
                        &_state.material.image, 0.0, 700.0, 100.0,100.0, 20.0, 430.0, 400.0, 400.0);
                }
            }
            Some(GameStageStateMachine::Focusing(_state)) => {
                let _m = _state.material.m;
                let _image = &_state.material.image;
                _state.material.message.draw(renderer, &_m, _image.clone()); 
            }
            Some(GameStageStateMachine::GameOver(_state)) => {
                match _state.material.stage {
                    'm' => {
                        renderer.polygon(
                            &Point{x: 400.0, y: 125.0},
                            &Point{x: 525.0, y:   0.0},
                            &Point{x: 525.0, y: 600.0},
                            &Point{x: 400.0, y: 475.0},
                            &'l'
                        );
                        renderer.polygon(
                            &Point{x: -75.0, y:0.0},
                            &Point{x:50.0, y:125.0},
                            &Point{x: 50.0, y: 475.0},
                            &Point{x: -75.0, y: 600.0},
                            &'l'
                        );
                        renderer.polygon(
                    &Point{x: 325.0, y:200.0},
                    &Point{x:400.0, y:125.0},
                    &Point{x: 400.00, y: 475.0},
                    &Point{x: 325.0, y: 400.0},
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
                    _ => {
                        let _ = renderer.draw_image(
                            &_state.material.image, 100.0, 0.0, 130.0, 98.0, 0.0, 0.0, 450.0, 600.0,
                        );
                    }
                }
                renderer.rect(
                    &Point{x: 0.0, y:0.0},
                    450.0,
                    600.0,
                    KIMIDORI_COLOR,
                    0.3
                );
                renderer.text(
                    &Point {
                        x: OPENING_TITLE_X,
                        y: OPENING_TITLE_Y,
                    },
                    OPENING_TITLE,
                    'c',
                    24,
                    'c',
                );
                renderer.text(
                    &Point {
                        x: GAMEOVER_MESSAGE_X,
                        y: GAMEOVER_MESSAGE_Y,
                    },
                    GAMEOVER_MESSAGE,
                    'c',
                    18,
                    'c',
                );
            }
            Some(GameStageStateMachine::Escape(_state)) => {
                let image = &_state.material.image;
                let _p = _state.material.p;
                let _d = _state.material.d;
                let _ghosts = &_state.material.ghosts;
                let _f = &_state.material.frame;
                _state.material.maze.draw_ground(renderer, &_p, _d, _f, image.clone(), _ghosts);
            }
            Some(GameStageStateMachine::GameClear(_state)) => {
                let _ = renderer.draw_image(
                    &_state.material.image, 100.0, 0.0, 130.0, 98.0, 0.0, 0.0, 450.0, 600.0,
                );
                // text box
                let _ = renderer.draw_image(
                    &_state.material.image, 0.0, 400.0, 100.0, 100.0, 5.0, 50.0, 430.0, 320.0,
                    );
                let _ = renderer.text(&Point{x:50.0, y:210.0}, "Congratuation. ", 'l', 18, 'l');
                let _ = renderer.text(&Point{x:50.0, y:245.0}, "GAME CLEAR!!", 'l', 18, 'l');
            }
            _=> {}
        }
        if let Some(machine) = &self.machine {
            machine.draw(renderer);
        }
    }
}