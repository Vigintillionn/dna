use plotter::types::{Case, Plot, RGBColor, Algorithm};
use sort::insertionsort::sort;
use plotter::functions::generators::{generate_random, generate_sorted, generate_reversed, generate_with_dupes, generate_nearly_sorted};

fn main() {
  Algorithm::new("Insertion Sort", |arr| sort(arr, true))
    .with_cases(vec![
      Case::new("Average")
        .with_generator(generate_random)
        .iterations(10)
        .plots(vec![
          Plot::new(|x| 0.25 * x as f64 * x as f64, RGBColor(255, 0, 0), "Expected Avg")
        ])
        .set_color(RGBColor(0, 255, 0)),
      Case::new("Best")
        .with_generator(generate_sorted)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| x as f64, RGBColor(255, 0, 0), "Expected Best")
        ])
        .set_color(RGBColor(255, 0, 0)),
      Case::new("Worst")
        .with_generator(generate_reversed)
        .iterations(1)
        .plots(vec![
          Plot::new(|x| 0.5 * x as f64 * x as f64, RGBColor(255, 0, 0), "Expected Worst")
        ]),
    ])
    .run()
    // .plot("insertionsort")
    .plot_seperate(vec!["insertionsort-average", "insertionsort-best", "insertionsort-worst"])
    .unwrap();


}

// fn get_cases() -> Vec<Case> {
//   vec![
//     Case::new("Average")
//       .with_generators(vec![generate_random])
//       .iterations(100)
//       .plots(vec![
//         Plot::new(|x| 0.25 * x as f64 * x as f64, RGBColor(255, 0, 0), "Expected")
//       ]),

//     Case::new("Best")
//       .with_generators(vec![generate_sorted])
//       .iterations(1)
//       .plots(vec![
//         Plot::new(|x| x as f64, RGBColor(255, 0, 0), "Expected")
//       ]),

//     Case::new("Worst")
//       .with_generators(vec![generate_reversed])
//       .iterations(1)
//       .plots(vec![
//         Plot::new(|x| 0.5 * x as f64 * x as f64, RGBColor(255, 0, 0), "Expected")
//       ]),

//     Case::new("Dupes")
//       .with_generators(vec![generate_with_dupes])
//       .iterations(25)
//       .plots(vec![
//         Plot::new(|x| 0.25 * x as f64 * x as f64, RGBColor(255, 0, 0), "Expected")
//       ]),

//     Case::new("Nearly-sorted")
//       .with_generators(vec![|i| generate_nearly_sorted(i, 100)])
//       .iterations(25)
//       .plots(vec![
//         Plot::new(|x| 25.74 * x as f64, RGBColor(255, 0, 0), "Expected")
//       ]),
//   ]
// }