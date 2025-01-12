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
use web_sys::{CanvasRenderingContext2d, HtmlImageElement, TouchEvent, TouchList, Touch};

const FRAME_SIZE: f64 = 1.0 / 60.0 * 10000.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
pub const SCREEN_WIDTH: f32 = 450.0;
pub const DEFAULT_COLOR: &str = "rgba(0,128, 0)";
pub const KIMIDORI_COLOR: &str = "rgba(184,210,0,1.0)";
pub const LIGHT_GREEN_COLOR: &str = "rgba(226,238,197,1.0)";
pub const GREEN_DARK_HEAVY: &str = "rgba(8,13,8,1.0)";
pub const GREEN_DARK_MIDDLE: &str = "rgba(15,23,15,1.0)";
pub const GREEN_DARK_LIGHT: &str = "rgba(17,31,17,1.0)";
pub const LIGHTCYAN: &str = "rgba(17,31,17,1.0)";

/*
#[derive(Deserialize, Clone)]
pub struct SheetRect {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}
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
            'l' => self.context.set_fill_style_str(LIGHT_GREEN_COLOR),
            'c' => self.context.set_fill_style_str(LIGHTCYAN),
            _ => self.context.set_fill_style_str(DEFAULT_COLOR),
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
        self.context.set_stroke_style_str(color);
        self.context.set_fill_style_str(color);
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
        self.context.set_stroke_style_str(DEFAULT_COLOR);
        self.context.set_fill_style_str(_color);
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
    fn update(&mut self, keystate: &KeyState, touchstate: &TouchState);
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
        let mut touch_receiver = prepare_touch_input()?;
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
        let mut touchstate = TouchState::new(browser::canvas()?.offset_left());

        *g.borrow_mut() = Some(browser::create_raf_closure(move |perf: f64| {
            process_input(&mut keystate, &mut keyevent_receiver);
            process_touch_input(&mut touchstate, &mut touch_receiver);

            game_loop.accumulated_delta += perf - game_loop.last_frame;
            while game_loop.accumulated_delta > FRAME_SIZE {
                game.update(&keystate, &touchstate);
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

pub struct TouchState {
    x: i32, // client_x
    y: i32, // client_y
    s: bool,// true: pressed, false: unpressed
    offset_x: i32, // Canvas offset
}
impl TouchState {
    const W:i32 = SCREEN_WIDTH as i32 / 3;
    const H:i32 = SCREEN_HEIGHT as i32 / 3;
    fn new(_offset_x: i32) -> Self {
        return TouchState {
            x: 0,
            y: 0,
            s: false,
            offset_x: _offset_x,
        };
    }
    pub fn is_pressed(&self) -> bool {
        self.s
    }
    pub fn touch_pressed(&self) -> &str {
        let _x = self.x - self.offset_x;
        let _y = self.y;
        if self.s {
            if  _x < Self::W && _y > Self::H && _y < 2 * Self::H {
                return "ArrowLeft";
            } else if _x > Self::W && _x < 2 * Self::W && _y < Self::H {
                return "ArrowUp";
            } else if _x >Self:: W && _x < 2 * Self::W && _y > Self::H && _y < 2 * Self::H {
                return "Space";
            } else if _x > 2 * Self::W && _y > Self::H && _y < 2 * Self::H {
                return "ArrowRight";
            } else if _x > Self::W && _x < 2 * Self::W && _y > 2 * Self::H {
                return "ArrowDown";
            }
        }
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
enum TouchPress {
    TouchStart(TouchEvent),
    TouchEnd(),
}

fn process_touch_input(state: &mut TouchState, touch_receiver: &mut UnboundedReceiver<TouchPress>) {
    loop {
        match touch_receiver.try_next() {
            Ok(None) => break,
            Err(_err) => break,
            Ok(Some(evt)) => match evt{
                TouchPress::TouchStart(evt) => {
                    let _list:TouchList = evt.touches();
                    let _touch:Touch = _list.item(0).unwrap();
                    state.set_pressed(_touch.client_x(), _touch.client_y());
                }
                TouchPress::TouchEnd() => {
                    state.set_released();
                },
            },
        };
    }
}

// For Touch Input
fn prepare_touch_input() -> Result<UnboundedReceiver<TouchPress>> {
    let (touch_start_sender, touch_receiver) = unbounded();
    let touch_start_sender = Rc::new(RefCell::new(touch_start_sender));
    let touch_end_sender = Rc::clone(&touch_start_sender);
    let ontouchstart = browser::closure_wrap(Box::new(move |_touchcode: TouchEvent| {
        let _ = touch_start_sender
            .borrow_mut()
            .start_send(TouchPress::TouchStart(_touchcode));
    }) as Box<dyn FnMut(TouchEvent)>);
    let ontouchend = browser::closure_wrap(Box::new(move |_touchcode: TouchEvent| {
        let _ = touch_end_sender
            .borrow_mut()
            .start_send(TouchPress::TouchEnd());
    }) as Box<dyn FnMut(TouchEvent)>);

    browser::canvas()?.set_ontouchstart(Some(ontouchstart.as_ref().unchecked_ref()));
    browser::canvas()?.set_ontouchend(Some(ontouchend.as_ref().unchecked_ref()));
    ontouchstart.forget();
    ontouchend.forget();

    Ok(touch_receiver)
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