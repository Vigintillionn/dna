# Datastrucures and Algorithms
This repo contains the code used for my tasks for a class I'm taking, datastructures and algorithms. It is used to plot the amount of comparissons or another metric between specific algorithms. In the table below we use [tilde notation](https://www.geeksforgeeks.org/difference-between-big-o-notations-and-tilde/) to determine the complexity of each algorithm.

## Overview of sorting algorithms and their cases
|           | Best Case                     | Average Case          | Worst Case            | In Place           |
| --------- | ----------------------------- | --------------------- | --------------------- | ------------------ |
| Selection | $\sim \dfrac{n^2}{2}$         | $\sim \dfrac{n^2}{2}$ | $\sim \dfrac{n^2}{2}$ | :heavy_check_mark: | 
| Insertion | $\sim n$                      | $\sim \dfrac{n^2}{4}$ | $\sim \dfrac{n^2}{2}$ | :heavy_check_mark: |
| Merge     | $\sim \dfrac{1}{2}n\log_2(n)$ | $\sim cn\log_2(n)$    | $\sim n\log_2(n)$     | :x:                |
| Quick     | $\sim n\log_2(n)$             | $\sim 1.39n\log_2(n)$ | $\sim \dfrac{n^2}{2}$ | :heavy_check_mark: |

## If you wish to use this project
Make sure you have rust installed. Clone this repo using `git clone git@github.com:Vigintillionn/dna.git` after which each algorithm can be ran by running it's binary like so: `cargo run --bin <binary>`. If you wish to run the `mergesort` algorithm for example you can run `cargo run --bin mergesort`. A couple cases will be ran on the algorithm and some plots should appear in the `./out` directory. If you wish to run all algorithms at once, the `main` binary can be run using `cargo run --bin main`. Every algorithm will be ran concurrently (this could take a couple minutes).

Below is an example of what one of these plots looks like, in this case the plot for the balance of a red-black tree: ![balance of a red-black tree plot](https://github.com/Vigintillionn/dna/blob/rewrite/out/balance-2-black-red.png?raw=true)