use std::io;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::{event::{self, KeyCode}},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Clear, List, ListState, Paragraph},
};

pub struct App<'a> {
    options: Vec<Span<'a>>,
    state: ListState,
    exit: bool,
    panel: String,
    bg_color: Color,
    pop_up: bool,
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
        let panel = "welcome to git conflict manager".to_string();
        let bg_color = Color::Rgb(0, 6, 61);
        Self {
            options,
            state,
            exit,
            panel,
            bg_color,
            pop_up: false,
        }
    }

    fn options() -> Vec<Span<'a>> {
        vec![
            "Keep Local Head Changes".white(),
            "Keep Foreign Branch Changes".white(),
            "Remove Markers and Keep Both Changes (Soon)".white(),
            "Merge Trees".white(),
            "Exit".white(),
        ]
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(index) => {
                if index >= self.options.len() - 1 {
                    0
                } else {
                    index + 1
                }
            }
            None => 0,
        };
        self.panel = i.to_string();
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
        self.panel = i.to_string();
        self.state.select(Some(i));
    }

    fn leave(&mut self) {
        self.exit = true;
    }


    fn update(&mut self) {
        self.pop_up= true;
    }

    #[allow(unused_must_use)]
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render(frame, frame.area()));
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
                KeyCode::Enter | KeyCode::Char('x') => self.update(),
                _ => (),
            }
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let [header, content, footer] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Fill(1),
            Constraint::Length(3),
        ])
        .spacing(1)
        .areas(area);

        let [left, right] = Layout::new(
            ratatui::layout::Direction::Horizontal,
            [Constraint::Percentage(40), Constraint::Percentage(60)],
        )
        .spacing(1)
        .areas(content);
        self.render_header_footer(frame, header, footer);
        self.render_main_content(frame, left, right);
        if self.pop_up {
            let opt_block = Block::bordered()
                .style(Style::new().bg(Color::Black).red())
                .title("Something");
            let adj_area=area.centered(Constraint::Percentage(60), Constraint::Percentage(20));
            frame.render_widget(Clear, adj_area);
            let options=Paragraph::new(Text::from(Line::from("Are you sure you want to continue")).centered()).block(opt_block);
            frame.render_widget(options, adj_area);
        }
    }

    fn render_header_footer(&mut self, frame: &mut Frame, header: Rect, footer: Rect) {
        let title = Line::from("Git Conflict Manager".white());
        let title_footer = Line::from("Controls".white());

        let instructions = Line::from(vec![
            " Scroll Down ".white(),
            " <Left> or <j>".red(),
            " Scroll Up ".white(),
            " <Up> or <k>".red(),
            " Exit ".white(),
            " <Esc> or <q>".red(),
            " Exec ".white(),
            " <Enter> or <x>".red(),
        ]);

        frame.render_widget(
            Paragraph::new(Text::from(
                Line::from("Here where you can resolve conflicts intuitivly")
                    .white()
                    .centered(),
            ))
            .block(
                Block::bordered()
                    .style(Style::new().bold().red().bg(self.bg_color))
                    .title(title.centered())
                    .bold(),
            ),
            header,
        );
        frame.render_widget(
            Paragraph::new(Text::from(instructions.centered())).block(
                Block::bordered()
                    .style(Style::new().bg(self.bg_color).bold().red())
                    .title(title_footer.centered())
                    .bold(),
            ),
            footer,
        );
    }
    fn render_main_content(&mut self, frame: &mut Frame, left: Rect, right: Rect) {
        frame.render_stateful_widget(
            List::new(self.options.clone())
                .highlight_symbol(">> ")
                .highlight_style(Style::new().bold().on_red().blue())
                .highlight_spacing(ratatui::widgets::HighlightSpacing::Always)
                .block(
                    Block::bordered()
                        .style(Style::new().red().bg(self.bg_color).bold())
                        .border_set(border::DOUBLE),
                ),
            left,
            &mut self.state,
        );
        let block = Block::bordered()
            .border_set(border::DOUBLE)
            .style(Style::new().red().bold().bg(self.bg_color));

        let paragraph = Paragraph::new(Text::from(self.panel.clone().white())).block(block);
        frame.render_widget(paragraph, right);
    }
}
