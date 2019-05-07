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

trait Grid : Clone + Send + Sync {
    fn new(size: usize) -> Self;
    fn get(&self, x: usize, y: usize) -> Option<State>;
    fn set(&mut self, x: usize, y: usize, v: State);
    fn size(&self) -> usize;
}

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

struct FlatVecGrid {
    data: Vec<State>,
    size: usize,
}

impl Clone for FlatVecGrid {
    fn clone(&self) -> Self {
        FlatVecGrid {
            data: self.data.clone(),
            size: self.size
        }
    }
}

impl Grid for FlatVecGrid {
    fn new(size: usize) -> FlatVecGrid {
        FlatVecGrid {
            data: vec![State::Off; size * size],
            size,
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

    fn size(&self) -> usize {
        self.size
    }
}

struct VecGrid {
    data: Vec<Vec<State>>,
    size: usize,
}

impl Clone for VecGrid {
    fn clone(&self) -> Self {
        VecGrid {
            data: self.data.clone(),
            size: self.size
        }
    }
}


impl Grid for VecGrid {
    fn new(size: usize) -> VecGrid {
        VecGrid {
            data: vec![vec![State::Off; size]; size],
            size,
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<State> {
        if x > self.size || y > self.size {
            None
        } else {
            Some(self.data[x][y])
        }
    }

    fn set(&mut self, x: usize, y: usize, v: State) {
        self.data[x][y] = v;
    }

    fn size(&self) -> usize {
        self.size
    }
}

struct HashMapGrid {
    data: HashMap<(usize, usize), State>,
    size: usize,
}

impl Clone for HashMapGrid {
    fn clone(&self) -> Self {
        HashMapGrid {
            data: self.data.clone(),
            size: self.size
        }
    }
}

impl Grid for HashMapGrid {
    fn new(size: usize) -> HashMapGrid {
        let mut grid = HashMapGrid {
            data: HashMap::with_capacity(size * size),
            size,
        };

        (0..size).into_iter()
            .zip(0..size)
            .for_each(|k| { grid.data.insert(k, State::Off); });

        grid
    }

    fn get(&self, x: usize, y: usize) -> Option<State> {
        let k = (x, y);
        match self.data.get(&k) {
            None => None,
            Some(s) => Some(*s),
        }
    }

    fn set(&mut self, x: usize, y: usize, v: State) {
        self.data.insert((x, y), v);
    }

    fn size(&self) -> usize {
        self.size
    }
}

fn neighbors<T: Grid>(grid: &T, x: usize, y: usize) -> u8 {
    (x-1..x+1).into_iter()
        .zip(y-1..y+1)
        .map(|(x, y)| grid.get(x, y))
        .filter(|o| o.is_some())
        .count() as u8
}

fn process(state: State, n: u8) -> State {
    match state {
        State::On => {
            if n < 2 {
                State::Off
            } else if n > 3 {
                State::Off
            } else {
                State::On
            }
        },
        State::Off => {
            if n == 3 {
                State::On
            } else {
                State::Off
            }
        }
    }
}

fn sim<T: Grid>(mut grid: T, iterations: usize) {
    setup(&mut grid);

    for _ in 0..iterations {

        //let mut out = grid.clone();

        let res: Vec<(usize, usize, State)> = (0..grid.size()).into_par_iter()
            .zip(0..grid.size())
            .map(|(x, y)| (x, y, process(grid.get(x, y).unwrap(), neighbors(&grid, x, y))))
            .collect::<Vec<_>>();

        //for (x, y, s) in res {
        //    out.set(x, y, s);
        //}

        //grid = out;
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("vec", |b| b.iter(|| sim(VecGrid::new(4096), black_box(1))));
    c.bench_function("flatvec", |b| b.iter(|| sim(FlatVecGrid::new(4096), black_box(1))));
    //c.bench_function("hashmap", |b| b.iter(|| sim(HashMapGrid::new(1024), black_box(1))));
}

// implement memcpy for flat grid

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
