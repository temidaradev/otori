import init, * as wasm from "./wasm.js"


const WIDTH = 64
const HEIGHT = 32
const SCALE = 15
const TICKS_PER_FRAME = 10
let anim_frame = 0
const canvas = document.getElementById("screen")
canvas.width = WIDTH * SCALE
canvas.height = HEIGHT * SCALE
const ctx = canvas.getContext("2d")
ctx.fillStyle = "black"
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE)

// List of ROMs (from the roms/ directory)
const ROMS = [
    "15PUZZLE","BLINKY","BLITZ","BRIX","CONNECT4","GUESS","HIDDEN","INVADERS","KALEID","MAZE","MERLIN","MISSILE","PONG","PONG2","PUZZLE","SYZYGY","TANK","TETRIS","TICTAC","UFO","VBRIX","VERS","WIPEOFF"
];

// Populate ROM selector
document.addEventListener("DOMContentLoaded", () => {
    const controls = document.getElementById("controls");
    const select = document.createElement("select");
    select.id = "romSelect";
    ROMS.forEach(rom => {
        const option = document.createElement("option");
        option.value = rom;
        option.textContent = rom;
        select.appendChild(option);
    });
    controls.insertBefore(select, controls.firstChild);
});

function mainloop(chip8) {
    try {
        for (let i = 0; i < TICKS_PER_FRAME; i++) {
            chip8.tick();
        }
        chip8.timer_tick(); // <-- use timer_tick, not tick_timers
        ctx.fillStyle = "black";
        ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE);
        ctx.fillStyle = "white";
        chip8.draw_screen(SCALE);
        anim_frame = window.requestAnimationFrame(() => {
            mainloop(chip8);
        });
    } catch (e) {
        console.error("Emulator error:", e);
        window.cancelAnimationFrame(anim_frame);
        alert("Emulator stopped due to error: " + e);
    }
}

async function run() {
    await init();
    let chip8;
    let loadedRom = null;
    // Add event listeners for ROM selection and Start button
    const startBtn = document.getElementById("startBtn");
    const romSelect = document.getElementById("romSelect");
    startBtn.addEventListener("click", async function() {
        if (!romSelect) return;
        const romName = romSelect.value;
        const romResponse = await fetch(`roms/${romName}`);
        const romData = new Uint8Array(await romResponse.arrayBuffer());
        chip8 = new wasm.EmuWasm();
        chip8.load_game(romData);
        document.addEventListener("keydown", function(evt) {
            chip8.keypress(evt, true)
        })
        document.addEventListener("keyup", function(evt) {
            chip8.keypress(evt, false)
        })
        mainloop(chip8);
    });
}
run().catch(console.error)
