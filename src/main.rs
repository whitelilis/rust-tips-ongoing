mod cant_assign_to_which_is_behind_a_ref;
mod cant_borrow_data_from_ref_as_mutable;
mod enum_compare;
mod many_loop_in_tokio;
mod mv_between_collection;
mod run_system_command;
mod subtle_index;
mod tokio_test;
mod using_method_in_trait;

use std::sync::Arc;

// use tokio::sync::Mutex can share status between thread
use tokio::sync::Mutex;
/**  !!!! do not use std::sync:Mutex, between thread
use std::sync::Mutex;
*/

#[tokio::main]
async fn main() {
    let data1 = Arc::new(Mutex::new(0));
    let data2 = Arc::clone(&data1);

    tokio::spawn(async move {
        let mut lock = data2.lock().await;
        *lock += 1;
    });

    let mut lock = data1.lock().await;
    *lock += 1;
}
