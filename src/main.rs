mod cli;
mod bonsai;
mod renderer;

use cli::Cli;
use bonsai::{Bonsai, BonsaiConfig, random_tree_style, random_pot_style};
use renderer::{ColorChoice, render_static, render_live, render_infinite, render_screensaver};
use rand::{SeedableRng, Rng};
use rand_chacha::ChaCha8Rng;
use std::fs;
use std::io::Write;

fn main() {
    let cli = Cli::parse_args();

    let fg_color = cli.fg_color.as_ref().and_then(|s| ColorChoice::parse(s));
    let bg_color = cli.bg_color.as_ref().and_then(|s| ColorChoice::parse(s));

    if cli.quiet && !cli.live && !cli.infinite {
        render_screensaver(&cli, fg_color, bg_color);
        return;
    }

    if cli.infinite {
        render_infinite(&cli, fg_color, bg_color);
        return;
    }

    let mut rng = match cli.seed {
        Some(s) => ChaCha8Rng::seed_from_u64(s),
        None => ChaCha8Rng::from_entropy(),
    };

    let tree_style = random_tree_style(&mut rng);
    let pot_style = random_pot_style(&mut rng);

    let config = BonsaiConfig {
        width: cli.width,
        height: cli.height,
        density: cli.density,
        tree_style,
        pot_style,
    };

    let seed_for_bonsai = cli.seed.unwrap_or_else(|| rng.gen());
    let bonsai = Bonsai::new(&config, Some(seed_for_bonsai));

    if let Some(output_path) = &cli.output {
        let text = bonsai.to_string();
        match fs::File::create(output_path) {
            Ok(mut f) => {
                if let Err(e) = f.write_all(text.as_bytes()) {
                    eprintln!("写入文件失败: {}", e);
                } else if !cli.quiet {
                    println!("盆景已保存到: {}", output_path);
                }
            }
            Err(e) => {
                eprintln!("创建文件失败: {}", e);
            }
        }
    }

    if cli.live {
        render_live(&bonsai, fg_color.as_ref(), bg_color.as_ref(), cli.delay, cli.quiet);
    } else if !cli.quiet {
        render_static(&bonsai, fg_color.as_ref(), bg_color.as_ref());
    } else {
        render_static(&bonsai, fg_color.as_ref(), bg_color.as_ref());
    }
}
