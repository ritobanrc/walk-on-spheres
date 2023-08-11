use crate::boundary_conditions::DirichletBoundary;
use crate::domain::Levelset;
use crate::math::*;
use eyre::Result;
use ndarray::Array2;
use rand::random;
use std::ops::Range;

pub struct WalkOnSpheres<L, B> {
    boundary_sdf: L,
    boundary_values: B,
    resolution: UIntVector,
    domain: Range<Vector>,
    eps: Real,
    max_iters: usize,
    solution: Array2<Real>,
    pub total_samples: usize,
}

impl<L: Levelset, B: DirichletBoundary> WalkOnSpheres<L, B> {
    pub fn new(
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

    pub fn update_laplace_solution(&mut self, samples: usize) -> Array2<Real> {
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
