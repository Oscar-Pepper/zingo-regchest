// use futures::prelude::*;
use std::{thread, time};
use zingo_testutils::{self, scenarios};

#[tokio::main]
async fn main() {
    let (_regtest_manager, mut child_process_handler) =
        scenarios::funded_orchard_mobileclient(1_000_000).await;
    println!("Successfully launched regtest environment!");
    loop {
        match child_process_handler.zcashd.try_wait() {
            Ok(Some(status)) => {
                println!("Process exited with status: {}", status);
                break;
            }
            Ok(None) => {
                // Process has not exited yet
            }
            Err(e) => {
                println!("Error while waiting for child: {:?}", e);
                break;
            }
        }
        match child_process_handler.lightwalletd.try_wait() {
            Ok(Some(status)) => {
                println!("Process exited with status: {}", status);
                break;
            }
            Ok(None) => {
                // Process has not exited yet
            }
            Err(e) => {
                println!("Error while waiting for child: {:?}", e);
                break;
            }
        }
        thread::sleep(time::Duration::from_millis(100))
    }
}
