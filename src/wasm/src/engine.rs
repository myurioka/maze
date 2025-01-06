use crate::browser::{self, LoopClosure};
//use num_traits::FromPrimitive;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::channel::{
    mpsc::{unbounded, UnboundedReceiver},
    oneshot::channel,
};
//use serde::Deserialize;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Mutex};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement, MouseEvent};

const FRAME_SIZE: f64 = 1.0 / 60.0 * 20000.0;
//const FRAME_SIZE: f64 = 1.0 / 60.0 * 1000.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
pub const SCREEN_WIDTH: f32 = 450.0;
pub const DEFAULT_COLOR: &str = "green";
pub const KIMIDORI_COLOR: &str = "#b8d200";
pub const LIGHT_GREEN_COLOR: &str = "#e2eec5";
pub const GREEN_DARK_HEAVY: &str = "#080d08";
pub const GREEN_DARK_MIDDLE: &str = "#0f170f";
pub const GREEN_DARK_LIGHT: &str = "#111f11";
pub const LIGHTCYAN: &str = "#e0ffff";


/*
#[derive(Deserialize, Clone)]
pub struct SheetRect {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}
*/

/*
#[derive(Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Cell {
    pub frame: SheetRect,
    pub sprite_source_size: SheetRect,
}

#[derive(Deserialize, Clone)]
pub struct Sheet {
    pub frames: HashMap<String, Cell>,
}
*/

/*
#[derive(Default)]
pub struct Rect {
    pub position: Point,
    pub width: i16,
    pub height: i16,
}
impl Rect {
    pub const fn new(position: Point, width: i16, height: i16) -> Self {
        Rect {
            position,
            width,
            height,
        }
    }
}
*/

#[derive(Clone, Copy, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}
/*
impl Point{
    pub fn new(x: f32, y: f32) -> Point {
        Point { x: x, y: y }
    }
}
*/

pub struct Renderer {
    context: CanvasRenderingContext2d,
}

impl Renderer {
    pub fn clear(&self, point: &Point, width: f32, height: f32) {
        self.context.set_global_alpha(1.0); 
        self.context.clear_rect(
            point.x.into(),
            point.y.into(),
            width as f64,
            height as f64,
        );
    }
    pub fn draw_image(&self, image:&HtmlImageElement, _sx: f32, _sy: f32, _sw: f32, _sh: f32, _dx: f32, _dy: f32, _dw: f32, _dh:f32){
        self.context.begin_path();
        let _ = self.context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                image,
                _sx as f64,
                _sy as f64,
                _sw as f64,
                _sh as f64,
                _dx as f64,
                _dy as f64,
                _dw as f64,
                _dh as f64,
            )
            .expect("Drawing is throwing excecptions! Unrecoverable error.");
    }

    pub fn text(&self, point: &Point, text: &str, align: char, size: usize, color: char) {
        match color {
            'l' => self.context.set_fill_style(&JsValue::from(LIGHT_GREEN_COLOR)),
            'c' => self.context.set_fill_style(&JsValue::from(LIGHTCYAN)),
            _ => self.context.set_fill_style(&JsValue::from(DEFAULT_COLOR)),
        }
        match align {
            'c' => self.context.set_text_align("center"),
            'l' => self.context.set_text_align("left"),
            _ => self.context.set_text_align("center"),
        }
        match size {
            24 => self.context.set_font("24px MyFont"),
            18 => self.context.set_font("18px MyFont"),
            14 => self.context.set_font("14px MyFont"),
            _ => self.context.set_font("18px MyFont"),
        }
        let _ = self.context.fill_text(text, point.x as f64, point.y as f64);
    }
    pub fn rect(&self, p: &Point, width: f32, height: f32, color: &str, alpha: f64) {
        self.context.set_global_alpha(alpha.into()); 
        self.context.set_stroke_style(&JsValue::from(color));
        self.context.set_fill_style(&JsValue::from(color));
        self.context.begin_path();
        self.context.rect(p.x as f64,p.y as f64, width as f64, height as f64);
        self.context.close_path();
        self.context.fill();
    }
    pub fn polygon(&self, a: &Point, b: &Point, c: &Point, d: &Point, color: &char) {
        let mut _color = "";
        match color { 
            'h' => _color = GREEN_DARK_HEAVY,
            'm' => _color = GREEN_DARK_MIDDLE,
            'l' => _color = GREEN_DARK_LIGHT,
            'b' => _color = "black",
            'p' => _color = "pink",
            _ => _color = "black",
        }
        self.context.set_stroke_style(&JsValue::from(DEFAULT_COLOR));
        self.context.set_fill_style(&JsValue::from(_color));
        self.context.begin_path();
        self.context.move_to(a.x as f64, a.y as f64);
        self.context.line_to(b.x as f64, b.y as f64);
        self.context.line_to(c.x as f64, c.y as f64);
        self.context.line_to(d.x as f64, d.y as f64);
        self.context.line_to(a.x as f64, a.y as f64);
        self.context.close_path();
        self.context.stroke();
        self.context.fill();
    }
}
pub async fn load_image(source: &str) -> Result<HtmlImageElement>{
    let image = browser::new_image()?;
    let (complete_tx, complete_rx) = channel::<Result<()>>();
    let success_tx = Rc::new(Mutex::new(Some(complete_tx)));
    let error_tx = Rc::clone(&success_tx);
    let success_callback = browser::closure_once(move || {
        if let Some(success_tx) = success_tx.lock().ok().and_then(|mut opt| opt.take()) {
            if let Err(err) = success_tx.send(Ok(())) {
                error!("Could not send successful image loaded message! {:#?}", err);
            }
        }
    });

    let error_callback: Closure<dyn FnMut(JsValue)> = browser::closure_once(move |err| {
        if let Some(error_tx) = error_tx.lock().ok().and_then(|mut opt| opt.take()) {
            if let Err(err) = error_tx.send(Err(anyhow!("Error Loading Image: {:#?}", err))) {
                error!("Could not send error message on loading image! {:#?}", err);
            }
        }
    });

    image.set_onload(Some(success_callback.as_ref().unchecked_ref()));
    image.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
    image.set_src(source);

    complete_rx.await??;

    Ok(image)
}

