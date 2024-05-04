use std::{iter, error::Error};
use rayon::prelude::*;

pub use plotters::style::RGBColor;
use crate::plot::plot_case;

pub const MAX_ARRAY_SIZE: usize = 10000;
pub const STEP_SIZE: usize = 200;

pub const BLUE: RGBColor = RGBColor(0, 0, 255);
pub const GREEN: RGBColor = RGBColor(0, 255, 0);
pub const RED: RGBColor = RGBColor(255, 0, 0);
pub const BLACK: RGBColor = RGBColor(0, 0, 0);

pub type PlotFunction = Box<dyn Fn(usize) -> f64 + Send + Sync>;
pub type AlgorithmFunction<T> = Box<dyn Fn(&mut [T]) -> f64 + Send + Sync>;
pub type GeneratorFunction<T> = Box<dyn Fn(usize) -> Vec<T> + Send + Sync>;

#[derive(Clone)]
pub struct AlgorithmResult {
  pub data: Vec<(usize, Vec<f64>)>,
  pub name: String,
  pub color: RGBColor
}

impl AlgorithmResult {
  pub fn new(data: Vec<(usize, Vec<f64>)>, name: &str, color: RGBColor) -> AlgorithmResult {
    AlgorithmResult {
      data,
      name: name.to_string(),
      color
    }
  }

  pub fn set_name(mut self, name: String) -> AlgorithmResult {
    self.name = name;
    self
  }

  pub fn set_color(mut self, color: RGBColor) -> AlgorithmResult {
    self.color = color;
    self
  }
}

pub struct Algorithm<T> {
  name: String,
  func: AlgorithmFunction<T>,
  generator: Option<GeneratorFunction<T>>,
  cases: Option<Vec<Case<T>>>,
  results: Option<Vec<AlgorithmResult>>,
  color: RGBColor,
  label: String
}

impl<T> Algorithm<T> {
  pub fn new<F>(name: &str, func: F) -> Algorithm<T>
  where F: Fn(&mut [T]) -> f64 + Send + Sync + 'static {
    Algorithm { 
      name: name.to_string(), 
      func: Box::new(func),
      generator: None,
      cases: None,
      results: None,
      color: BLUE,
      label: "Measured".to_string()
    }
  }

  pub fn with_case(mut self, case: Case<T>) -> Algorithm<T> {
    self.cases = Some(vec![case]);
    self
  }

  pub fn with_cases(mut self, cases: Vec<Case<T>>) -> Algorithm<T> {
    self.cases = Some(cases);
    self
  }

  pub fn with_generator<G>(mut self, generator: G) -> Algorithm<T>
  where G: Fn(usize) -> Vec<T> + Send + Sync + 'static {
    self.generator = Some(Box::new(generator));
    self
  }

  pub fn set_color(mut self, color: RGBColor) -> Algorithm<T> {
    self.color = color;
    self
  }

  pub fn set_label(mut self, label: &str) -> Algorithm<T> {
    self.label = label.to_string();
    self
  }

  pub fn plot(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
    if self.results.is_none() {
      return Err("No results found, consider running the algorithm first using run().".into());
    }

    println!("Plotting {}", &self.name);

    let plots = if let Some(cases) = &self.cases {
      cases.iter().map(|c| &c.plots).flatten().collect()
    } else {
      vec![]
    };

    let iterations = if let Some(cases) = &self.cases {
      cases.iter().map(|c| c.iterations).reduce(|a, b| a.max(b)).unwrap_or(1)
    } else {
      1
    };

    if let Some(results) = &self.results {
      plot_case(
        results.clone(),
        &self.name[..],
        file_name,
        iterations,
        &plots
      )?;
    }

    Ok(())
  }

  pub fn plot_seperate(&self, file_names: Vec<&str>) -> Result<(), Box<dyn Error>> {
    if self.results.is_none() {
      return Err("No results found, consider running the algorithm first using run().".into());
    }

    if file_names.len() != self.results.as_ref().unwrap().len() {
      return Err("Number of file names does not match the number of results".into());
    }

    let plots = if let Some(cases) = &self.cases {
      cases.iter().map(|c| &c.plots).collect()
    } else {
      vec![]
    };

    self.results
      .as_ref()
      .unwrap()
      .into_par_iter()
      .zip(file_names)
      .zip(plots)
      .for_each(|((results, file_name), plots)| {
        let title = format!("{} - {}", &self.name[..], &results.name[..]);

        println!("Plotting {}", &title[..]);

        let results = results.clone().set_color(self.color);
        let plots: Vec<&Plot> = plots.iter().map(|p| p).collect();

        let res = plot_case(
          vec![results.clone()],
          &title[..],
          file_name,
          results.data[0].1.len(),
          &plots
        );

        if let Err(e) = res {
          println!("Error plotting: {}", e);
        }
      });

    Ok(())
  }

  pub fn get_results(&self) -> Option<Vec<AlgorithmResult>> {
    self.results.clone()
  }

  pub fn get_cases(&self) -> Option<&Vec<Case<T>>> {
    self.cases.as_ref()
  }
}

