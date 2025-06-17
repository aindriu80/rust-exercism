use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    input_cells: HashMap<InputCellId, T>,
    compute_cells: HashMap<ComputeCellId, ComputeCell<'a, T>>,
    next_input_id: usize,
    next_compute_id: usize,
    next_callback_id: usize,
}

struct ComputeCell<'a, T> {
    dependencies: Vec<CellId>,
    compute_func: Box<dyn Fn(&[T]) -> T + 'a>,
    value: T,
    callbacks: HashMap<CallbackId, Box<dyn FnMut(T) + 'a>>,
}

impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Reactor {
            input_cells: HashMap::new(),
            compute_cells: HashMap::new(),
            next_input_id: 0,
            next_compute_id: 0,
            next_callback_id: 0,
        }
    }

    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = InputCellId(self.next_input_id);
        self.next_input_id += 1;
        self.input_cells.insert(id, initial);
        id
    }

    pub fn create_compute<F>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId>
    where
        F: Fn(&[T]) -> T + 'a,
    {
        // Check if all dependencies exist
        for &dep in dependencies {
            if self.value(dep).is_none() {
                return Err(dep);
            }
        }

        // Calculate initial value
        let dep_values: Vec<T> = dependencies
            .iter()
            .map(|&dep| self.value(dep).unwrap())
            .collect();
        let initial_value = compute_func(&dep_values);

        let id = ComputeCellId(self.next_compute_id);
        self.next_compute_id += 1;

        let compute_cell = ComputeCell {
            dependencies: dependencies.to_vec(),
            compute_func: Box::new(compute_func),
            value: initial_value,
            callbacks: HashMap::new(),
        };

        self.compute_cells.insert(id, compute_cell);
        Ok(id)
    }

    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(input_id) => self.input_cells.get(&input_id).copied(),
            CellId::Compute(compute_id) => {
                self.compute_cells.get(&compute_id).map(|cell| cell.value)
            }
        }
    }

    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        if !self.input_cells.contains_key(&id) {
            return false;
        }

        self.input_cells.insert(id, new_value);
        self.propagate_changes();
        true
    }

    fn propagate_changes(&mut self) {
        // Collect all compute cell IDs for processing
        let mut compute_ids: Vec<ComputeCellId> = self.compute_cells.keys().copied().collect();

        // Sort by ID to ensure consistent processing order
        compute_ids.sort_by_key(|id| id.0);

        // Track which cells changed values
        let mut cells_to_callback: Vec<(ComputeCellId, T, T)> = Vec::new(); // (id, old_value, new_value)

        // Propagate changes until no more changes occur
        let mut changed = true;
        while changed {
            changed = false;

            for &compute_id in &compute_ids {
                let (new_value, old_value) = {
                    let cell = self.compute_cells.get(&compute_id).unwrap();
                    let dep_values: Vec<T> = cell
                        .dependencies
                        .iter()
                        .map(|&dep| self.value(dep).unwrap())
                        .collect();
                    let new_val = (cell.compute_func)(&dep_values);
                    (new_val, cell.value)
                };

                if new_value != old_value {
                    changed = true;
                    cells_to_callback.push((compute_id, old_value, new_value));
                    self.compute_cells.get_mut(&compute_id).unwrap().value = new_value;
                }
            }
        }

        // Fire callbacks for cells that changed
        for (compute_id, _old_value, new_value) in cells_to_callback {
            let callback_ids: Vec<CallbackId> = self
                .compute_cells
                .get(&compute_id)
                .unwrap()
                .callbacks
                .keys()
                .copied()
                .collect();

            for callback_id in callback_ids {
                if let Some(cell) = self.compute_cells.get_mut(&compute_id) {
                    if let Some(callback) = cell.callbacks.get_mut(&callback_id) {
                        callback(new_value);
                    }
                }
            }
        }
    }

    pub fn add_callback<F>(&mut self, id: ComputeCellId, callback: F) -> Option<CallbackId>
    where
        F: FnMut(T) + 'a,
    {
        if let Some(cell) = self.compute_cells.get_mut(&id) {
            let callback_id = CallbackId(self.next_callback_id);
            self.next_callback_id += 1;
            cell.callbacks.insert(callback_id, Box::new(callback));
            Some(callback_id)
        } else {
            None
        }
    }

    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        if let Some(compute_cell) = self.compute_cells.get_mut(&cell) {
            if compute_cell.callbacks.remove(&callback).is_some() {
                Ok(())
            } else {
                Err(RemoveCallbackError::NonexistentCallback)
            }
        } else {
            Err(RemoveCallbackError::NonexistentCell)
        }
    }
}
