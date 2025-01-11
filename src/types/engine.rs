use std::collections::HashMap;

use super::error::RunErr;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Storage {
    variables: HashMap<String, i32>,
}

#[allow(unused)]
impl Storage {
    pub fn new() -> Storage {
        Default::default()
    }

    pub fn clear(&mut self) {
        self.variables.clear();
    }

    pub fn createVariable(&mut self, variable: String, val: i32) -> Result<(), RunErr> {
        // This is a kinda weird bit of code (both because of the default rust formatting and because it's written that way)
        // but what I wanted to do was to simply return an error if the `insert` function returned Some().
        // This is because the `let` command is supposed to initialize a variable, and if it returns Some(), it means that there was a variable
        // with that name.
        self.variables
            .insert(variable, val)
            .map_or(Ok(()), |_| Err((RunErr::TriedToInitializeExistingVariable)))
    }

    pub fn modifyVariable<F: FnOnce(i32) -> i32>(
        &mut self,
        key: String,
        func: F,
    ) -> Result<(), RunErr> {
        match self.variables.get(&key) {
            Some(contents) => {
                self.variables.insert(key, func(*contents));
                Ok(())
            }
            None => Err(RunErr::TriedToModifyNonexistentVariable),
        }
    }

    pub fn getValue(&self, key: String) -> Result<i32, RunErr> {
        self.variables
            .get(&key)
            .copied()
            .ok_or(RunErr::TriedToGetNonexistentVariable)
    }
}
