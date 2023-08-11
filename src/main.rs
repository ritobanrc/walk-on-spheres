pub mod math {
    pub use crate::vec_ext::VecExtPartialOrd;
    use nalgebra as na;

    pub const DIM: usize = 2;

    pub type Dim = na::Const<DIM>;

    pub type Real = f64;
    pub type Vector = na::SVector<Real, DIM>;
    //pub type IntVector = na::SVector<isize, DIM>;
    pub type UIntVector = na::SVector<usize, DIM>;

    pub use std::f64::consts;

    //pub type Mat = na::SMatrix<Real, DIM, DIM>;
}

mod vec_ext;
mod window;

use crate::math::*;
use eyre::Result;
use ndarray::Array2;
use rand::random;
use std::ops::Range;

trait Levelset: Sync {
    fn phi(&self, x: Vector) -> Real;
}

trait DirichletBoundary: Sync {
    fn value(&self, x: Vector) -> Real;
}

struct WalkOnSpheres<L, B> {
    boundary_sdf: L,
    boundary_values: B,
    resolution: UIntVector,
    domain: Range<Vector>,
    eps: Real,
    max_iters: usize,
    solution: Array2<Real>,
    total_samples: usize,
}

impl<L: Levelset, B: DirichletBoundary> WalkOnSpheres<L, B> {
    fn new(
        boundary_sdf: L,
        boundary_values: B,
        domain: Range<Vector>,
        resolution: UIntVector,
        eps: Real,
        max_iters: usize,
    ) -> Self {
        WalkOnSpheres {
            boundary_sdf,
            boundary_values,
            resolution,
            domain,
            eps,
            max_iters,
            solution: Array2::zeros([resolution.x, resolution.y]),
            total_samples: 0,
        }
    }

    /// Performs one step of the walk on spheres algorithm
    /// Note that boundary_sdf _must_ actually be a signed distance function for the algorithm to work
    /// (otherwise, the steps taken will not remain in the domain).
    fn walk_on_spheres_step(&self, start_pos: Vector) -> Result<Real> {
        let mut pos = start_pos;
        for _ in 0..self.max_iters {
            let radius = self.boundary_sdf.phi(pos).abs();
            if radius < self.eps {
                return Ok(self.boundary_values.value(pos));
            }

            let angle = random::<Real>() * consts::TAU; // angle in [0, 2pi)
            let offset = radius * Vector::new(angle.cos(), angle.sin());
            pos = pos + offset;
        }

        Err(eyre::eyre!(
            "WoS: Exceeded max iters without reaching boundary from pos {:?}",
            start_pos
        ))
    }

    fn update_laplace_solution(&mut self, samples: usize) -> Array2<Real> {
        let domain_size = self.domain.end - self.domain.start;
        let dx = domain_size.x / self.resolution.x as Real;
        let dy = domain_size.y / self.resolution.y as Real;

        self.total_samples += samples;

        let new_solution = ndarray::Zip::indexed(&self.solution).par_map_collect(|(i, j), val| {
            let pos_x = self.domain.start.x + (i as Real) * dx;
            let pos_y = self.domain.start.y + (j as Real) * dy;
            let pos = Vector::new(pos_x, pos_y);
            if self.boundary_sdf.phi(pos) > 0. {
                return 0.;
            }

            let mut val = *val;
            for _ in 0..samples {
                let sample = self.walk_on_spheres_step(pos);
                match sample {
                    Ok(sample) => val += sample,
                    Err(fail) => {
                        eprintln!("{:?}", fail);
                    }
                }
            }
            val
        });
        self.solution = new_solution;

        self.solution.map(|x| x / self.total_samples as Real)
    }
}

struct Circle {
    radius: Real,
}

impl Levelset for Circle {
    fn phi(&self, x: Vector) -> Real {
        x.norm() - self.radius
    }
}

struct Rect {
    size: Vector,
}

impl Levelset for Rect {
    fn phi(&self, x: Vector) -> Real {
        let d = x.abs() - self.size;
        d.component_max(&Vector::zeros()).magnitude() + f64::max(d.x, d.y).min(0.)
    }
}

struct Sinusoid {
    frequency: Real,
}

impl DirichletBoundary for Sinusoid {
    fn value(&self, pos: Vector) -> Real {
        (self.frequency * Real::atan2(pos.y, pos.x)).sin()
    }
}

fn main() {
    let boundary_sdf = Circle { radius: 1.5 };
    //let boundary_sdf = Rect {
    //size: Vector::new(1.5, 1.),
    //};
    let boundary_values = Sinusoid { frequency: 5. };
    let domain = Vector::from_element(-2.)..Vector::from_element(2.);
    let resolution = UIntVector::new(1080, 1080);

    let mut wos = WalkOnSpheres::new(boundary_sdf, boundary_values, domain, resolution, 0.05, 500);

    let window = window::RenderWindow::new(
        "Walk On Spheres",
        minifb::WindowOptions::default(),
        resolution.x,
        resolution.y,
    );
    let grad = colorgrad::spectral();

    let samples_per_frame = 5;

    let mut last_frame_time = std::time::Instant::now();
    window.display(|| {
        let now = std::time::Instant::now();
        let delta = now - last_frame_time;
        if wos.total_samples % 10 == 0 {
            println!(
                "FPS: {:?}, Samples: {:?}",
                1000. / delta.as_millis() as f32,
                wos.total_samples + samples_per_frame
            );
        }
        last_frame_time = now;

        wos.update_laplace_solution(samples_per_frame).map(|&x| {
            let x = x / 2. + 0.5;
            window::Color(grad.at(x).to_rgba8())
        })
    });
}
