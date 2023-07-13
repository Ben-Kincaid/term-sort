use crate::handlers;
use crate::sort;
use crossterm::event;
use std::io;
use tui::widgets::ListState;

#[derive(Clone, Copy)]
pub enum View {
    Menu,
    Insertion,
    Selection,
    Bubble,
    Shell,
    Merge,
    Heap,
    Quick,
    Quick3,
}

pub struct App {
    pub current_view: View,
    pub states: AppStates,
    pub ui_width: u16,
    pub sort: Option<Box<dyn sort::Sort>>,
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> Default for StatefulList<T> {
    fn default() -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items: Vec::new(),
        }
    }
}

impl<T> StatefulList<T> {
    pub fn set_items(mut self, items: Vec<T>) -> StatefulList<T> {
        self.items = items;
        self
    }
    pub fn initial_select(mut self, idx: usize) -> StatefulList<T> {
        self.state.select(Some(idx));
        self
    }
    pub fn next(&mut self) {
        if let Some(selected) = self.state.selected() {
            if selected + 1 >= self.items.len() {
                self.state.select(Some(0));
            } else {
                self.state.select(Some(selected + 1));
            }
        }
    }
    pub fn previous(&mut self) {
        if let Some(selected) = self.state.selected() {
            if selected == 0 {
                self.state.select(Some(self.items.len() - 1));
            } else {
                self.state.select(Some(selected - 1));
            }
        }
    }
}

pub struct MenuState {
    pub list: StatefulList<(&'static str, View)>,
}

impl<'a> MenuState {
    pub fn new() -> MenuState {
        let list = StatefulList::default()
            .set_items(vec![
                ("Insertion Sort", View::Insertion),
                ("Selection Sort", View::Selection),
                ("Bubble Sort", View::Bubble),
                ("Shell Sort", View::Shell),
                ("Merge Sort", View::Merge),
                ("Heap Sort", View::Heap),
                ("Quick Sort", View::Quick),
                ("Quick3 Sort", View::Quick3),
            ])
            .initial_select(0);
        MenuState { list }
    }
}

pub struct SortState<T>
where
    T: sort::Sort,
{
    pub sort: T,
}

pub struct AppStates {
    pub menu: Option<MenuState>,
}

impl AppStates {
    pub fn new() -> AppStates {
        let menu_state = MenuState::new();

        AppStates {
            menu: Some(menu_state),
        }
    }
}

impl App {
    pub fn new() -> App {
        let states = AppStates::new();

        App {
            current_view: View::Menu,
            ui_width: 0,
            states,
            sort: None,
        }
    }

    pub fn current_view(&self) -> &View {
        &self.current_view
    }

    pub fn set_current_view(&mut self, view: View) {
        self.current_view = view;
    }

    pub fn handle_input(&mut self, key: event::KeyEvent) -> Result<(), io::Error> {
        match self.current_view {
            View::Menu => {
                handlers::handle_menu_input(key, self)?;
            }
            _ => handlers::handle_sort_input(key, self)?,
        }
        Ok(())
    }
}