impl<T: Ord + Copy> Algorithm<T> {
  pub fn run(&mut self) -> &Algorithm<T> {
    println!("Running algorithm: {}", &self.name[..]);
    let start_alg = std::time::Instant::now();

    let binding = vec![Case::default()];
    let cases = self.cases.as_ref().unwrap_or(&binding);

    let result: Vec<AlgorithmResult> = cases.into_par_iter().map(|case| {
      println!("{} | Running case: {}", &self.name[..], &case.name[..]);
      let start_case = std::time::Instant::now();

      let generator = if let Some(gen) = &case.generator {
        gen
      } else {
        if let Some(gen) = &self.generator {
          gen
        } else {
          panic!("No generator found for case: {}", &case.name[..]);
        }
      };

      let color = case.color.unwrap_or(self.color);
      let (low, high) = case.range;

      let case_result: Vec<(usize, Vec<f64>)> = (low..=high).step_by(case.step_size).into_iter().map(|i| {
        let results: Vec<f64> = (0..case.iterations)
          .into_par_iter()
          .map(|_| {
            let mut arr = (generator)(i);
            (self.func)(&mut arr[..])
          })
          .collect();
        
          (i, results)
      }).collect();
      

      let duration = start_case.elapsed();
      println!("{} | Finished case: {} - took {:?}", &self.name[..], &case.name[..], duration);

      AlgorithmResult::new(case_result, &case.name[..], color)
    }).collect();

    let duration = start_alg.elapsed();
    println!("Finished algorithm: {} - took {:?}", &self.name[..], duration);

    self.results = Some(result);
    self    
  }
}

pub struct Combinator<T> {
  title: String,
  algs: Vec<Algorithm<T>>,
  results: Option<Vec<AlgorithmResult>>
}

impl<T> Combinator<T> {
  pub fn new(name: &str) -> Combinator<T> {
    Combinator {
      title: name.to_string(),
      algs: vec![],
      results: None
    }
  }

  pub fn with_algorithms(mut self, algs: Vec<Algorithm<T>>) -> Combinator<T> {
    self.algs = algs;
    self
  }

  pub fn add_algorithm(mut self, alg: Algorithm<T>) -> Combinator<T> {
    self.algs.push(alg);
    self
  }

  pub fn add_algorithms(mut self, algs: Vec<Algorithm<T>>) -> Combinator<T> {
    self.algs = algs;
    self
  }

  pub fn plot(&self, file_name: &str) -> Result<(), Box<dyn Error>> {
    println!("Plotting algorithms {}", self.algs.iter().map(|a| &a.name[..]).collect::<Vec<&str>>().join(", "));

    let cases: Vec<&Case<T>> = self.algs
      .iter()
      .filter(|a| a.get_cases().is_some())
      .map(|a| a.get_cases().unwrap()).flatten().collect();

    let plots: Vec<&Plot> = cases.iter().map(|c| &c.plots).flatten().collect();
    let max_iterations = cases.iter().map(|c| c.iterations.clone()).reduce(|a, b| a.max(b)).unwrap_or(1);

    if let Some(result) = &self.results {
      plot_case(
        result.clone(),
        &self.title[..], 
        file_name, 
        max_iterations, 
        &plots
      )?;
    }

    Ok(())
  }

