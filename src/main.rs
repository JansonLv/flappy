use bracket_lib::terminal::{main_loop, BError, BTermBuilder, GameState, BLUE};

fn main() -> BError {
    let ctx = BTermBuilder::simple80x50()
        .with_title("疯狂的小鸟")
        .build()?;
    main_loop(ctx, state::new())
}

struct state {
    mode: GameMode,
}

impl state {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
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
                bracket_lib::terminal::VirtualKeyCode::P => {}
                _ => {}
            }
        }
    }
}

impl GameState for state {
    fn tick(&mut self, ctx: &mut bracket_lib::terminal::BTerm) {
        match self.mode {
            GameMode::Menu => self.menu(ctx),
            GameMode::Play => todo!(),
            GameMode::End => todo!(),
        }
    }
}

enum GameMode {
    Menu,
    Play,
    End,
}
