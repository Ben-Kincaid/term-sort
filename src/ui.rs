use crate::app::{App, View};
use crate::sort;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{BarChart, Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub fn draw_header(f: &mut Frame<impl Backend>, chunk: Rect, title: &'static str) {
    let text = vec![Spans::from(vec![Span::raw(title)])];

    let block = Block::default();
    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::Red).bg(Color::Black))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, chunk);
}

pub fn draw_menu_list(
    f: &mut Frame<impl Backend>,
    chunk: Rect,
    items: &Vec<(&'static str, View)>,
    state: &mut ListState,
) {
    let items: Vec<ListItem> = items.iter().map(|(text, _)| ListItem::new(*text)).collect();

    let list = List::new(items)
        .block(Block::default())
        .style(Style::default().fg(Color::White))
        .highlight_style(
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::ITALIC),
        )
        .highlight_symbol(">>");

    f.render_stateful_widget(list, chunk, state);
}

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

    draw_header(f, chunks[0], "which-sort");

    let menu = app.states.menu.as_mut().unwrap();

    draw_menu_list(f, chunks[2], menu.list.items.as_ref(), &mut menu.list.state);
}

pub fn draw_sort(f: &mut Frame<impl Backend>, chunk: Rect, sort_iter: &mut impl sort::Sort) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Percentage(100)].as_ref())
        .split(chunk);

    let mut p_style = Style::default().fg(Color::Green);
    if sort_iter.is_active() {
        p_style = p_style.fg(Color::DarkGray);
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Gray));
    let paragraph = Paragraph::new("Enter = sort")
        .style(p_style)
        .block(block)
        .alignment(Alignment::Left);
    f.render_widget(paragraph, chunks[0]);

    let data: Vec<(&'static str, u64)> = sort_iter
        .items()
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let p = sort_iter.get_pointer();
            if sort_iter.is_active() == false
                || sort_iter.is_active() == true && (p.0 != i && p.1 != i)
            {
                ("", *x as u64)
            } else {
                ("⬆", *x as u64)
            }
        })
        .collect();

    let chart = BarChart::default()
        .block(Block::default().borders(Borders::ALL))
        .data(&data)
        .bar_width(1)
        .bar_style(Style::default().fg(Color::Yellow))
        .label_style(Style::default().fg(Color::Red));

    f.render_widget(chart, chunks[1]);
}

pub fn draw_single_sort(f: &mut Frame<impl Backend>, app: &mut App) {
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

    match app.current_view() {
        View::Bubble => {
            draw_header(f, chunks[0], "Bubble Sort");
            draw_sort(f, chunks[2], &mut app.states.bubble.as_mut().unwrap().sort);
        }
        _ => (),
    }
}
