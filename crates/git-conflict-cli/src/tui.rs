use std::{io};

use ratatui::{
    DefaultTerminal, Frame, crossterm::{event::{self, KeyCode}}, layout::{Constraint, Layout}, style::Style, widgets::{Block, List, ListState,}
};

pub struct App<'a> {
    pub options: Vec<&'a str>,
    pub state: ListState,
    pub exit: bool,
}
impl <'a>Default for App<'a>{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let options = Self::options();
        let state = ListState::default().with_offset(0);
        let exit=false;
        Self { options, state, exit }
    }
    fn options() -> Vec<&'a str> {
        vec![
            "Keep Local Head Changes",
            "Keep Foreign Branch Changes",
            "Remove Markers and Keep Both Changes (Soon)",
            "Merge Trees",
            "Exit",
        ]
    }
    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(index) => {
                if index > self.options.len() {
                    0
                } else {
                    index + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn prev(&mut self) {
        let i = match self.state.selected() {
            Some(index) => {
                if index == 0 {
                    self.options.len()
                } else {
                    index - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    fn leave(&mut self){
        self.exit=true;
    }

    #[allow(unused_must_use)]
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()>{
        while !self.exit{
            terminal.draw(|frame|
                self.render_options(frame, frame.area())
            );
            self.handle_events();
        }
        Ok(())
    }

    fn handle_events(&mut self){
        if let Some(event) =event::read().expect("no key pressed").as_key_press_event(){
            match event.code{
                KeyCode::Char('q') | KeyCode::Esc => self.leave(),
                KeyCode::Char('k') | KeyCode::Up => self.next(),
                KeyCode::Char('j') | KeyCode::Down => self.prev(),
                _ => ()
            }
        }
    }

    fn render_options(
        &mut self,
        frame: &mut Frame,
        area: ratatui::prelude::Rect,
    ) {
        let title = "Git Conflict Manager";
        let block = Block::new()
            .title(title)
            .style(Style::new().bold().italic())
            .border_type(ratatui::widgets::BorderType::Double);
        let layout = Layout::default().constraints([
            Constraint::Fill(1),
            Constraint::Percentage(30),
            Constraint::Percentage(70),
        ]);
        let [left, right] = area.layout(&layout);

        frame.render_stateful_widget(
            List::new(self.options.clone())
                .highlight_symbol(">> ")
                .block(block)
                .style(Style::default().bold().white()),
            left,
            &mut self.state,
        );
    }
}

