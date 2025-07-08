use chip8_core::*;
use js_sys::Uint8Array;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

#[wasm_bindgen]
pub struct EmuWasm {
    chip8: Emu,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl EmuWasm {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<EmuWasm, JsValue> {
        let chip8 = Emu::new();

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("screen").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();
        Ok(EmuWasm { chip8, ctx })
    }

    #[wasm_bindgen]
    pub fn draw_screen(&mut self, scale: usize) {
        let disp = self.chip8.get_display();
        for (i, item) in disp.iter().enumerate().take(SCREEN_WIDTH * SCREEN_HEIGHT) {
            if *item {
                let x = i % SCREEN_WIDTH;
                let y = i / SCREEN_WIDTH;
                self.ctx.fill_rect(
                    (x * scale) as f64,
                    (y * scale) as f64,
                    scale as f64,
                    scale as f64,
                );
            }
        }
    }

    #[wasm_bindgen]
    pub fn load_game(&mut self, data: Uint8Array) -> Result<(), JsValue> {
        self.chip8
            .load(&data.to_vec())
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen]
    pub fn tick(&mut self) -> Result<(), JsValue> {
        self.chip8
            .tick()
            .map_err(|e| JsValue::from_str(&format!("{:?}", e)))
    }

    #[wasm_bindgen]
    pub fn timer_tick(&mut self) {
        self.chip8.timer_tick();
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.chip8.reset();
    }

    #[wasm_bindgen]
    pub fn keypress(&mut self, evt: KeyboardEvent, pressed: bool) {
        let key = evt.key();
        if let Some(k) = key2btn(&key) {
            self.chip8.keypress(k, pressed);
        }
    }
}

fn key2btn(key: &str) -> Option<usize> {
    match key {
        "1" => Some(0x1),
        "2" => Some(0x2),
        "3" => Some(0x3),
        "4" => Some(0xc),
        "q" => Some(0x4),
        "w" => Some(0x5),
        "e" => Some(0x6),
        "r" => Some(0xd),
        "a" => Some(0x7),
        "s" => Some(0x8),
        "d" => Some(0x9),
        "f" => Some(0xe),
        "z" => Some(0xa),
        "x" => Some(0x0),
        "c" => Some(0xb),
        "v" => Some(0xf),
        _ => None,
    }
}
