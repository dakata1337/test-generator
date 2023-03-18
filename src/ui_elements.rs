use serde::Deserialize;
use tui::widgets::ListState;

#[derive(Deserialize)]
pub struct StatefulList<T> {
    #[serde(skip)]
    state: ListState,
    inner: Vec<T>,
}
impl<T> StatefulList<T> {
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.inner.iter()
    }
}
impl<T> StatefulList<T> {
    pub fn get_state(&mut self) -> &mut ListState {
        &mut self.state
    }
}

impl<T> Default for StatefulList<T> {
    fn default() -> Self {
        Self {
            state: Default::default(),
            inner: Default::default(),
        }
    }
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            inner: items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.inner.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.inner.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
