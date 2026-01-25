use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple Pomodoro timer in Rust")]
struct Args {
    // Number of sessions (pomodoros)
    #[arg(short, long, default_value_t = 2)]
    count: u32,

    // Work time (in minutes)
    #[arg(short, long, default_value_t = 1)]
    work_time: u32,

    // Break time (in minutes)
    #[arg(short, long, default_value_t = 1)]
    break_time: u32,
}

fn run_timer(minutes: u32, message: &str) {
    let total_seconds = minutes * 60;

    for i in (1..=total_seconds).rev() {
        let minutes = i / 60;
        let seconds = i % 60;

        print!("\r{} {:02}:{:02} remaining.", message, minutes, seconds);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(20));
    }

    print!("\r{} 00:00 remaining.\n", message);
}

fn main() {
    let args = Args::parse();

    println!("Starting Pomodoro cycle: {} sessions.", args.count);

    for i in 1..=args.count {
        println!("\n--- Session {}/{} ---", i, args.count);

        run_timer(args.work_time, "🚀 Work:");

        if i < args.count {
            run_timer(args.break_time, "☕ Break:");
        }
    }

    println!("\n\r--- 🎉 Finished! Good job! 🎉 ---");
}
