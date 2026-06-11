use rand::Rng;
use rand_chacha::ChaCha8Rng;
use rand::SeedableRng;
use std::cmp;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TreeStyle {
    Pine,
    Willow,
    Oak,
    Cherry,
    Bamboo,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PotStyle {
    Classic,
    Rectangular,
    Round,
    Decorated,
}

pub struct BonsaiConfig {
    pub width: usize,
    pub height: usize,
    pub density: u8,
    pub tree_style: TreeStyle,
    pub pot_style: PotStyle,
}

pub struct Bonsai {
    pub canvas: Vec<Vec<char>>,
    pub width: usize,
    pub height: usize,
    pub growth_order: Vec<(usize, usize, char)>,
}

impl Bonsai {
    pub fn new(config: &BonsaiConfig, seed: Option<u64>) -> Self {
        let mut rng = match seed {
            Some(s) => ChaCha8Rng::seed_from_u64(s),
            None => ChaCha8Rng::from_entropy(),
        };

        let mut canvas = vec![vec![' '; config.width]; config.height];
        let growth_order = Vec::new();
        let mut bonsai = Bonsai {
            canvas,
            width: config.width,
            height: config.height,
            growth_order,
        };

        let pot_height = 5;
        let tree_bottom = config.height - pot_height - 1;
        let center_x = config.width / 2;

        draw_pot(&mut bonsai, center_x, tree_bottom + 1, config.pot_style, &mut rng);
        draw_tree(&mut bonsai, center_x, tree_bottom, config, &mut rng);

        bonsai
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for row in &self.canvas {
            for c in row {
                result.push(*c);
            }
            result.push('\n');
        }
        result
    }

    fn set_char(&mut self, x: isize, y: isize, c: char) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            let (xu, yu) = (x as usize, y as usize);
            if self.canvas[yu][xu] == ' ' {
                self.canvas[yu][xu] = c;
                self.growth_order.push((xu, yu, c));
            }
        }
    }
}

fn draw_pot(bonsai: &mut Bonsai, cx: usize, top_y: usize, style: PotStyle, rng: &mut ChaCha8Rng) {
    let pot_width = cmp::min(20, bonsai.width / 3);
    let pot_height = 5;
    let half_w = pot_width / 2;

    match style {
        PotStyle::Classic => {
            for i in 0..pot_height {
                let shrink = if i < 2 { 0 } else { i - 1 };
                let left = (cx as isize) - (half_w as isize) + (shrink as isize);
                let right = (cx as isize) + (half_w as isize) - (shrink as isize);
                let y = (top_y + i) as isize;
                for x in left..=right {
                    if x == left || x == right || i == pot_height - 1 {
                        bonsai.set_char(x, y, '#');
                    } else {
                        if i == 0 || (rng.gen::<f32>() > 0.7) {
                            bonsai.set_char(x, y, '~');
                        } else {
                            bonsai.set_char(x, y, '.');
                        }
                    }
                }
            }
            let rim_left = (cx as isize) - (half_w as isize) - 1;
            let rim_right = (cx as isize) + (half_w as isize) + 1;
            for x in rim_left..=rim_right {
                bonsai.set_char(x, top_y as isize, '=');
            }
        }
        PotStyle::Rectangular => {
            for i in 0..pot_height {
                let left = (cx as isize) - (half_w as isize);
                let right = (cx as isize) + (half_w as isize);
                let y = (top_y + i) as isize;
                for x in left..=right {
                    if x == left || x == right || i == 0 || i == pot_height - 1 {
                        bonsai.set_char(x, y, '+');
                    } else {
                        bonsai.set_char(x, y, '-');
                    }
                }
            }
        }
        PotStyle::Round => {
            for i in 0..pot_height {
                let curve = if i == 0 || i == pot_height - 1 { 0 } else { 1 };
                let left = (cx as isize) - (half_w as isize) + (curve as isize);
                let right = (cx as isize) + (half_w as isize) - (curve as isize);
                let y = (top_y + i) as isize;
                for x in left..=right {
                    if x == left || x == right || i == pot_height - 1 {
                        bonsai.set_char(x, y, 'O');
                    } else {
                        bonsai.set_char(x, y, 'o');
                    }
                }
            }
        }
        PotStyle::Decorated => {
            for i in 0..pot_height {
                let shrink = if i < 2 { 0 } else { i - 1 };
                let left = (cx as isize) - (half_w as isize) + (shrink as isize);
                let right = (cx as isize) + (half_w as isize) - (shrink as isize);
                let y = (top_y + i) as isize;
                for x in left..=right {
                    if x == left || x == right {
                        bonsai.set_char(x, y, '║');
                    } else if i == 0 {
                        bonsai.set_char(x, y, '═');
                    } else if i == pot_height - 1 {
                        bonsai.set_char(x, y, '▀');
                    } else {
                        let decor_chars = ['♥', '♦', '♣', '♠', '*', '·'];
                        if rng.gen::<f32>() > 0.6 {
                            let idx = rng.gen_range(0..decor_chars.len());
                            bonsai.set_char(x, y, decor_chars[idx]);
                        }
                    }
                }
            }
            let rim_left = (cx as isize) - (half_w as isize) - 1;
            let rim_right = (cx as isize) + (half_w as isize) + 1;
            bonsai.set_char(rim_left, top_y as isize, '╔');
            bonsai.set_char(rim_right, top_y as isize, '╗');
        }
    }

    let soil_y = (top_y + 1) as isize;
    let soil_left = (cx as isize) - (half_w as isize) + 1;
    let soil_right = (cx as isize) + (half_w as isize) - 1;
    for x in soil_left..=soil_right {
        let soil_chars = ['.', ',', '`', '\''];
        let idx = rng.gen_range(0..soil_chars.len());
        bonsai.set_char(x, soil_y, soil_chars[idx]);
    }
}

