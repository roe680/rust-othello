use std::fmt::{Debug, Display};

#[derive(Copy, Clone, Default, PartialEq, Debug)]
pub enum Pattern {
    Black,
    White,
    #[default]
    None,
}

impl Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Pattern::Black => {
                write!(f, "Black")
            }
            Pattern::White => {
                write!(f, "White")
            }
            Pattern::None => {
                write!(f, "None")
            }
        }
    }
}

fn main() {
    let mut osero = Osero::default();
    let mut with = Pattern::Black;
    let level;
    loop {
        println!("レベルを選んでください");
        let mut level_str = String::new();
        std::io::stdin().read_line(&mut level_str).ok();
        if level_str.trim().is_empty() {
            println!("{}は無効です。", level_str);
            println!("例: 1");
        } else {
            if let Ok(num) = level_str.trim().parse::<usize>() {
                level = num;
                break;
            } else {
                println!("{}は無効です。", level_str);
                println!("例: 1");
            }
        }
    }

    println!("オセロ対決スタートです！");
    println!("黒（Black）が先手です。\n");

    loop {
        if osero.is_finished() {
            break;
        }

        // AIターン（白）
        if osero.is_moveable_with(with) {
            if with == Pattern::White {
                if let Some(pos) = osero.best_move(with, level) {
                    println!("\n-----------------------------");
                    println!("😼『うにゃっ、そこがよさそうだにゃ……』");
                    println!("AI（白）は {:?} に置いたにゃ〜", pos);
                    osero.put(with, pos);

                    // ここで盤面を表示！
                    println!("{}", osero.express());
                    let (black, white, none) = osero.many();
                    println!("黒 X: {}　白 O: {}　空白: {}", black, white, none);

                    with = with.fliped();
                } else {
                    println!("😿『置けないにゃ…パスするにゃ』");
                    with = with.fliped();
                }
                continue;
            }

            // プレイヤーターン（黒）
            if with == Pattern::Black {
                println!("\n-----------------------------");
                println!("{}", osero.express());
                let (black, white, none) = osero.many();
                println!("黒 X: {}　白 O: {}　空白: {}", black, white, none);
                println!(
                    "{} のターンです。座標を2つ半角スペースで入力してください（例: `3 2`）",
                    with
                );

                let mut pos = String::new();
                std::io::stdin().read_line(&mut pos).ok();

                if !pos.trim().is_empty() {
                    let vec = pos
                        .split_whitespace()
                        .map(|a| a.to_string())
                        .collect::<Vec<String>>();

                    let mut points: Vec<usize> =
                        vec.iter().filter_map(|s| s.parse::<usize>().ok()).collect();

                    if points.len() == 2 {
                        let at = (points[0], points[1]);
                        if osero.is_putabele(with, at) {
                            osero.put(with, at);
                            with = with.fliped();
                        } else {
                            println!("その場所には置けません。もう一度入力してください。");
                        }
                    } else {
                        println!("⚠️ 座標は2つ必要です。例: `3 2` のように入力してください。");
                    }
                } else {
                    println!("⚠️ 入力が空です。もう一度入力してください。");
                }
            }
        } else {
            println!("置けないので強制的にパスします。");
            with = with.fliped();
            continue;
        }
    }

    println!("\n=============================");
    println!("ゲーム終了です。結果を発表します！");

    let (black, white, _) = osero.many();
    println!("● Black: {}, ○ White: {}", black, white);

    if let Some(winner) = osero.which_win() {
        match winner {
            Pattern::Black => println!("🎉 黒（Black）の勝ちです！おめでとうございます！"),
            Pattern::White => println!("😼 白（AI）の勝ちだにゃ〜！やったにゃ〜！"),
            Pattern::None => println!(":thinking:...error"),
        }
    } else {
        println!("🤝 引き分けです。再挑戦してみてください！");
    }
    println!("=============================\n");
}

fn index_to_str(i: i32) -> String {
    match i {
        0 => "０",
        1 => "１",
        2 => "２",
        3 => "３",
        4 => "４",
        5 => "５",
        6 => "６",
        7 => "７",
        _ => "",
    }
    .to_string()
}

#[derive(Clone)]
pub struct Osero(pub [Pattern; 64]);

impl Default for Osero {
    fn default() -> Self {
        let mut osero = Osero([Pattern::None; 64]);
        osero.set(Pattern::Black, (3, 3));
        osero.set(Pattern::White, (4, 3));
        osero.set(Pattern::White, (3, 4));
        osero.set(Pattern::Black, (4, 4));
        osero
    }
}

impl Pattern {
    fn fliped(&self) -> Pattern {
        match self {
            Pattern::White => Pattern::Black,
            Pattern::Black => Pattern::White,
            Pattern::None => Pattern::None,
        }
    }
}

