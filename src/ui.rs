use crate::app::App;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn draw_menu(f: &mut Frame<impl Backend>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(f.size());

    let text = vec![Spans::from(vec![Span::raw("which-sort")])];

    let block = Block::default();
    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::Red).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, chunks[0]);

    let items: Vec<ListItem> = app
        .states
        .menu
        .as_ref()
        .unwrap()
        .list
        .items
        .iter()
        .map(|(item, _)| item)
        .cloned()
        .collect();

    let list = List::new(items)
        .block(Block::default())
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">>");

    f.render_stateful_widget(
        list,
        chunks[2],
        &mut app.states.menu.as_mut().unwrap().list.state,
    );
}

pub fn draw_single_sort(f: &mut Frame<impl Backend>, _: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Percentage(100),
            ]
            .as_ref(),
        )
        .split(f.size());

    let block = Block::default();
    f.render_widget(block, chunks[0]);
}
