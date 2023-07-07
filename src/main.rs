use bracket_lib::{
    random::RandomNumberGenerator,
    terminal::{main_loop, to_cp437, BError, BTermBuilder, GameState, BLACK, BLUE, GREEN, YELLOW},
};

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("疯狂的小鸟")
        .build()?;
    main_loop(ctx, state::new())
}

struct state {
    mode: GameMode,
    player: Player,
    obstacle: Obstacle,
    frame_time: f32,
    score: i32,
}

struct Player {
    x: i32,
    y: i32,
    velocity: f32,
}

impl Player {
    fn new() -> Self {
        Self {
            x: 15,
            y: 25,
            velocity: 1.0,
        }
    }
    fn render(&self, ctx: &mut bracket_lib::terminal::BTerm) {
        ctx.set(self.x, self.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn move1(&mut self) {
        self.y += self.velocity as i32;
    }

    fn is_dead(&self) -> bool {
        self.y > 50
    }

    fn flay(&mut self) {
        self.y -= 6;
    }
}

impl state {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
            player: Player::new(),
            frame_time: 0.0,
            obstacle: Obstacle::new(30, 0),
            score: 0,
        }
    }
    pub fn menu(&mut self, ctx: &mut bracket_lib::terminal::BTerm) {
        ctx.cls();
        ctx.print_centered(15, "Welcome Fapply");
        ctx.print_centered(20, "(P) Play Game");
        ctx.print_centered(25, "(Q) Quit Game");
        if let Some(k) = ctx.key {
            match k {
                bracket_lib::terminal::VirtualKeyCode::Q => ctx.quit(),
                bracket_lib::terminal::VirtualKeyCode::P => self.replay(),
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut bracket_lib::terminal::BTerm) {
        ctx.cls();
        ctx.print_centered(15, "You are dead!");
        ctx.print_centered(20, "(P) Replay Game");
        ctx.print_centered(25, "(Q) Quit Game");
        if let Some(k) = ctx.key {
            match k {
                bracket_lib::terminal::VirtualKeyCode::Q => ctx.quit(),
                bracket_lib::terminal::VirtualKeyCode::P => self.replay(),
                _ => {}
            }
        }
    }

    fn replay(&mut self) {
        self.score = 0;
        self.mode = GameMode::Play;
        self.player = Player::new();
        self.obstacle = Obstacle::new(30, self.score);
    }

    fn play(&mut self, ctx: &mut bracket_lib::terminal::BTerm) {
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time < 60.0 {
            return;
        }
        ctx.cls();
        self.player.move1();
        self.player.render(ctx);
        self.obstacle.render(ctx);
        self.frame_time = 0.0;

        // flay
        if let Some(bracket_lib::terminal::VirtualKeyCode::Space) = ctx.key {
            self.player.flay()
        }
        if self.player.y < 0 {
            self.player.y = 0
        }
        //  游戏结束
        if self.player.is_dead() {
            self.mode = GameMode::Dead;
        }
    }
}

impl GameState for state {
    fn tick(&mut self, ctx: &mut bracket_lib::terminal::BTerm) {
        match self.mode {
            GameMode::Menu => self.menu(ctx),
            GameMode::Play => self.play(ctx),
            GameMode::Dead => self.dead(ctx),
        }
    }
}

enum GameMode {
    Menu,
    Play,
    Dead,
}

/// Game obstacle
pub struct Obstacle {
    // 横坐标地址
    pub x: i32,
    gap_center_y: i32, // 空心中间点
    gap_size: i32,     // 空心间距
}

impl Obstacle {
    // score 难度值
    pub fn new(x: i32, score: i32) -> Self {
        let mut rand = RandomNumberGenerator::new();
        let gap_size = i32::max(10, 30 - score);
        Self {
            x,
            gap_center_y: rand.range(gap_size / 2, 50 - (gap_size / 2)),
            gap_size, // 难度越大，空心距离越小，最小10
        }
    }

    fn render(&mut self, ctx: &mut bracket_lib::terminal::BTerm) {
        self.x -= 1;
        // ctx.set(screen_x, y, GREEN, BLACK, to_cp437('#'));
        let end = self.gap_center_y - self.gap_size / 2;
        let start = self.gap_center_y + self.gap_size / 2;

        for y in 0..end {
            ctx.set(self.x, y, GREEN, BLACK, to_cp437('#'));
        }
        for y in start..50 {
            ctx.set(self.x, y, GREEN, BLACK, to_cp437('#'));
        }
    }
}
