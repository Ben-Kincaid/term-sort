use crate::app::{App, View};
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
                if let Some((_, view)) = menu.list.items.get(selected) {
                    match view {
                        View::Bubble => {
                            let items = rand::thread_rng()
                                .sample_iter::<f64, Standard>(Standard)
                                .take(app.ui_width as usize)
                                .map(|x| x * 100.0)
                                .collect();

                            app.states.bubble = Some(crate::app::SortState {
                                sort: crate::sort::BubbleSort::new(items),
                            });
                            app.set_current_view(crate::app::View::Bubble);
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
