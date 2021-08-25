use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

#[derive(Debug, Default, Clone)]
pub struct Manager {
    pub job_task_map: HashMap<String, HashSet<String>>, // only id -> id
}

#[allow(dead_code)]
pub struct Task {
    job_id: String,
    task_id: String,
}

#[allow(clippy::or_fun_call)]
fn _hash_check_exists_or_insert(arc: Arc<Mutex<Manager>>, task: &Task) {
    // WRONG
    /*  can't borrow data from ref as mutable
    arc.lock()
        .unwrap()
        .job_task_map
        .get(task.job_id.clone().as_ref())
        .get_or_insert(&HashSet::new())
        .insert(task.task_id.clone());
    */

    // RIGHT

    arc.lock()
        .unwrap()
        .job_task_map
        .entry(task.job_id.clone())
        .or_insert(HashSet::new())
        .borrow_mut()
        .insert(task.task_id.clone());
}
