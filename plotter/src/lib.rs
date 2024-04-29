use plotters::style::RGBColor;
use rayon::prelude::*;
use types::{Case, SortFunction};

pub mod functions;
pub mod plot;
pub mod types;

pub const MAX_ARRAY_SIZE: usize = 10000;
pub const STEP_SIZE: usize = 200;

pub fn test_sorting_algorithm(alg_name: &str, funcs: Vec<(SortFunction<i32>, RGBColor, &str)>, cases: Vec<Case>) {
    println!("{} | Running {} cases", alg_name, cases.len());
    cases.into_par_iter().for_each(move |case| {
        let case_name = case.name.clone();
        println!("{} | Running case: {}", alg_name, case_name);

        let start = std::time::Instant::now();
        test_case(&alg_name, &funcs, case);
        let duration = start.elapsed();

        println!("{} | Finished case: {} - took {}", alg_name, case_name, format!("{:?}", duration));
    });
}

fn test_case(alg_name: &str, funcs: &Vec<(SortFunction<i32>, RGBColor, &str)>, case: Case) {
    let mut data: Vec<(Vec<(usize, Vec<f64>)>, RGBColor, &str)> = vec![];

    for (func, color, name) in funcs {
        let mut func_data = vec![];

        for i in (0..=MAX_ARRAY_SIZE).step_by(STEP_SIZE) {
            let mut results = vec![];
            for _ in 0..case.iterations {
                let mut arr = (case.generator)(i);
                results.push(func(&mut arr[..]));
            }
    
            func_data.push((i, results));
        }

        data.push((func_data, *color, name));
    }

    let title = format!("{} - {}", alg_name, &case.name).to_owned();
    let file_name: String = format!("{}-{}", alg_name.to_lowercase(), &case.name.to_lowercase()).replace(" ", "-").to_owned();
    let _ = plot::plot_case(data, &title[..], &file_name[..], case.iterations, case.plots);
}