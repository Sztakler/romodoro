use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "A simple Pomodoro timer in Rust")]
struct Args {
    // Number of sessions (pomodoros)
    #[arg(short, long, default_value_t = 4)]
    count: u32,

    // Work time (in minutes)
    #[arg(short, long, default_value_t = 25)]
    work_time: u32,

    // Break time (in minutes)
    #[arg(short, long, default_value_t = 25)]
    break_time: u32,
}

fn main() {
    let args = Args::parse();

    println!("Starting Pomodoro session: {} sessions.", args.count)
}