fn draw_tree(bonsai: &mut Bonsai, cx: usize, bottom_y: usize, config: &BonsaiConfig, rng: &mut ChaCha8Rng) {
    let tree_height = bottom_y.saturating_sub(3);
    let trunk_height = tree_height / 3;

    match config.tree_style {
        TreeStyle::Pine => draw_pine(bonsai, cx, bottom_y, tree_height, trunk_height, config, rng),
        TreeStyle::Willow => draw_willow(bonsai, cx, bottom_y, tree_height, trunk_height, config, rng),
        TreeStyle::Oak => draw_oak(bonsai, cx, bottom_y, tree_height, trunk_height, config, rng),
        TreeStyle::Cherry => draw_cherry(bonsai, cx, bottom_y, tree_height, trunk_height, config, rng),
        TreeStyle::Bamboo => draw_bamboo(bonsai, cx, bottom_y, tree_height, config, rng),
    }
}

fn draw_pine(bonsai: &mut Bonsai, cx: usize, bottom_y: usize, tree_height: usize, trunk_height: usize, config: &BonsaiConfig, rng: &mut ChaCha8Rng) {
    let density_mult = config.density as isize;
    let trunk_start = bottom_y;
    let trunk_end = bottom_y - trunk_height;

    for y in (trunk_end..=trunk_start).rev() {
        let thickness = if y > trunk_end + trunk_height / 2 { 2 } else { 1 };
        for t in 0..thickness {
            bonsai.set_char(cx as isize - (thickness as isize) / 2 + t as isize, y as isize, '|');
        }
    }

    let foliage_start = trunk_end - 1;
    let layers = 4 + density_mult as usize;
    let base_width = 5 + density_mult * 2;

    for layer in 0..layers {
        let layer_y = foliage_start as isize - (layer as isize) * 3;
        if layer_y < 2 { break; }
        let layer_width = (base_width as isize) - (layer as isize);
        if layer_width < 1 { break; }
        let half_w = layer_width / 2;
        for x in (cx as isize) - half_w..=(cx as isize) + half_w {
            let needle_chars = ['*', '^', 'v', '<', '>', '.'];
            let idx = rng.gen_range(0..needle_chars.len());
            if rng.gen::<f32>() > 0.15 {
                bonsai.set_char(x, layer_y, needle_chars[idx]);
            }
            if rng.gen::<f32>() > 0.5 && layer_y - 1 >= 0 {
                let idx2 = rng.gen_range(0..needle_chars.len());
                bonsai.set_char(x, layer_y - 1, needle_chars[idx2]);
            }
        }
    }
}

