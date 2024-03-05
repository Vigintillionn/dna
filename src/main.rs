mod algorithms;
mod types;
mod generate_array;
mod plotter;

use std::vec;

use algorithms::selection::SelectionSort;
use algorithms::insertion::InsertionSort;
use algorithms::merge::MergeSort;
use types::{Sort, AlgorithmData}; // Import the Sort trait
use plotter::plot_comparisons;
use generate_array::{generate_random, generate_sorted, generate_reversed};

fn main() {
    let selection = SelectionSort::new();
    let insertion = InsertionSort::new();
    let merge = MergeSort::new();

    plot_data(&selection, vec![
        AlgorithmData {
            name: "selection-avg".to_string(),
            generator: generate_random,
        },
        AlgorithmData {
            name: "selection-best".to_string(),
            generator: generate_sorted,
        },
        AlgorithmData {
            name: "selection-worst".to_string(),
            generator: generate_reversed,
        },
    ]);

    plot_data(&insertion, vec![
        AlgorithmData {
            name: "insertion-avg".to_string(),
            generator: generate_random,
        },
        AlgorithmData {
            name: "insertion-best".to_string(),
            generator: generate_sorted,
        },
        AlgorithmData {
            name: "insertion-worst".to_string(),
            generator: generate_reversed,
        },
    ]);

    plot_data(&merge, vec![
        AlgorithmData {
            name: "merge-avg".to_string(),
            generator: generate_array::generate_random,
        },
        AlgorithmData {
            name: "merge-best".to_string(),
            generator: generate_array::generate_sorted,
        },
        AlgorithmData {
            name: "merge-worst".to_string(),
            generator: generate_array::generate_reversed,
        },
    ]);
}

fn plot_data<A: Sort<i32>>(sort: &A, cases: Vec<AlgorithmData>) {
    let mut overall = vec![];
    for case in cases {
        let mut data = vec![];
        let name = case.name.clone();
        for i in 1..1000 {
            let mut arr = (case.generator)(i);
            let comparisons = sort.sort(&mut arr);
            data.push((i, comparisons));
        }
        overall.push(data.clone());
        plot_comparisons(data, name);
    }
}
