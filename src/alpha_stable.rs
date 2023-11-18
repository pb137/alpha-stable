use std::f64::consts::PI;

use rand::Rng;
use rand_distr::{Exp1, Distribution};
use spfunc::gamma::gamma;

use crate::integrator::Integrator;
use crate::error::Error;

/// Defines an Alpha Stable distribution in Standard or Nolan's form.
#[derive(Debug)]
pub struct AlphaStable {
    alpha: f64,
    beta: f64,
    sigma: f64,
    mu: f64,
    mu_0: f64,
    tol: Tol,
    integrator: Integrator,
}

impl Default for AlphaStable {
    fn default() -> Self { 
        AlphaStable { alpha: 2.0, beta: 0.0, sigma: 0.0, mu: 0.0, mu_0: 0.0, tol: Tol::default(), integrator: Integrator::default() } 
    }
}

impl AlphaStable {

    /// Create distribution in standard form: S_alpha(sigma, beta, mu).
    pub fn new(alpha: f64, beta: f64, sigma: f64, mu: f64) -> Result<AlphaStable, Error> {
        
        if alpha <= 0.0 || alpha > 2.0 {
            return Err(Error::AlphaError {alpha});
        }

        if beta < -1.0 || beta > 1.0 {
            return Err(Error::BetaError {beta});    
        }

        let tol = Tol::default();

        let mu_0 = if close( alpha, 1.0, tol.alpha ) {
            mu + beta * sigma * 2.0 * sigma.ln() / PI
        } else {
            mu + beta * sigma * (0.5 * PI * alpha).tan()
        };

        Ok(AlphaStable { alpha, beta, sigma, mu, mu_0, tol, integrator: Integrator::default() })
    }

    /// Create distribution in Nolan's form: S^0_alpha(sigma, beta, mu_0)
    #[allow(non_snake_case)]
    pub fn new_S0(alpha: f64, beta: f64, sigma: f64, mu_0: f64) -> Result<AlphaStable, Error> {
        
        if alpha <= 0.0 || alpha > 2.0 {
            return Err(Error::AlphaError {alpha});
        }

        if beta < -1.0 || beta > 1.0 {
            return Err(Error::BetaError {beta});    
        }

        let tol = Tol::default();

        let mu = if close( alpha, 1.0, tol.alpha ) {
            mu_0 - beta * sigma * 2.0 * sigma.ln() / PI
        } else {
            mu_0 - beta * sigma * (0.5 * PI * alpha).tan()
        };   

        Ok(AlphaStable { alpha, beta, sigma, mu, mu_0, tol, integrator: Integrator::default() })
    }

    /// Set tolerances for testing if alpha, beta and zeta approach special values.
    pub fn with_tol(&mut self, tol: Tol) -> &mut Self {
        self.tol = tol;
        self
    }

    /// Set integration parameters.
    pub fn with_integrator(&mut self, integrator: Integrator) -> &mut Self {
        self.integrator = integrator;
        self
    }

    /// Return parameters as tuple of (alpha, beta, sigma, mu, mu_0).
    pub fn get_params(&self) -> (f64, f64, f64, f64, f64) { 
        (self.alpha, self.beta, self.sigma, self.mu, self.mu_0)
    }

    /// Sample from the distribution.
    /// 
    /// # Example
    /// 
    /// ```
    /// use rand::thread_rng;
    /// 
    /// let mut rng = thread_rng();
    /// let distribution = alpha_stable::AlphaStable::new( 1.5, 0.0, 1.0, 0.0).unwrap();
    /// let sample = distribution.sample(&mut rng);
    /// ```
    pub fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {

        if close( self.beta, 0.0, self.tol.beta ) {
            return self.sample_symmetric(rng);
        }

        let v = PI * (rng.gen::<f64>() - 0.5);
        
        let mut w = 0.0;
        while w == 0.0 {
            w = Exp1.sample(rng); 
        }

        if close( self.alpha, 1.0, self.tol.alpha ) {
            let x = ( (0.5 * PI  + self.beta * v) * v.tan() - 
                self.beta * ( (0.5 * PI * w * v.cos()) / (0.5 * PI + self.beta * v) ).ln()) / (0.5 * PI); 
            return self.sigma * x + self.beta * self.sigma * self.sigma.ln() / (0.5 * PI) + self.mu;
        }

        let t = self.beta * (0.5 * PI * self.alpha).tan();
        let s = (1.0 + t * t).powf( 0.5 / self.alpha );
        let b = t.atan() / self.alpha;
        let x = s * ( self.alpha * (v + b) ).sin() * 
                ( ( (v - self.alpha*(v + b)).cos() / w ).powf((1.0 - self.alpha) / self.alpha) )  / 
                ( v.cos().powf(1.0 / self.alpha) );
        return self.sigma * x + self.mu;
    }

    fn sample_symmetric<R: Rng + ?Sized>(&self, rng: &mut R) -> f64 {
        let v = PI * (rng.gen::<f64>() - 0.5);

        if close( self.alpha, 1.0, self.tol.alpha ) {
            return self.sigma * v.tan() + self.mu;
        }

        let mut w = 0.0_f64;
        while w == 0.0 {
            w = Exp1.sample(rng); 
        }

        if close( self.alpha, 2.0, self.tol.alpha ) {
            return 2.0 * v.sin() * w.sqrt() * self.sigma + self.mu;
        }

        let t = (self.alpha * v).sin() / v.cos().powf( 1.0 / self.alpha );
        let s = ( ((1.0 - self.alpha) * v).cos() / w ).powf( (1.0-self.alpha)/self.alpha );
        self.sigma * t * s + self.mu
    }

