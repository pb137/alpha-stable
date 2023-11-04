use gkquad::{single::Integrator as GKQIntegrator, Tolerance};

use crate::bisect::bisect;
use crate::error::Error;

/// Integrator:
/// - eps_quad - defines convergence tolerance for Gauss-Konrod integrator.
/// - eps_bisect - defines convergence tolerance for bisection routine used to find peak of integrand.
/// - limit_bisect - maximum number of bisection iterations
/// - continue_on_err - if set to true, integration will not error even if limit_bisect is exceeded or integral does not converge. This code is marked as 'unsafe'.
#[derive(Debug)]
pub struct Integrator {
    eps_quad: f64,
    eps_bisect: f64,
    limit_bisect: u64,
    continue_on_err: bool,
}

impl Integrator {

    pub fn new(eps_quad: f64, eps_bisect: f64, limit_bisect: u64, continue_on_err: bool) -> Self {
        Integrator { eps_quad, eps_bisect, limit_bisect, continue_on_err }
    }
}

impl Default for Integrator {
    fn default() -> Self { 
        Integrator { eps_quad: 1e-10, eps_bisect: 1e-10, limit_bisect: 50, continue_on_err: false }
    }
}

impl Integrator {
    pub(crate) fn integrate(&self, f: &dyn Fn(f64) -> f64, g: &dyn Fn(f64) -> f64, a: f64, b:f64) -> Result<f64, Error>  {

        let mut integrator = GKQIntegrator::new(g).tolerance(Tolerance::Relative(self.eps_quad));

        if self.continue_on_err {
            unsafe {
                let max = bisect(&f, a, b, self.eps_bisect, self.limit_bisect).estimate_unchecked();
                let i_1 = integrator.run(a..max).estimate_unchecked();
                let i_2 = integrator.run(max..b).estimate_unchecked();    
                return Ok(i_1 + i_2);
            }
        } else {
            let max = bisect(&f, a, b, self.eps_bisect, self.limit_bisect).estimate()?;
            match (integrator.run(a..max).estimate(), integrator.run(max..b).estimate()) {
                (Ok(i_1), Ok(i_2)) => {
                    return Ok(i_1 + i_2);
                },
                (Ok(_), Err(e)) => {
                    return Err(Error::GkQuad { source: e });
                },
                (Err(e), Ok(_)) => {
                    return Err(Error::GkQuad { source: e });
                },
                (Err(e), Err(_)) => {
                    return Err(Error::GkQuad { source: e });
                }
            }    
        }
    }
}
