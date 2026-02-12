use crate::persistance::greetings_repository::{initialize_greetings, GREETINGS_LIST};
use std::sync::OnceLock;

static INITIALIZED: OnceLock<()> = OnceLock::new();

fn ensure_greetings_initialized() {
    INITIALIZED.get_or_init(|| {
        initialize_greetings();
    });
}

pub fn retrieve_hello() -> String {
    ensure_greetings_initialized();
    if let Some(greeting_list) = GREETINGS_LIST.get() {
        for greeting in greeting_list {
            if greeting.greeting_type == "BASIC" {
                return greeting.greeting_message.clone();
            }
        }
    }
    
    "No se encontró un saludo básico".to_string()
}

pub fn retrieve_hello_world() -> String {
    ensure_greetings_initialized();
    if let Some(greeting_list) = GREETINGS_LIST.get() {
        for greeting in greeting_list {
            if greeting.greeting_type == "TECH" {
                return greeting.greeting_message.clone();
            }
        }
    }
    
    "No se encontró un saludo técnico".to_string()
}
