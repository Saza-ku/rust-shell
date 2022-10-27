use tracing_subscriber::{self, fmt, prelude::*, EnvFilter};

use event::ShellState;

mod event;

fn main() {
    tracing_subscriber::registry()
         .with(fmt::layer())
         .with(EnvFilter::from_default_env())
         .init();

    ShellState::new().run();
}
