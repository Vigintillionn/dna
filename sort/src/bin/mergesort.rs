use plotter::types::{Case, Plot, RGBColor, Algorithm};
use sort::mergesort::sort;
use plotter::test_sorting_algorithm;
use plotter::functions::generators::{generate_random, generate_sorted, generate_interleaved, generate_reversed};

fn main() {
  // test_sorting_algorithm(
  //   "Merge Sort",
  //   vec![
  //     (Box::new(sort), RGBColor(0, 0, 255), "Measured")
  //   ], 
  //   get_cases()
  // )

  Algorithm::new("Merge Sort", sort)
    .with_cases(vec![
      Case::new("Average")
        .with_generator(generate_random)
        .iterations(100)
        .plots(vec![
          Plot::new(|x| 0.85 * x as f64 * (x as f64).log2(), RGBColor(255, 0, 0), "Expected")
        ]),
      Case::new("Best")
        .with_generator(generate_sorted)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| 0.5 * x as f64 * (x as f64).log2(), RGBColor(255, 0, 0), "Expected")
        ]),
      Case::new("Worst")
        .with_generator(generate_reversed)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| x as f64 * (x as f64).log2(), RGBColor(255, 0, 0), "Expected")
        ]),
    ])
    .run()
    .plot("mergesort")
    .unwrap();

}

// fn get_cases() -> Vec<Case> {
//   vec![
//     Case::new("Average")
//       .with_generators(vec![generate_random])
//       .iterations(100)
//       .plots(vec![
//         Plot::new(|x| 0.85 * x as f64 * (x as f64).log2(), RGBColor(255, 0, 0), "Expected")
//       ]),

//     Case::new("Best")
//       .with_generators(vec![generate_sorted])
//       .iterations(1)
//       .plots(vec![
//         Plot::new(|x| 0.5 * x as f64 * (x as f64).log2(), RGBColor(255, 0, 0), "Expected")
//       ]),

//     Case::new("Worst")
//       .with_generators(vec![generate_interleaved])
//       .iterations(1)
//       .plots(vec![
//         Plot::new(|x| x as f64 * (x as f64).log2(), RGBColor(255, 0, 0), "Expected")
//       ]),
//   ]
// }