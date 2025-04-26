mod level;
mod player;

use crate::level::Level;
use crate::player::Player;
use log::{error, info};
use std::io::stdin;

fn main() {
    println!("Adınızı Giriniz");
    let mut player_name = String::new();

    let res = stdin().read_line(&mut player_name);
    match res {
        Ok(l) => info!("{} byte veri alındı", l),
        Err(e) => error!("Hata: {}", e),
    }

    let persival = Player {
        name: player_name.trim(),
        level: Level::Juniour,
    };

    print!("{:#?}", persival)
}
