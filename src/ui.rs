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

pub fn draw_header(
    f: &mut Frame<impl Backend>,
    chunk: Rect,
    title: String,
    sorting: bool,
    complete: bool,
) {
    let text = vec![Spans::from(vec![Span::raw(title)])];

    let p_style = if sorting {
        Style::default().bg(Color::Yellow).fg(Color::Black)
    } else if complete {
        Style::default().bg(Color::Green).fg(Color::Black)
    } else {
        Style::default().bg(Color::Black).fg(Color::White)
    };

    let block = Block::default();
    let paragraph = Paragraph::new(text)
        .block(block)
        .style(p_style)
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

pub fn draw_menu(f: &mut Frame<impl Backend>, app: &mut App, chunk: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Percentage(90),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(chunk);

    draw_header(f, chunks[0], "which-sort".to_string(), false, false);

    let menu = app.states.menu.as_mut().unwrap();

    draw_menu_list(f, chunks[2], menu.list.items.as_ref(), &mut menu.list.state);
    draw_menu_footer(f, chunks[3]);
}

pub fn draw_sort_hud(
    f: &mut Frame<impl Backend>,
    chunk: Rect,
    sort_name: &str,
    current_step: &str,
    sorting: bool,
    complete: bool,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .margin(0)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunk);

    let p_style = if sorting {
        Style::default().fg(Color::Yellow)
    } else if complete {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::Gray)
    };

    let border_style = if sorting {
        Style::default().fg(Color::Yellow)
    } else if complete {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::White)
    };

    // Left block
    let text = vec![Spans::from(vec![Span::raw(sort_name)])];

    let block = Block::default()
        .title("Sort Name")
        .borders(Borders::ALL)
        .border_style(border_style);
    let paragraph = Paragraph::new(text)
        .block(block)
        .style(p_style)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, chunks[0]);

    // Right block
    let text = vec![Spans::from(vec![Span::raw(current_step)])];

    let block = Block::default()
        .title("Current Step")
        .borders(Borders::ALL)
        .border_style(border_style);
    let paragraph = Paragraph::new(text)
        .block(block)
        .style(p_style)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    f.render_widget(paragraph, chunks[1]);
}

pub fn draw_sort(f: &mut Frame<impl Backend>, chunk: Rect, sort_iter: &mut Box<dyn sort::Sort>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Length(3), Constraint::Percentage(100)].as_ref())
        .split(chunk);

    draw_sort_hud(
        f,
        chunks[0],
        sort_iter.get_name().as_str(),
        sort_iter.get_current_step().to_string().as_str(),
        sort_iter.is_active(),
        sort_iter.is_sorted(),
    );

    let data: Vec<(&'static str, u64)> = sort_iter
        .items()
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let p = sort_iter.get_pointer();
            if sort_iter.is_sorted() == true
                || sort_iter.is_sorted() == false && (p.0 != i && p.1 != i)
            {
                ("", *x as u64)
            } else {
                ("⬆", *x as u64)
            }
        })
        .collect();

    let border_style = if sort_iter.is_active() {
        Style::default().fg(Color::LightYellow)
    } else if sort_iter.is_sorted() {
        Style::default().fg(Color::LightGreen)
    } else {
        Style::default().fg(Color::Gray)
    };

    let chart = BarChart::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .data(&data)
        .bar_width(1)
        .bar_style(border_style)
        .label_style(Style::default().fg(Color::Red));

    f.render_widget(chart, chunks[1]);
}

pub fn draw_menu_footer(f: &mut Frame<impl Backend>, chunk: Rect) {
    let block = Block::default()
        .title("q/Ctrl-c: quit, enter: select sort")
        .style(Style::default().fg(Color::LightBlue).bg(Color::Reset));

    f.render_widget(block, chunk);
}

pub fn draw_sort_footer(f: &mut Frame<impl Backend>, chunk: Rect, active: bool, sorted: bool) {
    let title = if active {
        "Enter: pause sort, q: back to menu"
    } else if sorted {
        "Enter: restart sort, q: back to menu"
    } else {
        "Enter: start sort, q: back to menu"
    };

    let block = Block::default()
        .title(title)
        .style(Style::default().fg(Color::LightBlue).bg(Color::Reset));

    f.render_widget(block, chunk);
}

pub fn draw_single_sort(f: &mut Frame<impl Backend>, app: &mut App, chunk: Rect) {
    if let Some(sort) = app.sort.as_mut() {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Percentage(90),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .split(chunk);

        draw_header(
            f,
            chunks[0],
            sort.get_name(),
            sort.is_active(),
            sort.is_sorted(),
        );
        draw_sort(f, chunks[2], sort);
        draw_sort_footer(f, chunks[3], sort.is_active(), sort.is_sorted());
    }
}
