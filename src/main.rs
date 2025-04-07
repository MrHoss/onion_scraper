use dotenv::dotenv;
use rand::distr::{Alphanumeric, SampleString};
use reqwest::{blocking::Client, Proxy};
use std::{
    env,
    fs::OpenOptions,
    io::Write,
    sync::{mpsc, Arc, Mutex},
    thread,
    time::Duration,
};

fn generate_random_onion() -> String {
    let rand_str: String = Alphanumeric.sample_string(&mut rand::rng(), 56);
    // format!("http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion/")
    // format!("http://s4k4ceiapwwgcm3mkb6e4diqecpo7kvdnfr5gg7sph7jjppqkvwwqtyd.onion/")
    format!("http://{}.onion", rand_str.to_lowercase())

}

fn try_onion_site(client: &Client, url: &str, timeout: Duration) -> bool {
    match client.get(url).timeout(timeout).send() {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("[*] FOUND: {}", url);
                true
            } else {
                println!("[!] Invalid ({}) {}", resp.status(), url);
                false
            }
        }
        Err(e) => {
            println!("[!] Unreachable: {} -- {}",e.status().map(|s| s.as_u16() as i32).unwrap_or(-1), e.to_string());
            false
        }
    }
}

fn main() {
    dotenv().ok();

    let proxy_addr = env::var("PROXY_ADDRESS").unwrap_or_else(|_| "127.0.0.1:9050".to_string());
    let num_threads: usize = env::var("NUM_THREADS")
        .unwrap_or_else(|_| "4".to_string())
        .parse()
        .unwrap_or(4);
    let timeout_secs: u64 = env::var("TIMEOUT_SECS")
        .unwrap_or_else(|_| "10".to_string())
        .parse()
        .unwrap_or(10);
    let total_requests: usize = env::var("TOTAL_REQUESTS")
        .unwrap_or_else(|_| "40".to_string())
        .parse()
        .unwrap_or(40);
    let output_file = env::var("OUTPUT_FILE").unwrap_or_else(|_| "found_onions.txt".to_string());
  
    let client = Arc::new(
        Client::builder()
            .proxy(Proxy::all(&format!("socks5h://{}", proxy_addr)).unwrap())
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap(),
    );

    let (tx, rx) = mpsc::channel::<()>();
    let rx = Arc::new(Mutex::new(rx)); // <- coloca o receiver dentro de Arc<Mutex<>>

    for _ in 0..num_threads {
        let rx = Arc::clone(&rx);
        let client = Arc::clone(&client);
        let output_file = output_file.clone();

        thread::spawn(move || {
            loop {
                let received = {
                    let lock = rx.lock().unwrap();
                    lock.recv()
                };

                match received {
                    Ok(_) => {
                        let url = generate_random_onion();
                        let success = try_onion_site(&client, &url, Duration::from_secs(timeout_secs));

                        if success {
                            let mut file = OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(&output_file)
                                .unwrap();
                            writeln!(file, "{}", url).unwrap();
                        }
                    },
                    Err(_) => break, // canal foi fechado
                }
            }
        });
    }

    for _ in 0..total_requests {
        tx.send(()).unwrap();
    }

    drop(tx);

    thread::sleep(Duration::from_secs(timeout_secs + 5));

    println!("Finalizado.");
}
