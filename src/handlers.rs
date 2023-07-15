use crate::app::{App, View};
use crate::sort::{
    bubble::BubbleSort, generate_random_data, insertion::InsertionSort, selection::SelectionSort,
    Sort,
};
use crossterm::event::{KeyCode, KeyEvent};
use std::io;

pub fn handle_menu_input(key: KeyEvent, app: &mut App) -> Result<(), io::Error> {
    if let Some(menu) = app.states.menu.as_mut() {
        match key.code {
            KeyCode::Up => {
                menu.list.previous();
            }
            KeyCode::Down => {
                menu.list.next();
            }
            KeyCode::Enter => {
                let selected = menu.list.state.selected().unwrap();
                let items = generate_random_data(app.ui_width as usize);

                if let Some((_, view)) = menu.list.items.get(selected) {
                    let view = view.clone();
                    app.sort = match view {
                        View::Bubble => Some(Box::new(BubbleSort::new(items))),
                        View::Insertion => Some(Box::new(InsertionSort::new(items))),
                        View::Selection => Some(Box::new(SelectionSort::new(items))),
                        _ => None,
                    };
                    app.set_current_view(view);
                }
            }
            _ => (),
        }
    }
    Ok(())
}

pub fn handle_sort_input(key: KeyEvent, app: &mut App) -> Result<(), io::Error> {
    match key.code {
        KeyCode::Enter => {
            if let Some(sort) = app.sort.as_mut() {
                if sort.is_sorted() {
                    let items = generate_random_data(app.ui_width as usize);
                    sort.reset(items);
                    sort.activate_sort();
                } else {
                    sort.toggle_sort();
                }
            }
        }
        _ => (),
    }
    Ok(())
}
