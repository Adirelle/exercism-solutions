use std::collections::{BTreeSet, HashMap};

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct InputCellId(usize);

/// `ComputeCellId` is a unique identifier for a compute cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct ComputeCellId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

impl CallbackId {
    fn generate(&mut self) -> Self {
        self.0 += 1;
        *self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

type ComputeFunc<'a, T> = dyn 'a + Fn(&[T]) -> T;
type CallbackFunc<'a, T> = dyn 'a + FnMut(T);

pub struct Reactor<'a, T> {
    inputs: Vec<InputCell<T>>,
    computes: Vec<ComputeCell<'a, T>>,
}

type InputCell<T> = ValueCell<T>;

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq + 'a> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            inputs: Vec::new(),
            computes: Vec::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = InputCellId(self.inputs.len());
        self.inputs.push(InputCell {
            value: initial,
            deps: Vec::new(),
        });
        id
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        inputs: &[CellId],
        func: F,
    ) -> Result<ComputeCellId, CellId> {
        if let Some(error) = inputs.iter().find(|id| match id {
            CellId::Input(InputCellId(idx)) => *idx >= self.inputs.len(),
            CellId::Compute(ComputeCellId(idx)) => *idx >= self.computes.len(),
        }) {
            return Err(*error);
        }
        let id = ComputeCellId(self.computes.len());
        inputs.iter().for_each(|input| match input {
            CellId::Input(InputCellId(idx)) => self.inputs[*idx].add_deps(id),
            CellId::Compute(ComputeCellId(idx)) => self.computes[*idx].add_deps(id),
        });
        let value = self.eval(inputs, &func);
        self.computes
            .push(ComputeCell::new(value, inputs.to_vec(), Box::new(func)));
        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(InputCellId(idx)) => self.inputs.get(idx).map(InputCell::get),
            CellId::Compute(ComputeCellId(idx)) => self.computes.get(idx).map(ComputeCell::get),
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellId, value: T) -> bool {
        let mut dirty: BTreeSet<ComputeCellId> = BTreeSet::new();
        if let Some(cell) = self.inputs.get_mut(id.0) {
            if let Some(deps) = cell.set(value) {
                deps.iter().for_each(|id| _ = dirty.insert(*id))
            }
        } else {
            return false;
        }
        while let Some(ComputeCellId(idx)) = dirty.pop_first() {
            let value = {
                let cell = &self.computes[idx];
                self.eval(&cell.inputs, &cell.func)
            };
            if let Some(deps) = self.computes[idx].set(value) {
                deps.iter().for_each(|id| _ = dirty.insert(*id))
            }
        }
        true
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        self.computes
            .get_mut(id.0)
            .map(|cell| cell.add_callback(Box::new(callback)))
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let Some(cell) = self.computes.get_mut(cell.0) else {
            return Err(RemoveCallbackError::NonexistentCell);
        };
        if cell.remove_callback(callback) {
            Ok(())
        } else {
            Err(RemoveCallbackError::NonexistentCallback)
        }
    }

    fn eval<F: Fn(&[T]) -> T>(&self, inputs: &[CellId], func: F) -> T {
        let args = inputs
            .iter()
            .map(|input| match input {
                CellId::Input(InputCellId(idx)) => self.inputs[*idx].get(),
                CellId::Compute(ComputeCellId(idx)) => self.computes[*idx].get(),
            })
            .collect::<Vec<_>>();
        func(&args)
    }
}

struct ValueCell<T> {
    value: T,
    deps: Vec<ComputeCellId>,
}

impl<T: Copy + PartialEq> ValueCell<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            deps: Vec::new(),
        }
    }

    fn set(&mut self, value: T) -> Option<&[ComputeCellId]> {
        if value != self.value {
            self.value = value;
            Some(&self.deps)
        } else {
            None
        }
    }

    fn get(&self) -> T {
        self.value
    }

    fn add_deps(&mut self, id: ComputeCellId) {
        self.deps.push(id);
    }
}

struct ComputeCell<'a, T> {
    value: ValueCell<T>,
    func: Box<ComputeFunc<'a, T>>,
    inputs: Vec<CellId>,
    callback_id: CallbackId,
    callbacks: HashMap<CallbackId, Box<CallbackFunc<'a, T>>>,
}

impl<'a, T: Copy + PartialEq> ComputeCell<'a, T> {
    fn new(value: T, inputs: Vec<CellId>, func: Box<ComputeFunc<'a, T>>) -> Self {
        Self {
            value: ValueCell::new(value),
            func,
            inputs,
            callback_id: CallbackId(0),
            callbacks: HashMap::new(),
        }
    }

    fn set(&mut self, value: T) -> Option<&[ComputeCellId]> {
        let deps = self.value.set(value);
        if deps.is_some() {
            self.callbacks
                .values_mut()
                .for_each(|callback| callback(value));
        }
        deps
    }

    fn get(&self) -> T {
        self.value.get()
    }

    fn add_deps(&mut self, id: ComputeCellId) {
        self.value.deps.push(id);
    }

    fn add_callback(&mut self, func: Box<CallbackFunc<'a, T>>) -> CallbackId {
        let id = self.callback_id.generate();
        self.callbacks.insert(id, func);
        id
    }

    fn remove_callback(&mut self, id: CallbackId) -> bool {
        self.callbacks.remove(&id).is_some()
    }
}
