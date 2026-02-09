use std::sync::Arc;

use crate::application::use_cases::UseCases;

#[derive(Clone)]
pub struct AppState {
    pub use_cases: Arc<UseCases>
}