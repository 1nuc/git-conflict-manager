//! Git conflict is CLI is a ratatui based tui that enables terminal user interactivity through
//! through manual control over the tui.
//! the tui module contains the essential functions required to simulate the actions performed by
//! the tool.
use std::io;
use crate::tui::App;
mod tui;
mod desc;


fn main() -> io::Result<()>{
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let mut app=App::default();
    ratatui::run(|terminal| app.run(terminal))
}
