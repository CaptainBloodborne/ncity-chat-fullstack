pub mod user;
pub mod chat;
pub mod login;
pub mod app_state;
pub mod middlewares;
pub mod ctx;
pub mod errors;

const AUTH_TOKEN: &'static str = "auth-token";

// static PROTECTED_ROUTES: [(&str, &str); 1] = [
//     ("/api/user", "DELETE"),
// ];