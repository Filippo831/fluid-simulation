use crate::{SIZE, ITER};

fn IX(x: u32, y: u32) -> usize {
    (x + y * SIZE) as usize
}

pub struct Fluid {
    SIZE: u32,
    dt: f32,
    diff: f32,
    visc: f32,

    s: Vec<f32>,
    density: Vec<f32>,

    vx: Vec<f32>,
    vy: Vec<f32>,

    vx0: Vec<f32>,
    vy0: Vec<f32>,
}

impl Fluid {
    pub fn new(dt: f32, diff: f32, visc: f32) -> Fluid {
        Fluid {
            SIZE,
            dt,
            diff,
            visc,
            s: Vec::with_capacity((SIZE * SIZE) as usize),
            density: Vec::with_capacity((SIZE * SIZE) as usize),
            vx: Vec::with_capacity((SIZE * SIZE) as usize),
            vy: Vec::with_capacity((SIZE * SIZE) as usize),
            vx0: Vec::with_capacity((SIZE * SIZE) as usize),
            vy0: Vec::with_capacity((SIZE * SIZE) as usize),
        }
    }

    pub fn addDensity(&mut self, x: u32, y: u32, amount: f32) {
        self.density[IX(x, y)] += amount;
    }

    pub fn addVelocity(&mut self, x: u32, y: u32, amount_x: f32, amount_y: f32) {
        self.vx[IX(x, y)] += amount_x;
        self.vy[IX(x, y)] += amount_y;
    }
}

fn set_bnd(b: u8, x: &mut Vec<f32>) {
    for k in 1..SIZE - 1 {
        if b == 3 {
            x[IX(k, 0)] = -x[IX(k, 1)];
            x[IX(k, SIZE - 1)] = -x[IX(k, SIZE - 2)];
        }
    }
    for j in 1..SIZE - 1 {
        if b == 3 {
            x[IX(0, j)] = -x[IX(1, j)];
            x[IX(SIZE - 1, j)] = -x[IX(SIZE - 2, j)];
        }
    }
    x[IX(0, 0)] = 0.5 * (x[IX(1, 0)] + x[IX(0, 1)]);
    x[IX(0, SIZE - 1)] = 0.5 * (x[IX(0, SIZE - 2)] + x[IX(1, SIZE - 1)]);
    x[IX(SIZE - 1, SIZE - 1)] = 0.5 * (x[IX(SIZE - 1, SIZE - 2)] + x[IX(SIZE - 2, SIZE - 1)]);
    x[IX(SIZE - 1, 0)] = 0.5 * (x[IX(SIZE - 2, 0)] + x[IX(SIZE - 1, 1)]);
}

fn lin_solve(b: u8, x: &mut Vec<f32>, a: f32, c: f32){}
