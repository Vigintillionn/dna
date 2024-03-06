# Datastrucures and Algorithms
This repo contains the code used for my tasks for a class I'm taking, datastructures and algorithms. It is used to plot the amount of comparissons between specific algorithms.

## Overview of algorithms and their cases
|           | Best Case                     | Average Case          | Worst Case            | In Place           |
| --------- | ----------------------------- | --------------------- | --------------------- | ------------------ |
| Selection | $\sim \dfrac{n^2}{2}$         | $\sim \dfrac{n^2}{2}$ | $\sim \dfrac{n^2}{2}$ | :heavy_check_mark: | 
| Insertion | $\sim n$                      | $\sim \dfrac{n^2}{4}$ | $\sim \dfrac{n^2}{2}$ | :heavy_check_mark: |
| Merge     | $\sim \dfrac{1}{2}n\log_2(n)$ | $\sim cn\log_2(n)$    | $\sim n\log_2(n)$     | :x:                |
| Quick     | $\sim n\log_2(n)$             | $\sim 1.39n\log_2(n)$ | $\sim \dfrac{n^2}{2}$ | :heavy_check_mark: |