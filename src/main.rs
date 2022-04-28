use dec::D128;
/// Entrypoint to the application that loads the different threads for the pipeline and then hooks them together
use mimalloc::MiMalloc;
use uuid::Uuid;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[macro_use]
extern crate logging;

use std::{thread, time::{UNIX_EPOCH, SystemTime}};
use tokio::{runtime::{Builder, Runtime}, time::Instant};
use crossbeam_channel::{Sender, Receiver, unbounded};

use rust_project_template::config::CONFIG;

/// Number of threads to have in the pool for each symbol pair added
const THREADS_PER_SYMBOL: usize = 3;

fn main() {
    if &CONFIG.env == "TEST" {
        info!("Test value = {}", &CONFIG.test_value);
    }
}