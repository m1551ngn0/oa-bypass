#[derive(Clone)]
pub struct AppState {
    // Состояние приложения пустое, так как токен приходит от клиента
}

impl AppState {
    pub fn new() -> Self {
        Self {}
    }
}