impl Osero {
    pub fn which_win(&self) -> Option<Pattern> {
        let mut black: usize = 0;
        let mut white: usize = 0;
        for point in self.0.iter() {
            match point {
                Pattern::Black => black += 1,
                Pattern::None => {}
                Pattern::White => white += 1,
            }
        }
        if black > white {
            Some(Pattern::Black)
        } else if black < white {
            Some(Pattern::White)
        } else {
            None
        }
    }

    pub fn is_finished(&self) -> bool {
        let no_empty_cells = !self.0.iter().any(|p| *p == Pattern::None);
        let no_moves_black = !self.is_moveable_with(Pattern::Black);
        let no_moves_white = !self.is_moveable_with(Pattern::White);

        no_empty_cells || (no_moves_black && no_moves_white)
    }

    pub fn many(&self) -> (usize, usize, usize) {
        let mut black: usize = 0;
        let mut white: usize = 0;
        let mut none: usize = 0;
        for point in self.0.iter() {
            match point {
                Pattern::Black => black += 1,
                Pattern::None => none += 1,
                Pattern::White => white += 1,
            }
        }
        (black, white, none)
    }

    pub fn express(&self) -> String {
        let mut result = String::new();
        let mut y = 0;

        // 横軸ラベル（0〜7）
        result.push_str("  0 1 2 3 4 5 6 7\n");

        for (i, point) in self.0.iter().enumerate() {
            if i % 8 == 0 {
                result.push_str(&format!("{} ", y)); // 縦軸ラベル（0〜7）
                y += 1;
            }

            match point {
                Pattern::None => result.push_str(". "),
                Pattern::Black => result.push_str("X "),
                Pattern::White => result.push_str("O "),
            }

            if i % 8 == 7 {
                result.push('\n');
            }
        }

        result
    }

    pub fn is_putabele(&self, with: Pattern, at: (usize, usize)) -> bool {
        at.0 < 8
            && at.1 < 8
            && self.get(at) == Some(Pattern::None)
            && with != Pattern::None
            && (self.is_lines_changeable(with, at, Self::get_horizontal_line_parts)
                || self.is_lines_changeable(with, at, Self::get_vertical_line_parts)
                || self.is_lines_changeable(with, at, Self::get_downer_right_line_parts)
                || self.is_lines_changeable(with, at, Self::get_upper_right_line_parts))
    }

    pub fn put(&mut self, with: Pattern, at: (usize, usize)) {
        if !self.is_putabele(with, at) {
            return;
        }

        // まず置く
        self.set(with, at);

        // 4方向それぞれでひっくり返し処理を呼ぶ
        self.process(
            with,
            at,
            Self::get_horizontal_line_parts,
            Self::set_horizontal_line,
        );
        self.process(
            with,
            at,
            Self::get_vertical_line_parts,
            Self::set_vertical_line,
        );
        self.process(
            with,
            at,
            Self::get_upper_right_line_parts,
            Self::set_upper_right_line,
        );
        self.process(
            with,
            at,
            Self::get_downer_right_line_parts,
            Self::set_downer_right_line,
        );
    }

    fn is_lines_changeable(
        &self,
        my: Pattern,
        at: (usize, usize),
        get_line: fn(&Osero, (usize, usize)) -> (Vec<Pattern>, Vec<Pattern>),
    ) -> bool {
        let (before, after) = get_line(self, at);
        is_changeable(before, my) || is_changeable(after, my)
    }

    fn process(
        &mut self,
        my: Pattern,
        at: (usize, usize),
        get_line: fn(&Osero, (usize, usize)) -> (Vec<Pattern>, Vec<Pattern>),
        set_line: fn(&mut Osero, Vec<Pattern>, (usize, usize)),
    ) {
        let (mut before, mut after) = get_line(self, at);
        before = change(before, my);
        after = change(after, my);
        //beforeとafterは置いた位置から近い順なので逆にする必要あり
        before.reverse();
        set_line(self, add(before, my, after), at);
    }

    fn get_horizontal_line_parts(&self, at: (usize, usize)) -> (Vec<Pattern>, Vec<Pattern>) {
        let mut left: Vec<Pattern> = Vec::new();
        let mut right: Vec<Pattern> = Vec::new();
        for i in 0..8 {
            if i < at.0 {
                if let Some(p) = self.get((i, at.1)) {
                    left.push(p);
                }
            } else if i > at.0 {
                if let Some(p) = self.get((i, at.1)) {
                    right.push(p);
                }
            }
        }
        left.reverse();
        (left, right)
    }

    fn get_vertical_line_parts(&self, at: (usize, usize)) -> (Vec<Pattern>, Vec<Pattern>) {
        let mut top: Vec<Pattern> = Vec::new();
        let mut bottom: Vec<Pattern> = Vec::new();
        for i in 0..8 {
            if i < at.1 {
                if let Some(p) = self.get((at.0, i)) {
                    top.push(p);
                }
            } else if i > at.1 {
                if let Some(p) = self.get((at.0, i)) {
                    bottom.push(p);
                }
            }
        }
        top.reverse();
        (top, bottom)
    }

