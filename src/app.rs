use crate::handlers;
use crossterm::event;
use std::io;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

pub enum View {
    Menu,
    HeapSort,
}
pub struct App<'a> {
    pub current_view: View,
    pub states: AppStates<'a>,
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
    pub fn items(mut self, items: Vec<T>) -> StatefulList<T> {
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

pub enum Sort {
    Insertion,
    Selection,
    Bubble,
    Shell,
    Merge,
    Heap,
    Quick,
    Quick3,
}

pub struct MenuState<'a> {
    pub list: StatefulList<(ListItem<'a>, Sort)>,
}

impl<'a> MenuState<'a> {
    pub fn new() -> MenuState<'a> {
        let list = StatefulList::default()
            .items(vec![
                (ListItem::new("Insertion Sort"), Sort::Insertion),
                (ListItem::new("Selection Sort"), Sort::Selection),
                (ListItem::new("Bubble Sort"), Sort::Bubble),
                (ListItem::new("Shell Sort"), Sort::Shell),
                (ListItem::new("Merge Sort"), Sort::Merge),
                (ListItem::new("Heap Sort"), Sort::Heap),
                (ListItem::new("Quick Sort"), Sort::Quick),
                (ListItem::new("Quick3 Sort"), Sort::Quick3),
            ])
            .initial_select(0);
        MenuState { list }
    }
}

pub struct AppStates<'a> {
    pub menu: Option<MenuState<'a>>,
}

impl<'a> AppStates<'a> {
    pub fn new() -> AppStates<'a> {
        let menu_state = MenuState::new();

        AppStates {
            menu: Some(menu_state),
        }
    }
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        let states = AppStates::new();

        App {
            current_view: View::Menu,
            states,
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
                if let Some(ref mut menu) = self.states.menu.as_mut() {
                    handlers::handle_menu_input(key, self)?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
