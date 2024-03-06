mod algorithms;
mod types;
mod generate_array;
mod plotter;

use std::vec;
use std::thread;
use std::time::Instant;
use rayon::prelude::*;

use algorithms::selection::SelectionSort;
use algorithms::insertion::InsertionSort;
use algorithms::merge::MergeSort;
use types::{Algorithm, AlgorithmData};
use plotter::plot_comparisons;

const STEP_SIZE: usize = 200;

fn main() {
    let selection = SelectionSort::new();
    let insertion = InsertionSort::new();
    let merge = MergeSort::new();

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
    selection_thread.join().unwrap();
    insertion_thread.join().unwrap();
    merge_thread.join().unwrap();
    let duration = start.elapsed();
    println!("Time elapsed is: {:?}", duration);
}

fn test_algorithm(algorithm: impl Algorithm + Sync) -> Result<(), Box<dyn std::error::Error>> {
    algorithm.get_cases().into_par_iter().for_each(|case| {
        println!("Testing {} case: {}", algorithm.get_name(), case.clone().name);
        test_case(&algorithm, case.clone());
        println!("Finished {} case: {}", algorithm.get_name(), case.name);
    });

    Ok(())
}

fn test_case(
    algorithm: &(impl Algorithm + Sync),
    case: AlgorithmData
) {
    let mut data = vec![];
    for i in (0..=10000).step_by(STEP_SIZE) {
        let mut arr = (case.generator)(i);
        let comparisons = algorithm.sort(&mut arr);
        data.push((i, comparisons));
    }
    plot_comparisons(
        data, 
        algorithm.get_name(), 
        case.clone().name, 
        format!("{}-{}", algorithm.get_name(), case.name),
        case.expected
    );
}