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
use algorithms::quicksort::Quicksort;
use plotter::{plot_comparisons, apply_expected};
use types::{Algorithm, AlgorithmData};

const MAX_SIZE_TO_TEST: usize = 10000;
const STEP_SIZE: usize = 200;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let selection = SelectionSort::new();
    let insertion = InsertionSort::new();
    let merge = MergeSort::new();
    let quicksort = Quicksort::new();

    // Spawn threads for each algorithm
    // let selection_thread = thread::spawn(|| {
    //     let _ = test_algorithm(selection);
    // });

    // let insertion_thread = thread::spawn(|| {
    //     let _ = test_algorithm(insertion);
    // });

    // let merge_thread = thread::spawn(|| {
    //     let _ = test_algorithm(merge);
    // });

    let quicksort_thread = thread::spawn(|| {
        let _ = test_algorithm(quicksort);
    });

    let start = Instant::now();

    // Wait for all threads to finish
    // selection_thread.join().unwrap();
    // insertion_thread.join().unwrap();
    // merge_thread.join().unwrap();
    quicksort_thread.join().unwrap();

    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn test_algorithm(algorithm: impl Algorithm + Sync) -> Result<(), Box<dyn std::error::Error>> {
    // Test each case in parallel
    algorithm.get_cases().into_par_iter().for_each(|case| {
        let case_name = *case.name.clone();
        println!("Testing {} case: {}", algorithm.get_name(), case_name);
        test_case(&algorithm, case);
        println!("Finished {} case: {}", algorithm.get_name(), case_name);
    });

    Ok(())
}

fn test_case(algorithm: &(impl Algorithm + Sync), case: AlgorithmData) {
    let mut data: Vec<(usize, Vec<usize>)> = vec![];
    let mut factors: Vec<f64> = vec![];
    let case_name = *case.name;
    // Test each array length
    for i in (0..=MAX_SIZE_TO_TEST).step_by(STEP_SIZE) {
        let mut results = vec![];
        for _ in 0..case.iterations {
            let mut arr = (case.generator)(i);
            let comparisons = algorithm.sort(&mut arr);
            results.push(comparisons);
            if i > 0 {
                factors.push(comparisons as f64 / apply_expected(case.expected.clone().function, 1.0, i as f64) as f64);
            }
        }
        data.push((i, results));
    }
    println!("Factor for {} - {}: {}", algorithm.get_name(), case_name, factors.iter().sum::<f64>() / factors.len() as f64);
    // Plot the results
    plot_comparisons(
        data,
        algorithm.get_name(),
        case_name.clone(),
        format!("{}-{}", algorithm.get_name(), case_name.to_lowercase()),
        case.expected,
        case.plots
    );
}

fn distance(slice: &Vec<i32>) -> f64 {
    let mut distance: i32 = 0;
    for i in 0..slice.len() {
        let d = (slice[i] - i as i32).abs();
        distance += d;
    }
    distance as f64 / slice.len() as f64
}