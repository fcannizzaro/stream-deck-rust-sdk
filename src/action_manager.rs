use std::collections::HashMap;

use crate::Action;

pub struct ActionManager {
    pub(crate) actions: HashMap<String, Box<dyn Action + Send>>,
}

impl ActionManager {
    pub fn new() -> Self {
        ActionManager {
            actions: HashMap::new(),
        }
    }

    pub fn register(mut self, actions: Vec<Box<dyn Action + Send>>) -> Self {
        for action in actions {
            self.actions.insert(action.uuid().to_string(), action);
        }
        self
    }

    pub(crate) fn get(&self, uuid: &String) -> &Box<dyn Action + Send> {
        self.actions.get(uuid).unwrap()
    }

    pub(crate) fn shared(&self) -> Option<&Box<dyn Action + Send>> {
        self.actions.get(&"shared".to_string())
    }
}
