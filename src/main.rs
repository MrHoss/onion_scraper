use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let num_threads = 4;
    let images = (1..=40).collect::<Vec<_>>();
    let mut handles = vec![];

    for _ in 0..num_threads {
        let rx = Arc::clone(&rx);
        let handle = thread::spawn(move || {
            loop {
                let task = {
                    let lock = rx.lock().unwrap();
                    lock.recv()
                };

                match task {
                    Ok(image_id) => {
                        println!("Thread {:?} processando imagem {}", thread::current().id(), image_id);
                        thread::sleep(Duration::from_millis(500));
                        println!("Thread {:?} terminou imagem {}", thread::current().id(), image_id);
                    }
                    Err(_) => break,
                }
            }
        });

        handles.push(handle);
    }

    for img in images {
        tx.send(img).unwrap();
    }

    drop(tx); // Fecha o canal

    // Aguarda todas as threads terminarem
    for handle in handles {
        handle.join().unwrap();
    }
}
