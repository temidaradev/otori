extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

mod emulators {
    pub mod chip8;
}

use emulators::chip8::run_chip8_emulator;
use sdl2::render::TextureQuery;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

    let window = video_subsystem
        .window("Otori", 400, 300)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let font_path = "../assets/PublicPixel.ttf";
    let font = ttf_context.load_font(font_path, 32).unwrap();

    let menu_items = ["Chip-8", "Exit"];
    let mut selected = 0;

    'menu: loop {
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        for (i, item) in menu_items.iter().enumerate() {
            let color = if i == selected {
                Color::RGB(255, 255, 0)
            } else {
                Color::RGB(255, 255, 255)
            };
            let surface = font.render(item).blended(color).unwrap();
            let texture_creator = canvas.texture_creator();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            let TextureQuery { width, height, .. } = texture.query();
            let target = Rect::new(100, 80 + (i as i32) * 60, width, height);
            canvas.copy(&texture, None, Some(target)).unwrap();
        }

        canvas.present();

        let events: Vec<_> = event_pump.poll_iter().collect();
        for event in events {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'menu,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    selected = (selected + 1) % menu_items.len();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if selected == 0 {
                        selected = menu_items.len() - 1;
                    } else {
                        selected -= 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => match selected {
                    0 => {
                        let rom_path = "../roms/PONG";
                        run_chip8_emulator(&sdl_context, &mut event_pump, rom_path);
                    }
                    1 => break 'menu,
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
