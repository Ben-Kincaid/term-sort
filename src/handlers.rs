use crate::app::{App, Sort};
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
                if let Some((_, sort)) = menu.list.items.get(selected) {
                    match sort {
                        Sort::Heap => {
                            app.set_current_view(crate::app::View::HeapSort);
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    Ok(())
}
