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

    pub fn register(mut self, action: Box<dyn Action + Send>) -> Self {
        let uuid = action.uuid();
        self.actions.insert(uuid.to_string(), action);
        self
    }

    pub(crate) fn get(&self, uuid: &String) -> &Box<dyn Action + Send> {
        self.actions.get(uuid).unwrap()
    }
}
