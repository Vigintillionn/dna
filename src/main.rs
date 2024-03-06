mod algorithms;
mod generate_array;
mod plotter;
mod types;

use rayon::prelude::*;
use std::thread;
use std::time::Instant;
use std::vec;

use algorithms::insertion::InsertionSort;
use algorithms::merge::MergeSort;
use algorithms::selection::SelectionSort;
use plotter::plot_comparisons;
use types::{Algorithm, AlgorithmData};

const STEP_SIZE: usize = 200;

fn main() {
    let selection = SelectionSort::new();
    let insertion = InsertionSort::new();
    let merge = MergeSort::new();

    // Spawn threads for each algorithm
    let selection_thread = thread::spawn(|| {
        let _ = test_algorithm(selection);
    });

    let insertion_thread = thread::spawn(|| {
        let _ = test_algorithm(insertion);
    });

    let merge_thread = thread::spawn(|| {
        let _ = test_algorithm(merge);
    });

    let start = Instant::now();

    // Wait for all threads to finish
    selection_thread.join().unwrap();
    insertion_thread.join().unwrap();
    merge_thread.join().unwrap();

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn test_algorithm(algorithm: impl Algorithm + Sync) -> Result<(), Box<dyn std::error::Error>> {
    // Test each case in parallel
    algorithm.get_cases().into_par_iter().for_each(|case| {
        println!("Testing {} case: {}", algorithm.get_name(), case.clone().name);
        test_case(&algorithm, case.clone());
        println!("Finished {} case: {}", algorithm.get_name(), case.name);
    });

    Ok(())
}

fn test_case(algorithm: &(impl Algorithm + Sync), case: AlgorithmData) {
    let mut data = vec![];
    // Test each array length
    for i in (0..=10000).step_by(STEP_SIZE) {
        let mut arr = (case.generator)(i);
        let comparisons = algorithm.sort(&mut arr);
        data.push((i, comparisons));
    }
    // Plot the results
    plot_comparisons(
        data,
        algorithm.get_name(),
        case.clone().name,
        format!("{}-{}", algorithm.get_name(), case.name),
        case.expected,
    );
}
