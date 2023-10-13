use piston_window::*;
use rand::Rng;
use std::time::Duration;
// use color;

// const LINE_HEIGHT: f64 = 40f64;

// fn writeln_text<G: Graphics<Texture=gfx_texture::Texture<gfx_device_gl::Resources>>>(
//     text: &str,
//     transform: graphics::context::Context, 
//     context: &piston_window::Context,
//     cache: &mut piston_window::Glyphs, 
//     graphics: &mut G
// ) -> graphics::context::Context {
//     let color = [0.0, 0.0, 0.0, 1.0];
//     let mut result: graphics::context::Context = transform;
//     Text::new_color(color, TEXT_FONT_SIZE)
//         .draw(text, cache, &context.draw_state, result.transform, graphics).unwrap();
//     result = result.trans(0f64, LINE_HEIGHT);
//     result
// } 

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cell {
    BLACK,
    WHITE,
    EMPTY
}

const D: [(isize, isize); 4] = [(0, 1), (1, 0), (1, 1), (-1, 1)];

pub struct Othello {
    board: [[Cell; 8]; 8],
    checked: Vec<Vec<Vec<(usize, isize, isize, bool)>>>,
    pub turn: Cell,
    pub player: Cell,
    pub select: (usize, usize),
    pre: (usize, usize),
    is_skipped: bool,
    cell_white: G2dTexture,
    cell_blue: G2dTexture,
    cell_yellow: G2dTexture,
    piece_black: G2dTexture,
    piece_white: G2dTexture,
    cnt: [isize; 2]
}

impl Othello {
    pub fn new(player: Cell, (cell_white, cell_blue, cell_yellow, piece_black, piece_white): (G2dTexture, G2dTexture, G2dTexture, G2dTexture, G2dTexture)) -> Self {
        let mut board = [[Cell::EMPTY; 8]; 8];
        board[3][3] = Cell::BLACK; board[4][4] = Cell::BLACK;
        board[3][4] = Cell::WHITE; board[4][3] = Cell::WHITE;
        return Self {
            board,
            checked: vec![vec![Vec::new(); 8]; 8],
            turn: Cell::BLACK,
            player,
            select: (9, 9),
            pre: (9, 9),
            is_skipped: false,
            cell_white,
            cell_blue,
            cell_yellow,
            piece_black,
            piece_white,
            cnt: [2; 2]
        };
    }

