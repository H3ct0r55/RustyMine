use crate::domain::supervisor::Supervisor;

#[derive(Clone)]
pub struct AppState {
    pub supervisor: Supervisor,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            supervisor: Supervisor::new(),
        }
    }
}