    /// Value of Probability Distribution function at x.
    /// 
    /// # Example
    /// 
    /// ```
    /// let distribution = alpha_stable::AlphaStable::new( 1.5, 0.0, 1.0, 0.0).unwrap();
    /// let val = distribution.pdf( 0.5 ).unwrap();
    /// ```
    pub fn pdf(&self, x: f64) -> Result<f64, Error> {
        let x = (x - self.mu_0) / self.sigma;
        let val = pdf_scaled(x, self.alpha, self.beta, &self.tol, &self.integrator)?;
        Ok(val/self.sigma)
    }                

}

// Calculates pdf by direct integration as described on page 7 of paper.
fn pdf_scaled(x: f64, alpha: f64, beta: f64, tol: &Tol, integrator: &Integrator) -> Result<f64, Error> {

    if close( alpha, 2.0, tol.alpha) {

        // Normal distribution
        return Ok((-0.25 * x * x).exp() / (4.0 * PI).sqrt());

    } else if close( alpha, 1.0, tol.alpha) && !close(beta, 0.0, tol.beta) {

        // alpha == 1, beta != 0
        let gamma = (-0.5 * PI * x / beta).exp();
        let a = -0.5 * PI;
        let b =  0.5 * PI;

        let val = integrator.integrate(
                &|theta| {
                    derivative_alpha_eq_1(theta, beta) * gamma - 1.0
                },
                &|theta| {
                    integrand_alpha_eq_1(theta, beta, gamma)
                },
                a, b,
        )?;

        return Ok(val * 0.5 * gamma / beta.abs());

    } else if close(alpha, 1.0, tol.alpha) && close(beta, 0.0, tol.beta) {
        
        // Cauchy distribution: alpha == 1, beta == 0
        return Ok(1.0 / ((1.0 + x * x) * PI));
    
    } else if !close(alpha, 1.0, tol.alpha) {

        // alpha != 1 cases
        let zeta = -beta * (0.5 * PI * alpha).tan();
        let eps = (-zeta).atan() / alpha;

        if close(x, zeta, tol.zeta) {

            // Special case x = zeta
            return Ok(gamma(1.0 + 1.0 / alpha) * eps.cos() / (PI * (1.0 + zeta * zeta).powf(0.5 / alpha)));
        
        } else if x > zeta {

            // x > zeta
            let gamma = (x - zeta).powf(alpha / (alpha - 1.0));
            let a = -eps;
            let b = 0.5 * PI;

            let val = integrator.integrate(
                &|theta| {
                    derivative_alpha_neq_1(theta, alpha, eps) * gamma - 1.0
                },
                &|theta| {
                    integrand_alpha_neq_1(theta, alpha, eps, gamma)
                },
                a, b,
            )?;

            return Ok(val * alpha * (x - zeta).powf(1.0/(alpha - 1.0)) / (PI * (alpha - 1.0).abs()));

        } else if x < zeta {
            // symmetric case
            return pdf_scaled(-x, alpha, -beta, tol, integrator);
        }
    }
    return Ok(0.0);
}

// Utility to test closeness
pub(crate) fn close( arg: f64, close_to: f64, with_tol: f64 ) -> bool {
    (arg-close_to).abs() <= with_tol.abs()
}

fn derivative_alpha_neq_1(theta: f64, alpha: f64, eps: f64) -> f64 {
    (alpha * eps).cos().powf(1.0/(alpha  - 1.0)) *
    (theta.cos() / (alpha * (theta + eps)).sin()).powf(alpha / (alpha - 1.0)) *
    (alpha * eps + (alpha - 1.0) * theta).cos() / theta.cos()
}

fn integrand_alpha_neq_1(theta: f64, alpha: f64, eps: f64, gamma: f64) -> f64 {
    let val = derivative_alpha_neq_1(theta, alpha, eps);
    if val.is_nan() || val.is_infinite() {
        return 0.0;
    }
    (val.ln() - val * gamma).exp()
}

fn derivative_alpha_eq_1(theta: f64, beta: f64) -> f64 {
    (1.0 + 2.0 * beta * theta / PI) * ( (0.5 * PI / beta + theta) * theta.tan() ).exp() / theta.cos()
}

fn integrand_alpha_eq_1(theta: f64, beta: f64, gamma: f64) -> f64 {
    let val = derivative_alpha_eq_1(theta, beta);
    if val.is_nan() || val.is_infinite() {
        return 0.0;
    }
    (val.ln() - val * gamma).exp()
}

/// Defines tolerances for testing if alpha, beta and zeta approach special values.
#[derive(Debug)]
pub struct Tol {
    alpha: f64,
    beta: f64,
    zeta: f64,
}

impl Tol {
    pub fn new( alpha: f64, beta: f64, zeta: f64) -> Self {
        Tol { alpha, beta, zeta }
    }
}

impl Default for Tol {
    fn default() -> Self { 
        Tol { alpha: 1e-6, beta: 1e-6, zeta: 1e-6 }
    }
}


