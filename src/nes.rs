
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
pub mod system;
pub mod rom;
pub mod cpu;
pub mod instruction;
pub mod pad;
pub mod ppu;
pub mod video;
use crate::cpu::Cpu;
use crate::system::System;

use crate::ppu::*;
use crate::cpu::*;
use crate::pad::*;


#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn get_screen_width() -> usize {
    VISIBLE_SCREEN_WIDTH
}
#[wasm_bindgen]
pub fn get_screen_height() -> usize {
    VISIBLE_SCREEN_HEIGHT
}
#[wasm_bindgen]
pub fn get_num_of_colors() -> usize {
    NUM_OF_COLOR
}

#[wasm_bindgen]
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum KeyEvent {
    PressA,
    PressB,
    PressSelect,
    PressStart,
    PressUp,
    PressDown,
    PressLeft,
    PressRight,
    ReleaseA,
    ReleaseB,
    ReleaseSelect,
    ReleaseStart,
    ReleaseUp,
    ReleaseDown,
    ReleaseLeft,
    ReleaseRight,
}

#[wasm_bindgen]
pub struct WasmEmulator {
    fb: [[[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
    cpu: Cpu,
    cpu_sys: System,
    ppu: Ppu,
}

impl Default for WasmEmulator {
    fn default() -> Self {
        Self {
            fb: [[[0; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT],
            cpu: Cpu::new(),
            cpu_sys: System::default(),
            ppu: Ppu::default(),
        }
    }
}

#[wasm_bindgen]
impl WasmEmulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmEmulator {
      console_log!("WasmEmulator::new()");
        WasmEmulator::default()
    }
   
    pub fn get_fb_ptr(&self) -> *const [[u8; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH] {
      console_log!("WasmEmulator::get_fb_ptr()");
        self.fb.as_ptr()
    }
 
    pub fn get_fb_size(&self) -> usize {
      console_log!("WasmEmulator::get_fb_size()");
        NUM_OF_COLOR * VISIBLE_SCREEN_WIDTH * VISIBLE_SCREEN_HEIGHT
    }

    pub fn reset(&mut self) {
     console_log!("WasmEmulator::reset()");
        self.fb = [[[0; NUM_OF_COLOR]; VISIBLE_SCREEN_WIDTH]; VISIBLE_SCREEN_HEIGHT];
        self.cpu.reset();
        self.cpu_sys.reset();
        self.ppu.reset();
        self.cpu.interrupt(&mut self.cpu_sys, Interrupt::RESET);
    }

    pub fn load(&mut self, binary: &[u8]) -> bool {
      console_log!("WasmEmulator::load()");
        let success = self
            .cpu_sys
            .rom
            .load_bin(|addr: usize| binary[addr]);
        if success {
            self.reset();
        }
        success
    }

    pub fn step_line(&mut self) {
       
        let mut total_cycle: usize = 0;
        while total_cycle < CYCLE_PER_DRAW_FRAME {
            // for debug
            // console_log!("a:{:02X} x:{:02X} y:{:02X} pc:{:04X} sp:{:02X} p:{:02X} ", self.cpu.a, self.cpu.x, self.cpu.y, self.cpu.pc, self.cpu.s, self.cpu.p);

            let cpu_cycle = usize::from(self.cpu.step(&mut self.cpu_sys));
            if let Some(interrupt) = self.ppu.step(cpu_cycle, &mut self.cpu_sys, &mut self.fb) {
                self.cpu.interrupt(&mut self.cpu_sys, interrupt);
            }
            total_cycle = total_cycle + cpu_cycle;
           
        }
    }
  
    pub fn update_key(&mut self, key: KeyEvent) {
        match key {
            KeyEvent::PressA => self.cpu_sys.pad1.push_button(PadButton::A),
            KeyEvent::PressB => self.cpu_sys.pad1.push_button(PadButton::B),
            KeyEvent::PressSelect => self.cpu_sys.pad1.push_button(PadButton::Select),
            KeyEvent::PressStart => self.cpu_sys.pad1.push_button(PadButton::Start),
            KeyEvent::PressUp => self.cpu_sys.pad1.push_button(PadButton::Up),
            KeyEvent::PressDown => self.cpu_sys.pad1.push_button(PadButton::Down),
            KeyEvent::PressLeft => self.cpu_sys.pad1.push_button(PadButton::Left),
            KeyEvent::PressRight => self.cpu_sys.pad1.push_button(PadButton::Right),

            KeyEvent::ReleaseA => self.cpu_sys.pad1.release_button(PadButton::A),
            KeyEvent::ReleaseB => self.cpu_sys.pad1.release_button(PadButton::B),
            KeyEvent::ReleaseSelect => self.cpu_sys.pad1.release_button(PadButton::Select),
            KeyEvent::ReleaseStart => self.cpu_sys.pad1.release_button(PadButton::Start),
            KeyEvent::ReleaseUp => self.cpu_sys.pad1.release_button(PadButton::Up),
            KeyEvent::ReleaseDown => self.cpu_sys.pad1.release_button(PadButton::Down),
            KeyEvent::ReleaseLeft => self.cpu_sys.pad1.release_button(PadButton::Left),
            KeyEvent::ReleaseRight => self.cpu_sys.pad1.release_button(PadButton::Right),
        }
    }
}