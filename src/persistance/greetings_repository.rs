use std::sync::OnceLock;

pub(crate) static GREETINGS_LIST: OnceLock<Vec<Greeting>> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct Greeting {
    pub greeting_message: String,
    pub greeting_type: String,
}
pub fn initialize_greetings() {
    let greetings = vec![
        Greeting {
            greeting_message: "¡Hola desde Rust!".to_string(),
            greeting_type: "BASIC".to_string(),
        },
        Greeting {
            greeting_message: "¡Hola mundo desde Rust!".to_string(),
            greeting_type: "TECH".to_string(),
        },
        Greeting {
            greeting_message: "¡Saludos desde Rust, es un placer saludarte!".to_string(),
            greeting_type: "FORMAL".to_string(),
        },
    ];

    GREETINGS_LIST.set(greetings).expect("Failed to initialize greetings list");
}
