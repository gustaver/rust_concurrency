use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use rand::Rng;
use itertools::Itertools;
use std::error::Error;


fn random_lorem_ipsum() -> Result<String, Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..10);
    let url = format!("https://loripsum.net/api/{}/long/plaintext", n);
    let body = reqwest::blocking::get(url)?.text()?;
    Ok(body)
}

fn word_frequencies(s: String) -> HashMap<String, u32> {
    s.split_whitespace().fold(HashMap::new(), |mut acc, w| {
        *acc.entry(w.to_owned()).or_insert(0) += 1;
        acc
    })
}

fn merge_maps(to: &mut HashMap<String, u32>, from: HashMap<String, u32>) {
    for (key, val) in from.iter() {
        let count = to.entry(key.to_owned()).or_insert(0);
        *count += *val;
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();

    for i in 1..10 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            match random_lorem_ipsum() {
                Ok(body) => tx_clone.send((i, word_frequencies(body))).unwrap(),
                Err(e) => panic!("Error getting random lorem ipsum {:?}", e)
            }
        });
    }

    let mut word_frequencies: HashMap<String, u32> = HashMap::new();
    for (i, received) in rx {
        println!("Got response from thread {}", i);

        merge_maps(&mut word_frequencies, received);

        let top_words = word_frequencies
            .iter()
            .sorted_by_key(|(_, &c)| c)
            .rev()
            .take(10)
            .collect::<Vec<_>>();
        println!("Top words: {:?}", top_words);
    }
}
