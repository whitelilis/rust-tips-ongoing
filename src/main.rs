mod cant_borrow_data_from_ref_as_mutable;
mod mv_between_collection;

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
