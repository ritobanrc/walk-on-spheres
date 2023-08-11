mod boundary_conditions;
mod domain;
#[allow(dead_code)]
mod math;
mod walk_on_spheres;
mod window;

use crate::math::*;
use boundary_conditions::Sinusoid;
use domain::Rect;
use walk_on_spheres::WalkOnSpheres;

fn main() {
    //let boundary_sdf = Circle { radius: 1.5 };
    let boundary_sdf = Rect {
        size: Vector::new(1.5, 1.),
    };
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