fn draw_willow(bonsai: &mut Bonsai, cx: usize, bottom_y: usize, tree_height: usize, trunk_height: usize, config: &BonsaiConfig, rng: &mut ChaCha8Rng) {
    let density_mult = config.density as isize;
    let trunk_start = bottom_y;
    let trunk_end = bottom_y - trunk_height;

    for y in (trunk_end..=trunk_start).rev() {
        let sway = ((y as f32 * 0.2).sin() * 2.0) as isize;
        bonsai.set_char(cx as isize + sway, y as isize, '/');
        bonsai.set_char(cx as isize + sway - 1, y as isize, '\\');
    }

    let canopy_top = trunk_end as isize - (tree_height as isize) / 2;
    let canopy_radius = (8 + density_mult * 3) as isize;

    for dy in -canopy_radius..=canopy_radius {
        for dx in -canopy_radius * 2..=canopy_radius * 2 {
            let dist = ((dx * dx) as f32 / 4.0 + (dy * dy) as f32).sqrt();
            if dist <= canopy_radius as f32 {
                let x = cx as isize + dx;
                let y = canopy_top + dy;
                if y < (trunk_end as isize) && rng.gen::<f32>() > 0.25 {
                    let leaf_chars = [',', '.', '`', '\'', ';'];
                    let idx = rng.gen_range(0..leaf_chars.len());
                    bonsai.set_char(x, y, leaf_chars[idx]);
                }
            }
        }
    }

    let strand_count = 6 + density_mult as usize;
    for s in 0..strand_count {
        let offset = (s as isize - strand_count as isize / 2) * 3;
        let strand_x = cx as isize + offset;
        let strand_length = 8 + rng.gen_range(0..6);
        for l in 0..strand_length {
            let sway = ((l as f32 * 0.5).sin() * 1.5) as isize;
            let y = canopy_top as isize + l as isize;
            if y < bottom_y as isize - 3 {
                let strand_chars = [',', '.', '`'];
                let idx = rng.gen_range(0..strand_chars.len());
                bonsai.set_char(strand_x + sway, y, strand_chars[idx]);
            }
        }
    }
}

fn draw_oak(bonsai: &mut Bonsai, cx: usize, bottom_y: usize, tree_height: usize, trunk_height: usize, config: &BonsaiConfig, rng: &mut ChaCha8Rng) {
    let density_mult = config.density as isize;
    let trunk_start = bottom_y;
    let trunk_end = bottom_y - trunk_height;

    for y in (trunk_end..=trunk_start).rev() {
        let thickness = 2 + if y > trunk_end + trunk_height / 3 { 1 } else { 0 };
        for t in 0..thickness {
            let bark_chars = ['|', 'I', 'l', '!'];
            let idx = rng.gen_range(0..bark_chars.len());
            bonsai.set_char(cx as isize - thickness as isize / 2 + t as isize, y as isize, bark_chars[idx]);
        }
    }

    let branch_count = 3 + density_mult as usize;
    let mut branches = Vec::new();
    for b in 0..branch_count {
        let branch_y = trunk_end as isize - (b as isize) * 2;
        let dir = if b % 2 == 0 { 1 } else { -1 };
        let branch_len = 6 + rng.gen_range(0..6);
        for l in 1..=branch_len {
            let x = cx as isize + dir * l as isize;
            let y = branch_y - (l as isize) / 2;
            if y > 0 {
                bonsai.set_char(x, y, if l % 3 == 0 { '\\' } else { '/' });
                branches.push((x, y));
            }
        }
    }

    let canopy_radius = (10 + density_mult * 3) as isize;
    let canopy_cy = trunk_end as isize - canopy_radius / 2;

    for dy in -canopy_radius..=canopy_radius {
        for dx in -canopy_radius * 2..=canopy_radius * 2 {
            let dist = ((dx * dx) as f32 / 3.5 + (dy * dy) as f32).sqrt();
            if dist <= canopy_radius as f32 && rng.gen::<f32>() > 0.35 {
                let x = cx as isize + dx;
                let y = canopy_cy + dy;
                if y > 0 {
                    let leaf_chars = ['@', '#', '&', '%', '*', 'o'];
                    let idx = rng.gen_range(0..leaf_chars.len());
                    bonsai.set_char(x, y, leaf_chars[idx]);
                }
            }
        }
    }
}

