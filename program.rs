use std::thread;

fn main() {
    let mut results = Vec::new();

    let values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for value in values {
        let handle = thread::spawn(move || {
            let result = value * 2;

            result
        });

        results.push(handle);
    }

    let computed_results: Vec<_> = results.into_iter().map(|handle| handle.join().unwrap()).collect();

    for result in computed_results {
        println!("Result: {}", result);
    }
}