#[async_trait(?Send)]
pub trait Game {
    async fn initialize(&self) -> Result<Box<dyn Game>>;
    fn update(&mut self, keystate: &KeyState, mousestate: &MouseState);
    fn draw(&self, renderer: &Renderer);
}

pub struct GameLoop {
    last_frame: f64,
    accumulated_delta: f64,
}
type SharedLoopClosure = Rc<RefCell<Option<LoopClosure>>>;

impl GameLoop {
    pub async fn start(game: impl Game + 'static) -> Result<()> {
        let mut keyevent_receiver = prepare_input()?;
        let mut mouse_receiver = prepare_mouse_input()?;
        let mut game = game.initialize().await?;
        let mut game_loop = GameLoop {
            last_frame: browser::now()?.into(),
            accumulated_delta: 0.0,
        };

        let renderer = Renderer {
            context: browser::context()?,
        };

        let f: SharedLoopClosure = Rc::new(RefCell::new(None));
        let g = f.clone();

        let mut keystate = KeyState::new();
        let mut mousestate = MouseState::new(browser::canvas()?.offset_left());

        *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
            process_input(&mut keystate, &mut keyevent_receiver);
            process_mouse_input(&mut mousestate, &mut mouse_receiver);

            game_loop.accumulated_delta += perf - game_loop.last_frame;
            while game_loop.accumulated_delta > FRAME_SIZE {
                game.update(&keystate, &mousestate);
                game_loop.accumulated_delta -= FRAME_SIZE;
            }
            game_loop.last_frame = perf;
            game.draw(&renderer);

            let _= browser::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        browser::request_animation_frame(
            g.borrow()
                .as_ref()
                .ok_or_else(|| anyhow!("GameLoop: Loop is None"))?,
        )?;
        Ok(())
    }
}

pub struct MouseState {
    x: i32, // client_x
    y: i32, // client_y
    s: bool,// true: pressed, false: unpressed
    offset_x: i32, // Canvas offset
}
impl MouseState {
    fn new(_offset_x: i32) -> Self {
        return MouseState {
            x: 0,
            y: 0,
            s: false,
            offset_x: _offset_x,
        };
    }
    pub fn is_pressed(&self) -> bool {
        self.s
    }
    const W:i32 = SCREEN_WIDTH as i32 / 3;
    const H:i32 = SCREEN_HEIGHT as i32 / 3;
    pub fn mouse_pressed(&self) -> &str {
        if self.s {
            if  self.x - self.offset_x < Self::W && self.y > Self::H &&self.y < 2 * Self::H {
                log!("PASS ArrowLeft");
                return "ArrowLeft";
            }
            /*
            } else if self.x > Self::W && self.x < 2 * Self::W && self.y < Self::H {
                log!("PASS ArrowUp x:{}, y:{}", self.x, self.y);
                //return "ArrowUp";
            } else if self.x >Self:: W && self.x < 2 * Self::W && self.y > Self::H && self.y < 2 * Self::H {
                log!("PASS SPACE");
                //return "Space";
            } else if self.x > 2 * Self::W && self.y > Self::H && self.y < 2 * Self::H {
                log!("PASS ArrowRight");
                //return "ArrowRight";
            } else if self.x > Self::W && self.x < 2 * Self::W && self.y > 2 * Self::H {
                log!("PASS ArrowDown");
                //return "ArrowDown";
            }
            */
        }
        //log!("PASS NONE");
        return "";
    }
    fn set_pressed(&mut self, _x: i32, _y: i32 ) {
        self.x = _x;
        self.y = _y;
        self.s = true;
    }