fn draw_cherry(bonsai: &mut Bonsai, cx: usize, bottom_y: usize, tree_height: usize, trunk_height: usize, config: &BonsaiConfig, rng: &mut ChaCha8Rng) {
    let density_mult = config.density as isize;
    let trunk_start = bottom_y;
    let trunk_end = bottom_y - trunk_height;

    for y in (trunk_end..=trunk_start).rev() {
        let thickness = 2;
        for t in 0..thickness {
            let bark_chars = ['|', 'I', 'l'];
            let idx = rng.gen_range(0..bark_chars.len());
            bonsai.set_char(cx as isize - thickness as isize / 2 + t as isize, y as isize, bark_chars[idx]);
        }
    }

    let branch_count = 4 + density_mult as usize;
    for b in 0..branch_count {
        let branch_y = trunk_end as isize - (b as isize) * 2;
        let dir = if b % 2 == 0 { 1 } else { -1 };
        let branch_len = 5 + rng.gen_range(0..5);
        for l in 1..=branch_len {
            let x = cx as isize + dir * l as isize;
            let y = branch_y - (l as isize) / 3;
            if y > 0 {
                bonsai.set_char(x, y, '-');
            }
        }
    }

    let canopy_radius = (9 + density_mult * 3) as isize;
    let canopy_cy = trunk_end as isize - canopy_radius / 2;

    for dy in -canopy_radius..=canopy_radius {
        for dx in -canopy_radius * 2..=canopy_radius * 2 {
            let dist = ((dx * dx) as f32 / 3.5 + (dy * dy) as f32).sqrt();
            if dist <= canopy_radius as f32 {
                let x = cx as isize + dx;
                let y = canopy_cy + dy;
                if y > 0 && rng.gen::<f32>() > 0.3 {
                    let petal_chars = ['*', 'o', 'O', '.', '·'];
                    let idx = rng.gen_range(0..petal_chars.len());
                    bonsai.set_char(x, y, petal_chars[idx]);
                }
            }
        }
    }

    let petal_fall = density_mult as usize * 2;
    for _ in 0..petal_fall {
        let px = (cx as isize) + rng.gen_range(-15..15);
        let py = (canopy_cy + canopy_radius) + rng.gen_range(1..10);
        if py < (bottom_y as isize) - 6 {
            let petals = ['·', '.', '*'];
            let idx = rng.gen_range(0..petals.len());
            bonsai.set_char(px, py, petals[idx]);
        }
    }
}

fn draw_bamboo(bonsai: &mut Bonsai, cx: usize, bottom_y: usize, tree_height: usize, config: &BonsaiConfig, rng: &mut ChaCha8Rng) {
    let density_mult = config.density as isize;
    let stalk_count = 3 + density_mult as usize / 2;

    for s in 0..stalk_count {
        let stalk_x = cx as isize + (s as isize - stalk_count as isize / 2) * 3;
        let stalk_height = tree_height as isize - rng.gen_range(0..8) as isize;
        if stalk_height < 5 { continue; }

        for y in 0..stalk_height {
            let py = bottom_y as isize - y;
            if py < 0 { break; }
            let bamboo_chars = if y % 5 == 0 { '=' } else { '|' };
            bonsai.set_char(stalk_x, py, bamboo_chars);
            if rng.gen::<f32>() > 0.5 {
                bonsai.set_char(stalk_x + 1, py, bamboo_chars);
            }
        }

        let leaf_nodes = 2 + density_mult as usize / 2;
        for ln in 0..leaf_nodes {
            let ly = bottom_y as isize - (ln as isize) * (stalk_height / leaf_nodes as isize) - 2;
            if ly < 0 { continue; }
            let leaf_count = 2 + rng.gen_range(0..3);
            for lf in 0..leaf_count {
                let dir = if lf % 2 == 0 { 1 } else { -1 };
                let leaf_len = 3 + rng.gen_range(0..4);
                for ll in 1..=leaf_len {
                    let lx = stalk_x + dir * ll as isize;
                    let ly2 = ly - ll as isize / 2;
                    if ly2 >= 0 {
                        let leaf_chars = ['/', '\\', 'V', '^'];
                        let idx = rng.gen_range(0..leaf_chars.len());
                        bonsai.set_char(lx, ly2, leaf_chars[idx]);
                    }
                }
            }
        }
    }
}

pub fn random_tree_style(rng: &mut ChaCha8Rng) -> TreeStyle {
    let styles = [TreeStyle::Pine, TreeStyle::Willow, TreeStyle::Oak, TreeStyle::Cherry, TreeStyle::Bamboo];
    let idx = rng.gen_range(0..styles.len());
    styles[idx]
}

pub fn random_pot_style(rng: &mut ChaCha8Rng) -> PotStyle {
    let styles = [PotStyle::Classic, PotStyle::Rectangular, PotStyle::Round, PotStyle::Decorated];
    let idx = rng.gen_range(0..styles.len());
    styles[idx]
}
