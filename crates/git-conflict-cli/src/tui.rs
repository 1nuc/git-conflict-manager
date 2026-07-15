use ratatui::{
    layout::{Constraint, Layout}, style::Style, symbols::border::Set, widgets::{Block, List, ListState, Paragraph, StatefulWidget, Widget}
};

pub struct App<'a> {
    pub options: Vec<&'a str>,
    pub state: ListState,
}

impl<'a> App<'a> {
    pub fn new() -> Self {
        let options = Self::options();
        let state = ListState::default().with_offset(0);
        Self { options, state }
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
}
impl<'a> StatefulWidget for &App<'a> {
    type State = ListState;
    fn render(
        self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        let title = "Git Conflict Manager";
        let block = Block::new()
            .title(title)
            .style(Style::new().bold().italic())
            .border_type(ratatui::widgets::BorderType::Double);
        let layout=Layout::default().constraints([Constraint::Fill(1),Constraint::Percentage(30), Constraint::Percentage(70)]);
        let [left, right]=area.layout(&layout);

        let list = List::new(self.options).highlight_symbol(">> ").block(block).style(Style::default().bold().white());
    }
}

