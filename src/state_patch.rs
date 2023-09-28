use std::{
    rc::Rc, collections::{HashMap, HashSet},
};

use crate::{object::{RTObject, Object}, container::Container};

#[derive(Clone)]
pub struct StatePatch {
    pub globals: HashMap<String, Rc<dyn RTObject>>,
    pub changed_variables: HashSet<String>,
    pub visit_counts: HashMap<String, usize>,
    pub turn_indices: HashMap<String, usize>,
}

impl StatePatch {
    pub fn new(to_copy: Option<&StatePatch>) -> StatePatch {
        match to_copy {
            Some(to_copy) => StatePatch {
                globals: to_copy.globals.clone(),
                changed_variables: to_copy.changed_variables.clone(),
                visit_counts: to_copy.visit_counts.clone(),
                turn_indices: to_copy.turn_indices.clone(),
            },
            None => StatePatch {
                globals: HashMap::new(),
                changed_variables: HashSet::new(),
                visit_counts:  HashMap::new(),
                turn_indices:  HashMap::new(),
            },
        }
    }

    pub fn get_visit_count(&self, container: &Rc<Container>) -> Option<usize> {
        let key = Object::get_path(container.as_ref()).to_string();
        self.visit_counts.get(&key).copied()
    }

    pub fn set_visit_count(&mut self, container: &Rc<Container>, count: usize) {
        let key = Object::get_path(container.as_ref()).to_string();
        self.visit_counts.insert(key, count);
    }

    pub fn  get_global(&self, name: &str) -> Option<Rc<dyn RTObject>>{
        self.globals.get(name).cloned()
    }

    pub fn set_global(&mut self, name: &str, value: Rc<dyn RTObject>) {
        self.globals.insert(name.to_string(), value);
    }

    pub(crate) fn add_changed_variable(&mut self, name: &str) {
        self.changed_variables.insert(name.to_string());
    }

    pub(crate) fn set_turn_index(&mut self, container: &Container, index: i32) {
        let key = Object::get_path(container).to_string();
        self.turn_indices.insert(key, index as usize);
    }

    pub(crate) fn get_turn_index(&self, container: &Container) -> Option<&usize> {
        let key = Object::get_path(container).to_string();
        return self.turn_indices.get(&key);
    }
}