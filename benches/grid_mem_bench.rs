#![feature(existential_type)]
use std::default::Default;
use std::option::Option;

#[macro_use]
extern crate criterion;
use criterion::black_box;
use criterion::Criterion;

extern crate hashbrown;
use hashbrown::HashMap;

extern crate rayon;
use rayon::prelude::*;

#[derive(Copy, Clone)]
enum State {
    On,
    Off,
}

/*
fn place_glider<T: Grid>(grid: &mut T, x: usize, y: usize) {
    grid.set(x, y, State::On);
    grid.set(x + 1, y, State::On);
    grid.set(x + 2, y, State::On);
    grid.set(x + 2, y + 1, State::On);
    grid.set(x + 1, y + 2, State::On);
}

fn setup<T: Grid>(grid: &mut T) {
    let n = grid.size() / 3;
    for i in (0..n).step_by(3) {
        for j in (0..n).step_by(3) {
            place_glider(grid, i, j);
        }
    }
}
*/

struct Grid {
    data: Vec<State>,
    size: usize,
}

impl Grid {
    fn new(size: usize) -> Self {
        Grid {
            data: vec![State::Off; size * size],
            size
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<State> {
        if x > self.size || y > self.size {
            None
        } else {
            Some(self.data[x + y * self.size])
        }
    }

    fn set(&mut self, x: usize, y: usize, v: State) {
        self.data[x + y * self.size] = v;
    }
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Grid {
            data: self.data.clone(),
            size: self.size
        }
    }
}

fn clone_grid(grid: Grid) -> Grid {
    let new = grid.clone();
    new
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function_over_inputs("clone", |b, &size| b.iter(|| clone_grid(black_box(Grid::new(size)))), (8..=16).map(|v| 2usize.pow(v)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
