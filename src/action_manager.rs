use crate::Action;
use std::collections::HashMap;

pub struct ActionManager {
    pub(crate) actions: HashMap<String, Box<dyn Action>>,
}

impl ActionManager {
    pub fn new() -> Self {
        ActionManager {
            actions: HashMap::new(),
        }
    }

    pub fn register(mut self, action: Box<dyn Action>) -> Self {
        let uuid = action.uuid();
        self.actions.insert(uuid.to_string(), action);
        self
    }

    pub(crate) fn get(&self, uuid: &String) -> &Box<dyn Action> {
        self.actions.get(uuid).unwrap()
    }
}
