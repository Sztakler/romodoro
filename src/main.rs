use anyhow::Context;
use chrono::Local;
use clap::Parser;
use notify_rust::{Notification, NotificationHandle};
use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

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
    #[arg(short, long, default_value_t = 5)]
    break_time: u32,
}

fn update_notification(handle: &mut NotificationHandle, summary: &str, body: &str) {
    handle.summary(summary);
    handle.body(body);
    handle.update();
}

fn send_notification(summary: &str, body: &str) -> anyhow::Result<()> {
    Notification::new()
        .summary(summary)
        .body(body)
        .timeout(5000)
        .show()
        .context("Couldn't send the notification")?;
    Ok(())
}

fn run_timer(minutes: u32, message: &str) -> anyhow::Result<()> {
    let total_seconds = minutes * 60;

    let mut notification = Notification::new()
        .summary(message)
        .body("Time to focus!")
        .timeout(0)
        .show()
        .context("Couldn't send the notification")?;

    for i in (1..=total_seconds).rev() {
        let minutes = i / 60;
        let seconds = i % 60;
        let time_str = format!("{:02}:{:02} remaining.", minutes, seconds);

        print!("\r{}: {}", message, time_str);
        update_notification(&mut notification, message, &time_str);

        io::stdout()
            .flush()
            .context("Error while flushing stdout buffer")?;
        thread::sleep(Duration::from_millis(100));
    }

    print!("\r{}: 00:00 remaining.\n", message);
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let start_time = Instant::now();
    let start_display = chrono::Local::now().format("%H:%M:%S").to_string();

    println!("Starting Pomodoro cycle: {} sessions.", args.count);

    for i in 1..=args.count {
        println!("\n--- Session {}/{} ---", i, args.count);

        send_notification("🦀 Pomodoro", "Time to focus!")?;
        run_timer(args.work_time, "🚀 Work")?;

        if i < args.count {
            send_notification("🦀 Pomodoro", "Take a break!")?;
            run_timer(args.break_time, "☕ Break")?;
        }
    }

    let end_display = chrono::Local::now().format("%H:%M:%S").to_string();
    let duration = start_time.elapsed();

    let hours = duration.as_secs() / 3600;
    let mins = (duration.as_secs() % 3600) / 60;
    let secs = duration.as_secs() % 60;

    send_notification("😿 Finished!", "I'm tired boss. 😿")?;
    println!("\n\r--- 😿 I'm tired boss. 😿 ---");
    println!(
        "\n\rStarted at: {}\nFinished at: {}\nTotal time spent: {}h {}m {}s",
        start_display, end_display, hours, mins, secs
    );
    Ok(())
}