    fn get_upper_right_line_parts(&self, at: (usize, usize)) -> (Vec<Pattern>, Vec<Pattern>) {
        let a = at.1 as isize - at.0 as isize;
        let mut left_down: Vec<Pattern> = Vec::new();
        let mut right_up: Vec<Pattern> = Vec::new();
        for i in 0..8 {
            let x = i;
            let y = i as isize + a;
            if y < 0 || y >= 8 {
                continue;
            }
            if x < at.0 {
                if let Some(p) = self.get((x, y as usize)) {
                    left_down.push(p);
                }
            } else if x > at.0 {
                if let Some(p) = self.get((x, y as usize)) {
                    right_up.push(p);
                }
            }
        }
        left_down.reverse();
        (left_down, right_up)
    }

    fn get_downer_right_line_parts(&self, at: (usize, usize)) -> (Vec<Pattern>, Vec<Pattern>) {
        let a = at.1 as isize + at.0 as isize;
        let mut left_up: Vec<Pattern> = Vec::new();
        let mut right_down: Vec<Pattern> = Vec::new();
        for i in 0..8 {
            let x = i;
            let y = a - i as isize;
            if y < 0 || y >= 8 {
                continue;
            }
            if x < at.0 {
                if let Some(p) = self.get((x, y as usize)) {
                    left_up.push(p);
                }
            } else if x > at.0 {
                if let Some(p) = self.get((x, y as usize)) {
                    right_down.push(p);
                }
            }
        }
        left_up.reverse();
        (left_up, right_down)
    }

    fn get_mut(&mut self, at: (usize, usize)) -> Option<&mut Pattern> {
        if at.0 < 8 && at.1 < 8 {
            self.0.get_mut(at.1 * 8 + at.0)
        } else {
            None
        }
    }

    fn get(&self, at: (usize, usize)) -> Option<Pattern> {
        if at.0 < 8 && at.1 < 8 {
            self.0.get(at.1 * 8 + at.0).copied()
        } else {
            None
        }
    }

    fn set(&mut self, with: Pattern, at: (usize, usize)) -> bool {
        if at.0 < 8 && at.1 < 8 {
            if let Some(point) = self.get_mut(at) {
                *point = with;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn set_upper_right_line(&mut self, line: Vec<Pattern>, at: (usize, usize)) {
        let a = at.1 as isize - at.0 as isize;
        let mut i = 0;

        for x in 0..8 {
            let y = x as isize + a;
            if y < 0 || y >= 8 {
                continue;
            }

            if i >= line.len() {
                break;
            }

            self.set(line[i], (x, y as usize));
            i += 1;
        }
    }

    fn set_downer_right_line(&mut self, line: Vec<Pattern>, at: (usize, usize)) {
        let a = at.1 as isize + at.0 as isize;
        let mut i = 0;

        for x in 0..8 {
            let y = a - x as isize;
            if y < 0 || y >= 8 {
                continue;
            }

            if i >= line.len() {
                break;
            }

            self.set(line[i], (x, y as usize));
            i += 1;
        }
    }

    fn set_horizontal_line(&mut self, line: Vec<Pattern>, at: (usize, usize)) {
        if at.0 < 8 && at.1 < 8 {
            let mut i = 0;
            for point in line {
                self.set(point, (i, at.1));
                i += 1;
            }
        }
    }

    fn set_vertical_line(&mut self, line: Vec<Pattern>, at: (usize, usize)) {
        if at.0 < 8 && at.1 < 8 {
            let mut i = 0;
            for point in line {
                self.set(point, (at.0, i));
                i += 1;
            }
        }
    }

    pub fn is_moveable_with(&self, with: Pattern) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                if self.is_putabele(with, (x, y)) {
                    return true;
                }
            }
        }
        false
    }

    pub fn get_moveable_points(&self, with: Pattern) -> Vec<(usize, usize)> {
        let mut result = vec![];
        for y in 0..8 {
            for x in 0..8 {
                if self.is_putabele(with, (x, y)) {
                    result.push((x, y));
                }
            }
        }
        result
    }
}

fn change(line: Vec<Pattern>, with: Pattern) -> Vec<Pattern> {
    if with == Pattern::None {
        return line;
    }
    let mut result: Vec<Pattern> = vec![];
    for point in line.iter() {
        if *point == Pattern::None {
            return line;
        }
        if *point == with {
            for i in result.len()..line.len() {
                if let Some(push) = line.get(i).copied() {
                    result.push(push);
                }
            }
            return result;
        }
        result.push(point.fliped());
    }
    line
}

