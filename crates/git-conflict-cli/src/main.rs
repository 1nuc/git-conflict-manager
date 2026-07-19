use std::io;
use crate::tui::App;
mod tui;
mod desc;


fn main() -> io::Result<()>{
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let mut app=App::default();
    ratatui::run(|terminal| app.run(terminal))
}