    pub fn draw(&mut self, c: Context, g: &mut G2d) {
        self.check();
        // let mut text = writeln_text(
        //     &format!("{}の\nターンです", if self.turn == self.player {"あなた"} else {"コンピュータ"}),
        // )
        // let mut text = format!("{}の\nターンです", if self.turn == self.player {"あなた"} else {"コンピュータ"});
        // if self.cnt == 0 {
        // }
        // if self.is_skipped {
        //     text = format!("{}は\nスキップされました", if self.turn == self.player {"あなた"} else {"コンピュータ"});
        // }

        // text::Text::new_color([0.0, 0.0, 0.0, 1.0], 14)
        //     .draw(&text, cache, draw_state, transform, g);

        // let a = Text::new(16);
        // a.draw_pos(&text, [30.0, 200.0], &mut c, draw_state, transform, g);

        

        let start_pos = (120.0, 40.0);
        for y in 0..8 {
            for x in 0..8 {
                if (y, x) == self.pre {
                    image(
                        &self.cell_yellow,
                        c
                            .transform
                            .trans(
                                start_pos.0 + 50.0 * x as f64,
                                start_pos.1 + 50.0 * y as f64
                            ),
                        g
                    );
                } else if self.checked[y][x].is_empty() {
                    image(
                        &self.cell_white,
                        c
                            .transform
                            .trans(
                                start_pos.0 + 50.0 * x as f64,
                                start_pos.1 + 50.0 * y as f64
                            ),
                        g
                    );
                } else {
                    image(
                        &self.cell_blue,
                        c
                            .transform
                            .trans(
                                start_pos.0 + 50.0 * x as f64,
                                start_pos.1 + 50.0 * y as f64
                            ),
                        g
                    );
                }
                match self.board[y][x] {
                    Cell::BLACK => {
                        image(
                            &self.piece_black,
                            c
                                .transform
                                .trans(
                                    start_pos.0 + 50.0 * x as f64 + 4.0,
                                    start_pos.1 + 50.0 * y as f64 + 4.0
                                ),
                            g
                        );
                    },
                    Cell::WHITE => {
                        image(
                            &self.piece_white,
                            c
                                .transform
                                .trans(
                                    start_pos.0 + 50.0 * x as f64 + 4.0,
                                    start_pos.1 + 50.0 * y as f64 + 4.0
                                ),
                            g
                        );
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn check(&mut self) {
        self.checked = vec![vec![Vec::new(); 8]; 8];
        self.is_skipped = false;
        for y in 0..8 {
            for x in 0..8 {
                if self.board[y][x] == Cell::EMPTY || self.board[y][x] == self.turn {
                    for (ddy, ddx) in D {
                        let mut dy = ddy; let mut dx = ddx;
                        let mut cnt = 1;
                        let mut flag = true;
                        while self.in_board((y, x), (dy, dx)) {
                            let ny = (y as isize + dy) as usize;
                            let nx = (x as isize + dx) as usize;
                            match self.board[ny][nx] {
                                Cell::EMPTY => {
                                    if self.board[y][x] == Cell::EMPTY {
                                        flag = false;
                                    }
                                    break;
                                },
                                _ => {
                                    if self.board[ny][nx] == self.turn {
                                        break;
                                    } else {
                                        cnt += 1;
                                        dy += ddy; dx += ddx;
                                    }
                                }
                            }
                        }
                        if flag && cnt > 1 && self.in_board((y, x), (dy, dx)) {
                            self.is_skipped = true;
                            let ny = (y as isize + dy) as usize;
                            let nx = (x as isize + dx) as usize;
                            if self.board[y][x] != self.board[ny][nx] {
                                if self.board[y][x] == Cell::EMPTY {
                                    self.checked[y][x].push((cnt, ddy, ddx, true));
                                } else {
                                    self.checked[ny][nx].push((cnt, ddy, ddx, false));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn turn_change(&mut self) {
        match self.turn {
            Cell::BLACK => self.turn = Cell::WHITE,
            Cell::WHITE => self.turn = Cell::BLACK,
            _ => {}
        };
    }

    pub fn flip(&mut self) {
        if !self.in_board(self.select, (0, 0)) { return; }
        let (y, x) = self.select;
        if self.checked[y][x].is_empty() { return; }
        self.cnt[if self.turn == Cell::BLACK {0} else {1}] += 1;
        self.board[y][x] = self.turn;
        for (cnt, ddy, ddx, rev) in self.checked[y][x].clone() {
            for i in 1..cnt {
                if rev {
                    self.board[(y as isize + (ddy * i as isize)) as usize][(x as isize + (ddx * i as isize)) as usize] = self.turn;
                } else {
                    self.board[(y as isize - (ddy * i as isize)) as usize][(x as isize - (ddx * i as isize)) as usize] = self.turn;
                }
                self.cnt[if self.turn == Cell::BLACK {0} else {1}] += 1;
                self.cnt[if self.turn == Cell::BLACK {1} else {0}] -= 1;
            }
        }
        self.turn_change();
        self.pre = self.select;
        println!("(黒, 白): {:?}", self.cnt);
    }

    pub fn put_greedy(&mut self) {
        let wait_time = rand::thread_rng().gen_range(4..8);
        for _ in 0..wait_time {
            std::thread::sleep(Duration::from_millis(500));
        }
        let mut max = 0;
        let mut cand = Vec::new();
        for y in 0..8 {
            for x in 0..8 {
                let mut sum = 0;
                for (cnt, _, _, _) in self.checked[y][x].clone() {
                    sum += cnt;
                }
                if sum > max {
                    max = sum;
                    cand = Vec::new();
                    cand.push((y, x));
                } else if sum == max {
                    cand.push((y, x));
                }
            }
        }
        let n = cand.len();
        let i = rand::thread_rng().gen_range(0..n);
        self.select = cand[i];
        self.flip();
    }

    fn in_board(&self, pos: (usize, usize), dir: (isize, isize)) -> bool {
        let y = pos.0 as isize + dir.0;
        let x = pos.1 as isize + dir.1;
        return 0 <= y && y < 8 && 0 <= x && x < 8;
    }
}