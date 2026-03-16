use crate::persistance::greetings_repository::{GREETINGS_LIST, initialize_greetings};
use std::sync::OnceLock;

static INITIALIZED: OnceLock<()> = OnceLock::new();

fn ensure_greetings_initialized() {
    INITIALIZED.get_or_init(|| {
        initialize_greetings();
    });
}

pub fn retrieve_hello() -> String {
    ensure_greetings_initialized();
    if let Some(lock) = GREETINGS_LIST.get() {
        let greeting_list = lock.read().expect("Failed to acquire read lock");
        for greeting in greeting_list.iter() {
            if greeting.greeting_type == "BASIC" {
                return greeting.greeting_message.clone();
            }
        }
    }

    "No se encontró un saludo básico".to_string()
}

pub fn retrieve_hello_world() -> String {
    ensure_greetings_initialized();
    if let Some(lock) = GREETINGS_LIST.get() {
        let greeting_list = lock.read().expect("Failed to acquire read lock");
        for greeting in greeting_list.iter() {
            if greeting.greeting_type == "TECH" {
                return greeting.greeting_message.clone();
            }
        }
    }

    "No se encontró un saludo técnico".to_string()
}

pub fn retrieve_greetings() -> Vec<crate::persistance::greetings_repository::Greeting> {
    ensure_greetings_initialized();
    if let Some(lock) = GREETINGS_LIST.get() {
        let greeting_list = lock.read().expect("Failed to acquire read lock");
        return greeting_list.clone();
    }

    vec![]
}

pub fn save_greeting(greeting_message: String, greeting_type: String) {
    ensure_greetings_initialized();
    if let Some(lock) = GREETINGS_LIST.get() {
        let mut greeting_list = lock.write().expect("Failed to acquire write lock");
        let new_greeting = crate::persistance::greetings_repository::Greeting {
            greeting_message,
            greeting_type,
        };
        greeting_list.push(new_greeting);
    }
}
