extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use crate::emulator_handler::chip8::run_chip8_emulator;
use sdl2::render::TextureQuery;
use std::fs;
use std::path::Path;

mod emulator_handler {
    pub mod chip8;
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string()).unwrap();

    show_rom_selection(&sdl_context, &mut event_pump, &ttf_context);
}

fn get_roms() -> Result<Vec<String>, std::io::Error> {
    let mut roms = Vec::new();
    let roms_path = Path::new("roms");

    if roms_path.exists() && roms_path.is_dir() {
        for entry in fs::read_dir(roms_path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if let Some(file_str) = file_name.to_str() {
                        roms.push(file_str.to_string());
                    }
                }
            }
        }
    }

    roms.sort();
    Ok(roms)
}

fn show_rom_selection(
    sdl_context: &sdl2::Sdl,
    event_pump: &mut sdl2::EventPump,
    ttf_context: &sdl2::ttf::Sdl2TtfContext,
) {
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Select ROM", 800, 600)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let roms = match get_roms() {
        Ok(roms) => roms,
        Err(e) => {
            eprintln!("Error reading ROM directory: {}", e);
            return;
        }
    };

    if roms.is_empty() {
        eprintln!("No ROMs found in roms/ directory");
        return;
    }

    let mut selected = 0;
    let mut scroll_offset = 0;
    let max_visible_items = 8;
    let font = ttf_context.load_font("assets/PublicPixel.ttf", 32).unwrap();
    let small_font = ttf_context.load_font("assets/PublicPixel.ttf", 24).unwrap();

    'rom_menu: loop {
        canvas.set_draw_color(Color::RGB(30, 30, 30));
        canvas.clear();

        let title_surface = font
            .render("Select ROM")
            .blended(Color::RGB(255, 255, 255))
            .unwrap();
        let texture_creator = canvas.texture_creator();
        let title_texture = texture_creator
            .create_texture_from_surface(&title_surface)
            .unwrap();
        let TextureQuery {
            width: title_width,
            height: title_height,
            ..
        } = title_texture.query();
        let title_target = Rect::new(
            400 - (title_width as i32) / 2,
            30,
            title_width,
            title_height,
        );
        canvas
            .copy(&title_texture, None, Some(title_target))
            .unwrap();

        if selected < scroll_offset {
            scroll_offset = selected;
        } else if selected >= scroll_offset + max_visible_items {
            scroll_offset = selected - max_visible_items + 1;
        }

        for (i, rom) in roms
            .iter()
            .enumerate()
            .skip(scroll_offset)
            .take(max_visible_items)
        {
            let color = if i == selected {
                Color::RGB(255, 255, 0)
            } else {
                Color::RGB(255, 255, 255)
            };

            let surface = small_font.render(rom).blended(color).unwrap();
            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .unwrap();
            let TextureQuery { width, height, .. } = texture.query();
            let y_pos = 120 + ((i - scroll_offset) as i32) * 50;
            let target = Rect::new(100, y_pos, width, height);
            canvas.copy(&texture, None, Some(target)).unwrap();
        }

        if roms.len() > max_visible_items {
            let scroll_text = format!("{}/{}", selected + 1, roms.len());
            let scroll_surface = small_font
                .render(&scroll_text)
                .blended(Color::RGB(150, 150, 150))
                .unwrap();
            let scroll_texture = texture_creator
                .create_texture_from_surface(&scroll_surface)
                .unwrap();
            let TextureQuery {
                width: scroll_width,
                height: scroll_height,
                ..
            } = scroll_texture.query();
            let scroll_target =
                Rect::new(700 - scroll_width as i32, 550, scroll_width, scroll_height);
            canvas
                .copy(&scroll_texture, None, Some(scroll_target))
                .unwrap();
        }

        canvas.present();

        let events: Vec<_> = event_pump.poll_iter().collect();
        for event in events {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'rom_menu,
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    selected = (selected + 1) % roms.len();
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if selected == 0 {
                        selected = roms.len() - 1;
                    } else {
                        selected -= 1;
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    let rom_path = format!("roms/{}", roms[selected]);
                    run_chip8_emulator(sdl_context, event_pump, &rom_path);
                }
                _ => {}
            }
        }
    }
}
