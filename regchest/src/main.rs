use std::{thread, time};
use zingo_testutils::{self, scenarios};

#[tokio::main]
async fn main() {
    let (regtest_manager, mut child_process_handler) =
        scenarios::funded_orchard_mobileclient(1_000_000).await;
    regtest_manager
        .generate_n_blocks(10)
        .expect("Failed to generate blocks.");
    println!("Successfully launched regchest!");

    loop {
        match child_process_handler.zcashd.try_wait() {
            Ok(Some(status)) => {
                println!("Zcashd exited with status: {}", status);
                break;
            }
            Ok(None) => {
                // Process has not exited yet
            }
            Err(e) => {
                println!("Error while waiting for zcashd: {:?}", e);
                break;
            }
        }
        match child_process_handler.lightwalletd.try_wait() {
            Ok(Some(status)) => {
                println!("Lightwalletd exited with status: {}", status);
                break;
            }
            Ok(None) => {
                // Process has not exited yet
            }
            Err(e) => {
                println!("Error while waiting for lightwalletd: {:?}", e);
                break;
            }
        }
        thread::sleep(time::Duration::from_millis(100))
    }
}
