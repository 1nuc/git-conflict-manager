use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, List, ListState},
};

pub struct App<'a> {
    pub options: Vec<&'a str>,
    pub state: ListState,
    pub exit: bool,
    pub panel: String,
}

impl<'a> Default for App<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let options = Self::options();
        let state = ListState::default().with_offset(0);
        let exit = false;
        let panel="welcome to git conflict manager".to_string();
        Self {
            options,
            state,
            exit,
            panel,
        }
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
                if index >= self.options.len() - 1 {
                    self.panel=self.state.selected().unwrap().to_string();
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
                    self.panel=self.state.selected().unwrap().to_string();
                    self.options.len()
                } else {
                    index - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn leave(&mut self) {
        self.exit = true;
    }

    #[allow(unused_must_use)]
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_options(frame, frame.area()));
            self.handle_events();
        }
        Ok(())
    }

    fn handle_events(&mut self) {
        if let Some(event) = event::read().expect("no key pressed").as_key_press_event() {
            match event.code {
                KeyCode::Char('q') | KeyCode::Esc => self.leave(),
                KeyCode::Char('k') | KeyCode::Up => self.prev(),
                KeyCode::Char('j') | KeyCode::Down => self.next(),
                _ => (),
            }
        }
    }

    fn render_options(&mut self, frame: &mut Frame, area: ratatui::prelude::Rect) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);

        let horizontal =
            Layout::horizontal([Constraint::Percentage(40), Constraint::Percentage(60)]).spacing(1);

        let [top, down] = area.layout(&vertical);
        let [left, right] = area.layout(&horizontal);

        let title = Line::from(Span::from("Git Conflict Manager"));
        let instructions = Line::from(vec![
            " Scroll Down ".blue(),
            " <Left> or <j>".red(),
            " Scroll Up ".blue(),
            " <Up> or <k>".red(),
            " Exit ".blue(),
            " <Esc> or <q>".red(),
        ]);

        frame.render_widget(title.centered().bold().blue(), top);

        frame.render_stateful_widget(
            List::new(self.options.clone())
                .highlight_symbol(">> ")
                .highlight_style(Style::new().bold().on_green())
                .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
                .block(
                    Block::bordered()
                        .style(Style::new().blue().bold())
                        .title_bottom(instructions.centered())
                        .border_set(border::DOUBLE),
                ),
            left,
            &mut self.state,
        );
        frame.render_widget(Line::from(Span::raw(self.panel.clone())), right);
    }
}
