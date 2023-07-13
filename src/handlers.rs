use crate::app::{App, View};
use crate::sort::bubble::BubbleSort;
use crossterm::event::{KeyCode, KeyEvent};
use rand::{distributions::Standard, Rng};
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
                let items: Vec<f64> = rand::thread_rng()
                    .sample_iter::<f64, Standard>(Standard)
                    .take(app.ui_width as usize)
                    .map(|x| x * 100.0)
                    .collect();

                if let Some((_, view)) = menu.list.items.get(selected) {
                    let view = view.clone();
                    app.sort = match view {
                        View::Bubble => Some(Box::new(BubbleSort::new(items))),
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
                sort.activate_sort();
            }
        }
        _ => (),
    }
    Ok(())
}
