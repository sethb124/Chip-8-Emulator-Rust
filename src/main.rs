use std::{env::args, fs::File, process::ExitCode, thread::sleep, time::Duration};

use minifb::Key;

use cpu::{Cpu, Mode};
use display::Display;

mod audio;
mod cpu;
mod display;
mod instruction;

// const SCALE: u64 = 10000;
const IPS: u64 = 720;
// const IPS: u64 = 720 * SCALE;
// const FPS: u64 = 6000;
// const IPS: u64 = 720;
// const IPS: u64 = 20000;
const FPS: u64 = 60;
// const FPS: u64 = 60 * SCALE;
const CPF: u64 = IPS / FPS;
// const CPF: u64 = 5000;

fn main() -> ExitCode {
    let exec_name = args().next().unwrap();
    let mode = if let Some(arg) = args().nth(1) {
        match arg.as_str() {
            "c" | "cosmac" => Mode::Cosmac,
            "s" | "super" => Mode::Super,
            "x" | "xo" => Mode::Xo,
            _ => {
                eprintln!("Unknown mode: {arg}");
                return ExitCode::FAILURE;
            }
        }
    } else {
        eprintln!("Usage: {exec_name} <mode> <file>");
        return ExitCode::FAILURE;
    };
    let Some(fname) = args().nth(2) else {
        eprintln!("Usage: {exec_name} <mode> <file>");
        return ExitCode::FAILURE;
    };
    let Ok(mut program) = File::open(&fname) else {
        eprintln!("Unable to open file: {fname}");
        return ExitCode::FAILURE;
    };
    let mut cpu = Cpu::with_mode(mode);
    if cpu.load(&mut program).is_err() {
        eprintln!("Unable to read file: {}", fname);
        return ExitCode::FAILURE;
    }
    let mut disp = Display::with_fps(FPS as usize);
    // let mut disp = Display::with_fps(60);
    let mut cycs = 0;
    while disp.window.is_open() && !disp.window.is_key_down(Key::Escape) {
        let ins = cpu.fetch();
        cpu.execute(&ins, &mut disp);
        cycs += 1;
        if cycs == CPF {
            disp.update();
            cpu.dec_timers();
            cycs = 0;
        } else {
            disp.just_updated = false;
        }
        sleep(Duration::from_nanos(1e9 as u64 / IPS));
    }
    ExitCode::SUCCESS
}