    fn set_released(&mut self) {
        self.s = false;
    }
}
enum MousePress {
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
}

fn process_mouse_input(state: &mut MouseState, mouse_receiver: &mut UnboundedReceiver<MousePress>) {
    loop {
        match mouse_receiver.try_next() {
            Ok(None) => break,
            Err(_err) => break,
            Ok(Some(evt)) => match evt{
                MousePress::MouseDown(evt) =>
                    state.set_pressed(evt.client_x(), evt.client_y() ),
                MousePress::MouseUp(evt) =>
                    state.set_released(),
            },
        };
    }
}
// For Mouse Input
fn prepare_mouse_input() -> Result<UnboundedReceiver<MousePress>> {
    let (mousedown_sender, mouse_receiver) = unbounded();
    let mousedown_sender = Rc::new(RefCell::new(mousedown_sender));
    let mouseup_sender = Rc::clone(&mousedown_sender);
    let onmousedown = browser::closure_wrap(Box::new(move |mousecode: MouseEvent| {
        let _ = mousedown_sender
            .borrow_mut()
            .start_send(MousePress::MouseDown(mousecode));
    }) as Box<dyn FnMut(MouseEvent)>);

    let onmouseup = browser::closure_wrap(Box::new(move |mousecode: MouseEvent| {
        let _ = mouseup_sender
            .borrow_mut()
            .start_send(MousePress::MouseUp(mousecode));
    }) as Box<dyn FnMut(MouseEvent)>);

    browser::canvas()?.set_onmousedown(Some(onmousedown.as_ref().unchecked_ref()));
    browser::canvas()?.set_onmouseup(Some(onmouseup.as_ref().unchecked_ref()));
    onmousedown.forget();
    onmouseup.forget();

    Ok(mouse_receiver)
}

pub struct KeyState {
    pressed_keys: HashMap<String,
    web_sys::KeyboardEvent>,
}

impl KeyState {
    fn new() -> Self {
        return KeyState {
            pressed_keys: HashMap::new(),
        };
    }
    pub fn is_pressed(&self, code: &str) -> bool {
        self.pressed_keys.contains_key(code)
    }

    fn set_pressed(&mut self, code: &str, event: web_sys::KeyboardEvent) {
        self.pressed_keys.insert(code.into(), event);
    }

    fn set_released(&mut self, code: &str) {
        self.pressed_keys.remove(code.into());
    }
}

enum KeyPress {
    KeyUp(web_sys::KeyboardEvent),
    KeyDown(web_sys::KeyboardEvent),
}

fn process_input(state: &mut KeyState, keyevent_receiver: &mut UnboundedReceiver<KeyPress>) {
    loop {
        match keyevent_receiver.try_next() {
            Ok(None) => break,
            Err(_err) => break,
            Ok(Some(evt)) => match evt {
                KeyPress::KeyUp(evt) => state.set_released(&evt.code()),
                KeyPress::KeyDown(evt) => state.set_pressed(&evt.code(), evt),
            },
        };
    }
}

// For Keypress Input
fn prepare_input() -> Result<UnboundedReceiver<KeyPress>> {
    let (keydown_sender, keyevent_receiver) = unbounded();
    let keydown_sender = Rc::new(RefCell::new(keydown_sender));
    let keyup_sender = Rc::clone(&keydown_sender);
    let onkeydown = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        let _ = keydown_sender
            .borrow_mut()
            .start_send(KeyPress::KeyDown(keycode));
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    let onkeyup = browser::closure_wrap(Box::new(move |keycode: web_sys::KeyboardEvent| {
        let _ = keyup_sender
            .borrow_mut()
            .start_send(KeyPress::KeyUp(keycode));
    }) as Box<dyn FnMut(web_sys::KeyboardEvent)>);

    browser::canvas()?.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    browser::canvas()?.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    onkeydown.forget();
    onkeyup.forget();

    Ok(keyevent_receiver)
}