  pub fn plot_seperate(&self, file_names: Vec<&str>) -> Result<(), Box<dyn Error>> {
    if self.results.is_none() {
      return Err("No results found, consider running the combinator first using run().".into());
    }

    // Group each case together
    // Eg [Case1.1, Case 1.2, Case 2.1, Case 2.2] -> [[Case 1.1, Case 2.1], [Case 1.2, Case 2.2]]
    if !self.algs.iter().all(|a| a.get_cases().is_some() || a.get_cases().unwrap().len() == file_names.len()) {
      return Err("Number of file names does not match the number of results or not all algorithms have the same amount of cases".into());
    }

    for i in 0..self.algs[0].get_cases().unwrap().len() {
      let cases: Vec<&Case<T>> = self.algs.iter().map(|a| &a.get_cases().unwrap()[i]).collect();
      let results: Vec<AlgorithmResult> = self.algs.iter().map(|a| a.get_results().unwrap()[i].clone()).collect();

      let plots: Vec<&Plot> = cases.iter().map(|c| &c.plots).flatten().collect();

      let title = format!("{} - {}", &self.title[..], &results[0].name[..]);

      println!("Plotting {}", &title[..]);

      plot_case(
        results.clone(),
        &title[..],
        file_names[i],
        results[0].data[0].1.len(),
        &plots
      )?;
    }

    Ok(())
  
  }
}

// fn group_array<T>(arr: Vec<T>, n: usize) -> Vec<Vec<T>> {
//   let mut grouped: Vec<Vec<T>> = vec![vec![]; n];

//   for (i, res) in arr.into_iter().enumerate() {
//     let group = i % n;
//     grouped[group].push(res);
//   }

//   grouped
// }

impl<T: Ord + Copy + 'static> Combinator<T> {
  pub fn run(mut self) -> Combinator<T> {
    let (tx, rx) = std::sync::mpsc::channel::<Vec<AlgorithmResult>>();

    std::thread::scope(|s| {
      for (alg, tx) in iter::zip(&mut self.algs, iter::repeat(tx)) {
        s.spawn(move || tx.send(alg.run().get_results().unwrap()).unwrap());
      }
    });

    let results: Vec<AlgorithmResult> = rx.iter().flatten().collect();
    self.results = Some(results);
    self
  }
}

pub struct Plot {
  pub func: PlotFunction,
  pub color: RGBColor,
  pub name: String
}

impl Plot {
  pub fn new<F>(func: F, color: RGBColor, name: &str) -> Plot
  where F: Fn(usize) -> f64 + Send + Sync + 'static {
    Plot {
      func: Box::new(func),
      color,
      name: name.to_string()
    }
  }
}

pub struct Case<T> {
  pub name: String,
  pub generator: Option<GeneratorFunction<T>>,
  pub iterations: usize,
  pub plots: Vec<Plot>,
  pub range: (usize, usize),
  pub step_size: usize,
  pub color: Option<RGBColor>
}

impl<T> Case<T> {
  pub fn new(name: &str) -> Case<T> {
    Case {
      name: name.to_string(),
      generator: None,
      iterations: 1,
      plots: vec![],
      range: (0, MAX_ARRAY_SIZE),
      step_size: STEP_SIZE,
      color: None,
    }
  }

  pub fn default() -> Case<T> {
    Case::new("")
  }

  pub fn with_generator<G>(mut self, generator: G) -> Case<T>
  where G: Fn(usize) -> Vec<T> + Send + Sync + 'static {
    self.generator = Some(Box::new(generator));
    self
  }

  pub fn iterations(mut self, iterations: usize) -> Case<T> {
    self.iterations = iterations;
    self
  }

  pub fn plots(mut self, plots: Vec<Plot>) -> Case<T> {
    self.plots = plots;
    self
  }

  pub fn range(mut self, range: (usize, usize)) -> Case<T> {
    self.range = range;
    self
  }

  pub fn step_size(mut self, step_size: usize) -> Case<T> {
    self.step_size = step_size;
    self
  }

  pub fn set_color(mut self, color: RGBColor) -> Case<T> {
    self.color = Some(color);
    self
  }
}