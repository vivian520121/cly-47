use colored::Colorize;
use std::time::Duration;
use crossterm::{
    execute,
    terminal::{Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, size},
    cursor::{MoveTo, Hide, Show},
    event::{self, Event, KeyCode},
};
use crate::bonsai::Bonsai;
use crate::cli::Cli;

pub enum ColorChoice {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl ColorChoice {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "black" => Some(ColorChoice::Black),
            "red" => Some(ColorChoice::Red),
            "green" => Some(ColorChoice::Green),
            "yellow" => Some(ColorChoice::Yellow),
            "blue" => Some(ColorChoice::Blue),
            "magenta" => Some(ColorChoice::Magenta),
            "cyan" => Some(ColorChoice::Cyan),
            "white" => Some(ColorChoice::White),
            _ => None,
        }
    }

    fn apply_fg(&self, s: String) -> colored::ColoredString {
        match self {
            ColorChoice::Black => s.black(),
            ColorChoice::Red => s.red(),
            ColorChoice::Green => s.green(),
            ColorChoice::Yellow => s.yellow(),
            ColorChoice::Blue => s.blue(),
            ColorChoice::Magenta => s.magenta(),
            ColorChoice::Cyan => s.cyan(),
            ColorChoice::White => s.white(),
        }
    }

    fn apply_bg(&self, s: colored::ColoredString) -> colored::ColoredString {
        match self {
            ColorChoice::Black => s.on_black(),
            ColorChoice::Red => s.on_red(),
            ColorChoice::Green => s.on_green(),
            ColorChoice::Yellow => s.on_yellow(),
            ColorChoice::Blue => s.on_blue(),
            ColorChoice::Magenta => s.on_magenta(),
            ColorChoice::Cyan => s.on_cyan(),
            ColorChoice::White => s.on_white(),
        }
    }
}

pub fn apply_colors(text: &str, fg: Option<&ColorChoice>, bg: Option<&ColorChoice>) -> String {
    match (fg, bg) {
        (None, None) => text.to_string(),
        (Some(fg_c), None) => {
            let mut result = String::new();
            for c in text.chars() {
                if c == '\n' {
                    result.push('\n');
                } else {
                    let colored = fg_c.apply_fg(c.to_string());
                    result.push_str(&format!("{}", colored));
                }
            }
            result
        }
        (None, Some(bg_c)) => {
            let mut result = String::new();
            for c in text.chars() {
                if c == '\n' {
                    result.push('\n');
                } else {
                    let colored = bg_c.apply_bg(c.to_string().white());
                    result.push_str(&format!("{}", colored));
                }
            }
            result
        }
        (Some(fg_c), Some(bg_c)) => {
            let mut result = String::new();
            for c in text.chars() {
                if c == '\n' {
                    result.push('\n');
                } else {
                    let colored = fg_c.apply_fg(c.to_string());
                    let colored = bg_c.apply_bg(colored);
                    result.push_str(&format!("{}", colored));
                }
            }
            result
        }
    }
}

pub fn render_static(bonsai: &Bonsai, fg: Option<&ColorChoice>, bg: Option<&ColorChoice>) {
    let text = bonsai.to_string();
    let colored = apply_colors(&text, fg, bg);
    println!("{}", colored);
}

pub fn render_live(bonsai: &Bonsai, fg: Option<&ColorChoice>, bg: Option<&ColorChoice>, delay_ms: u64, quiet: bool) {
    let growth_order = &bonsai.growth_order;
    let mut canvas: Vec<Vec<char>> = vec![vec![' '; bonsai.width]; bonsai.height];
    let delay = Duration::from_millis(delay_ms);

    if !quiet {
        let _ = execute!(std::io::stdout(), EnterAlternateScreen, Hide);
    }

    for (x, y, c) in growth_order {
        canvas[*y][*x] = *c;

        if !quiet {
            let _ = execute!(std::io::stdout(), MoveTo(0, 0), Clear(ClearType::All));
        }

        let mut text = String::new();
        for row in &canvas {
            for ch in row {
                text.push(*ch);
            }
            text.push('\n');
        }
        let colored = apply_colors(&text, fg, bg);
        print!("{}", colored);

        std::thread::sleep(delay);
    }

    if !quiet {
        std::thread::sleep(Duration::from_millis(500));
        let _ = execute!(std::io::stdout(), Show, LeaveAlternateScreen);
    }
}

pub fn render_infinite(cli: &Cli, fg: Option<ColorChoice>, bg: Option<ColorChoice>) {
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use crate::bonsai::{Bonsai, BonsaiConfig, random_tree_style, random_pot_style};

    let _ = execute!(std::io::stdout(), EnterAlternateScreen, Hide);

    let mut seed_counter: u64 = cli.seed.unwrap_or(0);

    loop {
        let mut rng = ChaCha8Rng::seed_from_u64(seed_counter);
        let tree_style = random_tree_style(&mut rng);
        let pot_style = random_pot_style(&mut rng);

        let config = BonsaiConfig {
            width: cli.width,
            height: cli.height,
            density: cli.density,
            tree_style,
            pot_style,
        };

        let bonsai = Bonsai::new(&config, Some(seed_counter));

        if cli.live {
            let growth_order = bonsai.growth_order.clone();
            let mut canvas: Vec<Vec<char>> = vec![vec![' '; bonsai.width]; bonsai.height];
            let delay = Duration::from_millis(cli.delay);

            for (x, y, c) in &growth_order {
                canvas[*y][*x] = *c;

                let _ = execute!(std::io::stdout(), MoveTo(0, 0), Clear(ClearType::All));

                let mut text = String::new();
                for row in &canvas {
                    for ch in row {
                        text.push(*ch);
                    }
                    text.push('\n');
                }
                let colored = apply_colors(&text, fg.as_ref(), bg.as_ref());
                print!("{}", colored);

                std::thread::sleep(delay);

                if event::poll(Duration::from_millis(0)).unwrap_or(false) {
                    if let Event::Key(key) = event::read().unwrap_or(Event::Key(event::KeyEvent::new(KeyCode::Null, event::KeyModifiers::NONE))) {
                        if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                            let _ = execute!(std::io::stdout(), Show, LeaveAlternateScreen);
                            return;
                        }
                    }
                }
            }
        } else {
            let _ = execute!(std::io::stdout(), MoveTo(0, 0), Clear(ClearType::All));
            render_static(&bonsai, fg.as_ref(), bg.as_ref());
        }

        let sleep_start = std::time::Instant::now();
        let interval = Duration::from_secs(cli.interval);
        while sleep_start.elapsed() < interval {
            if event::poll(Duration::from_millis(100)).unwrap_or(false) {
                if let Event::Key(key) = event::read().unwrap_or(Event::Key(event::KeyEvent::new(KeyCode::Null, event::KeyModifiers::NONE))) {
                    if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                        let _ = execute!(std::io::stdout(), Show, LeaveAlternateScreen);
                        return;
                    }
                }
            }
        }

        seed_counter = seed_counter.wrapping_add(1);
    }
}

pub fn get_terminal_size() -> (usize, usize) {
    if let Ok((w, h)) = size() {
        (w as usize, h as usize)
    } else {
        (80, 40)
    }
}

pub fn render_screensaver(cli: &Cli, fg: Option<ColorChoice>, bg: Option<ColorChoice>) {
    let (tw, th) = get_terminal_size();
    let mut effective_cli = cli.clone();
    effective_cli.width = tw;
    effective_cli.height = th;
    effective_cli.infinite = true;
    effective_cli.quiet = true;
    render_infinite(&effective_cli, fg, bg);
}
