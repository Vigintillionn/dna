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
use plotter::{plot_comparisons, apply_expected};
use types::{Algorithm, AlgorithmData};

const MAX_SIZE_TO_TEST: usize = 10000;
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
    let mut data: Vec<(usize, Vec<usize>)> = vec![];
    let mut factors: Vec<f64> = vec![];
    // Test each array length
    for i in (0..=MAX_SIZE_TO_TEST).step_by(STEP_SIZE) {
        let mut results = vec![];
        for _ in 0..case.clone().iterations {
            let mut arr = (case.generator)(i);
            let comparisons = algorithm.sort(&mut arr);
            results.push(comparisons);
            if i > 0 {
                factors.push(comparisons as f64 / apply_expected(case.expected.clone().function, 1.0, i as f64) as f64);
            }
        }
        data.push((i, results));
    }
    println!("Factor for {} - {}: {}", algorithm.get_name(), case.name, factors.iter().sum::<f64>() / factors.len() as f64);
    // Plot the results
    plot_comparisons(
        data,
        algorithm.get_name(),
        case.clone().name,
        format!("{}-{}", algorithm.get_name(), case.name),
        case.expected,
    );
}
