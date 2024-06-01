use plotter::types::{Algorithm, Case, Plot, RED, GREEN, Combinator};

fn main() {
  Algorithm::new("Maze", get_bruteforce_complexity)
    .with_case(
      Case::new("Brute Force")
        .iterations(20)
        .step_size(1)
        .range((1, 15))
        .with_generator(|size| (0..size as i32).into_iter().collect())
        .plots(vec![
          Plot::new(|n| 2f64.powi(n as i32), GREEN, "2^n"),
          Plot::new(|n| 20f64 * 2f64.powi(n as i32), RED, "20*2^n")
        ])
    )
    .run()
    .plot("maze-bf")
    .unwrap();

  Algorithm::new("Maze", get_dp_complexity)
    .with_case(
      Case::new("Dynamic Programming")
        .iterations(20)
        .step_size(1)
        .range((1, 15))
        .with_generator(|size| (0..size as i32).into_iter().collect())
        .plots(vec![
          Plot::new(|n| n.pow(2) as f64, RED, "n^2"),
        ])
    )
    .run()
    .plot("maze-dp")
    .unwrap();

  Combinator::new("DP vs BF")
    .add_algorithms(vec![
      Algorithm::new("BF", get_bruteforce_complexity)
      .with_case(
        Case::new("BF")
          .iterations(20)
          .step_size(1)
          .range((1, 7))
          .with_generator(|size| (0..size as i32).into_iter().collect())
      )
      .set_color(RED),
      Algorithm::new("DP", get_dp_complexity)
        .with_case(
          Case::new("DP")
            .iterations(20)
            .step_size(1)
            .range((1, 7))
            .with_generator(|size| (0..size as i32).into_iter().collect())
        )
    ])
    .run()
    .plot("maze-combined")
    .unwrap();
}

fn get_bruteforce_complexity(arr: &mut[i32]) -> f64 {
  let maze = dynprog::maze::build_maze(arr.len(), |_, _| 0.2);
  let start = (0, 0);
  let end = (arr.len() - 1, arr.len() - 1);
  let brute = dynprog::maze::find_shortest_path_bruteforce(maze.clone(), start, end);
  brute.1 as f64
}

fn get_dp_complexity(arr: &mut[i32]) -> f64 {
  let maze = dynprog::maze::build_maze(arr.len(), |_, _| 0.2);
  let start = (0, 0);
  let end = (arr.len() - 1, arr.len() - 1);
  let dp = dynprog::maze::find_shorted_path_dynamic(maze, start, end);
  dp.1 as f64
}