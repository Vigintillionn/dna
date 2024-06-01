use rand::prelude::*;

pub type MazeType = Vec<Vec<Maze>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Maze {
  Path,
  Wall,
}

// Find the shortest path from start to end in a maze
// using a brute force approach
pub fn find_shortest_path_bruteforce(maze: MazeType, start: (usize, usize), end: (usize, usize)) -> (usize, usize) {
  let mut paths: Vec<Vec<(usize, usize)>> = vec![];
  let mut path: Vec<(usize, usize)> = vec![];

  path.push(start);

  let mut count = 0;
  bruteforce_helper(&maze, end, path, &mut paths, &mut count);

  (paths.len(), count)
}

fn bruteforce_helper(maze: &MazeType, end: (usize, usize), path: Vec<(usize, usize)>, paths: &mut Vec<Vec<(usize, usize)>>, count: &mut usize) {
  *count += 1;
  let current = path[path.len() - 1];

  if current == end {
    paths.push(path.clone());
    return;
  }

  let (row, col) = current;
  
  if row < maze.len() && col < maze[0].len() && maze[row][col] == Maze::Path {
    let neighbors = vec![(row, col + 1), (row + 1, col)];

    for neighbor in neighbors {
      let mut path = path.clone();
      let (n_row, n_col) = neighbor;
      if n_row < maze.len() && n_col < maze[0].len() && maze[n_row][n_col] == Maze::Path {
        path.push(neighbor);
        bruteforce_helper(maze, end, path, paths, count);
      }
    }
  }
}

pub fn find_shorted_path_dynamic(maze: MazeType, start: (usize, usize), end: (usize, usize)) -> (usize, usize) {
  // Create a table to store the number of paths to each cell
  let mut table = vec![vec![std::usize::MIN; maze[0].len()]; maze.len()];
  table[start.0][start.1] = 1;
  

  let mut count = 0;
  // Iterate through the maze
  for row in 0..=end.0 {
    for col in 0..=end.1 {
      count += 1;
      // Skip the start cell and walls
      if (row == 0 && col == 0) || maze[row][col] == Maze::Wall {
        continue;
      }

      // Calculate the number of paths to the current cell
      let mut amount_of_paths = 0;
      if col > 0 {
        amount_of_paths += table[row][col - 1];
      }
      if row > 0 {
        amount_of_paths += table[row - 1][col];
      }

      // Store the number of paths in the table
      if table[row][col] < amount_of_paths {
        table[row][col] = amount_of_paths;
      }
    }
  }

  if table[maze.len() - 1][maze[0].len() - 1] == std::usize::MIN {
    (0, count)
  } else {
    (table[maze.len() - 1][maze[0].len() - 1], count)
  }
}

// Print the maze to the console
pub fn print_maze(maze: &MazeType) {
  for row in maze {
    for cell in row {
      match cell {
        Maze::Path => print!("."),
        Maze::Wall => print!("#"),
      }
    }
    println!();
  }
}

// Build a maze of size x size with a 30% chance of a wall at each cell
pub fn build_maze<F>(size: usize, wall_chance: F) -> MazeType
where F: Fn(usize, usize) -> f64 {
  let mut maze = vec![vec![Maze::Path; size]; size];
  add_walls(&mut maze, size, wall_chance);
  maze
}

// Add walls to the maze based on the wall_chance function
fn add_walls<F>(maze: &mut MazeType, size: usize, wall_chance: F)
where F: Fn(usize, usize) -> f64 {
  let mut rng = rand::thread_rng();

  for i in 0..size {
    for j in 0..size {
      if (i == 0 && j == 0) || (i == size - 1 && j == size - 1) {
        continue;
      }

      if wall_chance(i, j) > rng.gen::<f64>() {
        maze[i][j] = Maze::Wall;
      }
    }
  }
}

