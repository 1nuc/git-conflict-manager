use std::{env, io};
use log::*;
use git_conflict::{GitOps, Initialize, git_src::Repo};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyCode},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span, Text},
    widgets::{Block, Clear, List, ListState, Paragraph, Wrap},
};

pub struct App<'a> {
    options: Vec<Span<'a>>,
    state: ListState,
    exit: bool,
    panel: String,
    bg_color: Color,
    pop_up: bool,
    exec_opt: ExecOption<'a>,
    args: Vec<String>
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
        let bg_color = Color::Rgb(14, 9, 26);
        let exec_opt=ExecOption::default();
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            println!(
                "{}",
                "You have to specify the names of the conflicted branches"
                    .italic()
                    .bold()
                    .red()
            );
            Self::show_example();
        }
        Self {
            options,
            state,
            exit,
            panel,
            bg_color,
            pop_up: false,
            exec_opt,
            args,
        }
    }

    fn show_example() {
        warn!(
            "{}",
            "Example: cargo r src_branch dest_branch"
                .italic()
                .bold()
                .yellow()
        );
        warn!(
            "{}",
            "src_branch is the branch is the branch you are currently at whcih is pointed by head"
                .italic()
                .bold()
                .yellow()
        );
        warn!(
            "{}",
            "to check for your source branch type git status"
                .italic()
                .bold()
                .yellow()
        );
        warn!(
            "{}",
            "dest_branch is the branch you are trying to merge"
                .italic()
                .bold()
                .yellow()
        );
        warn!(
            "{}",
            "rewrite the command with specifying the name of the branches"
                .italic()
                .bold()
                .yellow()
        );
    }

    fn options() -> Vec<Span<'a>> {
        vec![
            "Keep Local Head Changes".white(),
            "Keep Foreign Branch Changes".white(),
            "Remove Markers and Keep Both Changes".white(),
            "Merge Trees".white(),
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
        if self.pop_up{
            self.pop_up=false;
        }
        else{
            self.exit = true;
        }
    }


    fn update(&mut self) {
        self.pop_up= true;
    }

    fn exit_pop_up(&mut self) {
        if self.pop_up{
            self.pop_up=false;
        }
    }

    fn update_pop_up(&mut self) {
        if self.pop_up{
            if self.exec_opt.is_tree{
                todo!()
            }
            else{
                self.exec_opt.exec(args, None, None);
            }
        }
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
                KeyCode::Char('n') => self.exit_pop_up(),
                KeyCode::Char('y') => self.update_pop_up(),
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
            self.render_pop_up(frame, area);
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
                .highlight_style(Style::new().bold().on_white().black())
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

    fn render_pop_up(&mut self, frame: &mut Frame, area: Rect){
        let exec_option=self.exec_opt.run(self.panel.clone()).expect("Index is None");
        let opt_block = Block::bordered()
            .style(Style::new().bg(self.bg_color).red())
            .title_bottom(exec_option.controls.centered());
        let adj_area=area.centered(Constraint::Percentage(60), Constraint::Percentage(20));
        frame.render_widget(Clear, adj_area);
        let options=Paragraph::new(Text::from(exec_option.msg).centered().bold()).centered().wrap(Wrap{trim: true}).block(opt_block);
        frame.render_widget(options, adj_area);
    }
}

#[derive(Clone)]
enum ExecTypes{
    AcceptLocal,
    AcceptForeign,
    Combination,
    TreeMerge,
    Idle,
}

#[derive(Clone)]
struct ExecOption<'a>{
    msg: Line<'a>,
    controls: Line<'a>,
    is_tree: bool,
    res_method: ExecTypes,
}


impl <'a> Default for ExecOption<'a>{
    fn default() -> Self {
        Self{
            msg: Line::default(),
            controls: Line::default(),
            is_tree: false,
            res_method: ExecTypes::Idle,
        }
    }
}
impl <'a>ExecOption <'a>{
    fn run(&mut self, index: String) -> Option<Self>{
        match index.as_str(){
            "0" =>{
                self.res_method=ExecTypes::AcceptLocal;
                self.return_msg();
                Some(self.clone())
            },
            "1" =>{
                self.res_method=ExecTypes::AcceptForeign;
                self.return_msg();
                Some(self.clone())
            },
            "2" => {
                self.res_method=ExecTypes::Combination;
                self.return_msg();
                Some(self.clone())
            },
            "3" =>{
                self.res_method=ExecTypes::TreeMerge;
                self.tree_msg();
                Some(self.clone())
            },
            _ => None,
        }
    }
    
    fn return_msg(&mut self) -> Self{
        self.msg=Line::from("Are you sure you want to execute").white().centered();
        self.controls=Line::from(vec![
            "Yes ".white(),
            " <y> ".red(),
            "No ".white(),
            " <n>".red(),
        ]);
        self.clone()
    }

    fn tree_msg(&mut self)-> Self{
         self.msg=Line::from(vec![
            "Parenet Interference? ".white(),
            "For example: if the head branch latest commit is -add features x-".white(),
            "And the incoming branch commit is -fix feature x-".white(),
            "And the ancestor commit of branches is -ship feature x-".white(),
            "The new merge commit will combine the latest cleanest path (ancestor commit) to the new accepted changes".white(),
        ]);
        self.controls=Line::from(vec![
            "Yes".white(),
            "<y>".red(),
            "No".white(),
            "<n>".red(),
        ]);
        self.is_tree=true;
        self.clone()
    }

    fn exec(&self, args: Vec<String>, parent_interference: Option<bool>, version: Option<bool>){
        let git_control=Repo::init(args[0].clone(), args[1].clone());
        match self.res_method{
            ExecTypes::AcceptLocal => git_control.call_discarding(true),
            ExecTypes::AcceptForeign => git_control.call_discarding(false),
            ExecTypes::Combination => git_control.call_combinition(),
            ExecTypes::TreeMerge => git_control.call_tree_merge(Some(version).is_some(), Some(parent_interference).is_some()),
            _ =>(),
        }
    }
}

