use std::thread;
use std::process::Command;

fn main() {
  let binaries = [
    "insertionsort", 
    "selectionsort",
    "mergesort",
    "quicksort",
    "balance-1-twothree",
    "balance-1",
    "balance-2",
  ];

  let handles: Vec<_> = binaries.iter().map(|bin| {
    let bin = bin.to_string();
    thread::spawn(move || {
      Command::new("cargo")
        .args(&["run", "--bin", &bin])
        .status()
        .expect("failed to execute process")
    })
  }).collect();

  for handle in handles {
    handle.join().unwrap();
  }
}