use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "cbonsai", version, about = "终端ASCII盆景生成CLI工具")]
pub struct Cli {
    #[arg(short = 'l', long, help = "启用逐字符生长动画模式")]
    pub live: bool,

    #[arg(short = 'i', long, help = "无限循环动画模式")]
    pub infinite: bool,

    #[arg(short = 's', long, help = "随机种子，用于固定生成结果")]
    pub seed: Option<u64>,

    #[arg(short = 'W', long, default_value_t = 80, help = "画布宽度")]
    pub width: usize,

    #[arg(short = 'H', long, default_value_t = 40, help = "画布高度")]
    pub height: usize,

    #[arg(short = 'd', long, default_value_t = 3, value_range!(1..=5), help = "枝叶密度 (1-5)")]
    pub density: u8,

    #[arg(long, help = "前景色 (black, red, green, yellow, blue, magenta, cyan, white)")]
    pub fg_color: Option<String>,

    #[arg(long, help = "背景色 (black, red, green, yellow, blue, magenta, cyan, white)")]
    pub bg_color: Option<String>,

    #[arg(short = 'o', long, help = "导出为txt文件路径")]
    pub output: Option<String>,

    #[arg(short = 'q', long, help = "静默屏保模式，无多余输出")]
    pub quiet: bool,

    #[arg(long, help = "生长动画每帧延迟毫秒数", default_value_t = 15)]
    pub delay: u64,

    #[arg(long, help = "无限循环模式下每棵树之间的间隔秒数", default_value_t = 3)]
    pub interval: u64,
}

impl Cli {
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}
