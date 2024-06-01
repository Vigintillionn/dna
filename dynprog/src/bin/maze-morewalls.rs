use plotter::types::{Algorithm, Case, Plot, RED};

fn main() {
  Algorithm::new("Maze", get_bruteforce_complexity)
    .with_case(
      Case::new("Brute Force")
        .iterations(20)
        .step_size(1)
        .range((1, 15))
        .with_generator(|size| (0..size as i32).into_iter().collect())
        .plots(vec![
        ])
    )
    .run()
    .plot("maze-bf-mw")
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
    .plot("maze-dp-mw")
    .unwrap();
}

fn get_bruteforce_complexity(arr: &mut[i32]) -> f64 {
  let maze = dynprog::maze::build_maze(arr.len(), |i, j| if i == 0 || j == 0 || i == arr.len() - 1 || j == arr.len() - 1 { 0.0 } else { 0.8 });
  let start = (0, 0);
  let end = (arr.len() - 1, arr.len() - 1);
  let brute = dynprog::maze::find_shortest_path_bruteforce(maze.clone(), start, end);
  brute.1 as f64
}

fn get_dp_complexity(arr: &mut[i32]) -> f64 {
  let maze = dynprog::maze::build_maze(arr.len(), |i, j| if i == 0 || j == 0 || i == arr.len() - 1 || j == arr.len() - 1 { 0.0 } else { 0.8 });
  let start = (0, 0);
  let end = (arr.len() - 1, arr.len() - 1);
  let dp = dynprog::maze::find_shorted_path_dynamic(maze, start, end);
  dp.1 as f64
}