fn is_changeable(line: Vec<Pattern>, with: Pattern) -> bool {
    if with == Pattern::None {
        return false;
    }

    let mut has_opponent = false;

    for point in line.iter() {
        if *point == Pattern::None {
            return false; // 空きが出たら無効
        }
        if *point == with {
            return has_opponent; // 相手の石を挟んでたら true
        }
        has_opponent = true; //ちゃんと相手のこま、踏む必要があるので、相手の石を挟んでいたらtrue
    }

    false
}

fn add(mut before: Vec<Pattern>, my: Pattern, after: Vec<Pattern>) -> Vec<Pattern> {
    before.push(my);
    before.extend(after);
    before
}

//使わない可能性あり
fn to_array(vec: Vec<Pattern>) -> Option<[Pattern; 8]> {
    vec.try_into().ok()
}

//AI
impl Osero {
    pub fn count_all_flips(&self, with: Pattern, at: (usize, usize)) -> usize {
        if self.get(at) != Some(Pattern::None) {
            return 0; // すでに石があったらだめ
        }

        let dirs = [
            Self::get_horizontal_line_parts,
            Self::get_vertical_line_parts,
            Self::get_upper_right_line_parts,
            Self::get_downer_right_line_parts,
        ];

        let mut total = 0;
        for dir in dirs {
            let (before, after) = dir(self, at);
            total += count_flips(before, with);
            total += count_flips(after, with);
        }

        total
    }
}

use rand::seq::IndexedRandom;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
const POSITION_SCORE: [i32; 64] = [
    100, -20, 10, 5, 5, 10, -20, 100, -20, -50, -2, -2, -2, -2, -50, -20, 10, -2, 5, 1, 1, 5, -2,
    10, 5, -2, 1, 0, 0, 1, -2, 5, 5, -2, 1, 0, 0, 1, -2, 5, 10, -2, 5, 1, 1, 5, -2, 10, -20, -50,
    -2, -2, -2, -2, -50, -20, 100, -20, 10, 5, 5, 10, -20, 100,
];

impl Osero {
    pub fn best_move(&self, with: Pattern, level: usize) -> Option<(usize, usize)> {
        if level == 1 {
            self.level1(with)
        } else {
            self.minimax(level - 1, with)
        }
    }

    pub fn level1(&self, with: Pattern) -> Option<(usize, usize)> {
        let mut best_score = usize::MIN;
        let mut best_moves = vec![];

        for y in 0..8 {
            for x in 0..8 {
                let flips = self.count_all_flips(with, (x, y));
                if flips == 0 {
                    continue;
                }
                let total_score = flips;

                if total_score > best_score {
                    best_score = total_score;
                    best_moves = vec![(x, y)];
                } else if total_score == best_score {
                    best_moves.push((x, y));
                }
            }
        }

        let mut rng = rand::rng();
        best_moves.choose(&mut rng).copied()
    }

    fn get_total_score(&self, with: Pattern, at: (usize, usize), depth: usize) -> i32 {
        if !self.is_putabele(with, at) {
            return 0;
        }
        let mut osero = self.clone();

        let index = at.1 * 8 + at.0;
        let position_score = POSITION_SCORE[index];

        let score = self.count_all_flips(with, at) as i32 * 10 + position_score;
        osero.put(with, at);
        let points = osero.get_moveable_points(with.fliped());
        let mut depth_max_score: i32 = i32::MIN;
        if depth > 0 {
            if points.is_empty() {
                depth_max_score = 0;
            } else {
                for at in points.clone() {
                    let score = osero.get_total_score(with.fliped(), at, depth - 1);
                    if score > depth_max_score {
                        depth_max_score = score;
                    }
                }
            }
            score - depth_max_score
        } else {
            score
        }
    }

    fn minimax(&self, depth: usize, with: Pattern) -> Option<(usize, usize)> {
        let mut osero = self.clone();
        let mut best_score = i32::MIN;
        let mut best_moves = vec![];

        let points = osero.get_moveable_points(with);
        let scores = points
            .par_iter()
            .map(|at| {
                let score = osero.get_total_score(with, *at, depth);
                score
            })
            .collect::<Vec<i32>>();

        for (i, score) in scores.iter().enumerate() {
            if *score > best_score {
                best_score = *score;
                best_moves = vec![points[i]];
            } else if *score == best_score {
                best_moves.push(points[i]);
            }
        }
        let mut rng = rand::rng();
        let best_move = best_moves.choose(&mut rng).copied();
        best_move
    }
}

fn count_flips(line: Vec<Pattern>, with: Pattern) -> usize {
    if with == Pattern::None {
        return 0;
    }
    let mut count = 0;
    for point in line.iter() {
        if *point == Pattern::None {
            return 0;
        }
        if *point == with {
            return count;
        }
        count += 1;
    }
    0
}
