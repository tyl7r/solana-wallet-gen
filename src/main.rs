use solana_sdk::{signature::Keypair};
use solana_sdk::signer::Signer;
use std::{thread, time::{Duration, Instant}};
use std::sync::{Arc, Mutex};

fn main() {
    //CONFIG
    // String = name you'd like to choose
    let name = "gay";
    // Amt of threads (recommended: 20 for high-end PC, 5 for potato)
    let threads = 5;

    let mut handles = vec![];

    let found_flag = Arc::new(Mutex::new(false));

    let start_time = Instant::now();
    let found_flag_clone = Arc::clone(&found_flag);
    let time_handle = thread::spawn(move || {
        loop {
            let elapsed = start_time.elapsed().as_secs();
            println!("Elapsed Time: {} seconds", elapsed);
            thread::sleep(Duration::from_secs(1));
            if *found_flag_clone.lock().unwrap() {
                break;
            }
        }
    });
    handles.push(time_handle);

    for _ in 0..threads {
        let found_flag_clone = Arc::clone(&found_flag);
        let handle = thread::spawn(move || {

            loop {
                if *found_flag_clone.lock().unwrap() {
                    break;
                }
                let wallet = Keypair::new();

                let pub_key_str = wallet.pubkey().to_string();

                if pub_key_str.starts_with(name) {
                    println!("Found matching wallet!");
                    println!("Wallet Public Key: {}", wallet.pubkey());
                    println!("Wallet Private Key: {:?}", wallet.to_bytes())
                    *found_flag_clone.lock().unwrap() = true;
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
