# Rust Pomodoro Timer 🦀

A simple CLI Pomodoro timer with notifications and keyboard control.

It's nothing much, really. Just a silly little tool I've made while studying for exams. But it's pretty useful imho.

It tracks your study and break time in both terminal window and using notifications, so you can always see how much time during current session is left.

Keyboard input is being handled by a separate thread using signals and `mpsc` to ensure non-blocking behaviour.

And it has some cute emojis 🦀 😿 🚀 ☕.

## Controls

* `Space` / `P`: Pause/Resume
* `Q`: Quit
* `Ctrl+C`: Exit

## Installation

Make sure you have [Rust and Cargo](https://rustup.rs/) installed.

### Option 1: Install directly from GitHub

```bash
cargo install --git https://github.com/Sztakler/romodoro.git

```

### Option 2: Build from source

```bash
git clone https://github.com/Sztakler/romodoro.git
cd romodoro
cargo build --release
cargo install --path .

```

## Usage

Run with default settings (25min work / 5min break / 4 sessions):

```bash
romodoro

```

### Custom Sessions

You can easily customize the timer using flags:

| Flag | Description | Default |
| --- | --- | --- |
| `-w`, `--work-time` | Duration of a work session (minutes) | 25 |
| `-b`, `--break-time` | Duration of a break session (minutes) | 5 |
| `-c`, `--count` | Number of focus sessions | 4 |

**Examples:**

* **Long haul:** 50min work, 10min break, 6 sessions
```bash
romodoro -w 50 -b 10 -c 6

```


* **Quick sprint:** 15min work, 2min break
```bash
romodoro -w 15 -b 2

```



To see all available options, just run:

```bash
romodoro --help

```
