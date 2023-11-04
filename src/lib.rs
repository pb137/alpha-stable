//! Alpha Stable
//! 
//! Library to sample from, and generate Probability Distributions of, Alpha Stable distributions.
//! 
//! Implementation uses approach described in: Stable Distributions, Szymon Barak, Wolfgang Hardle and Rafal Weron available at: <http://hdl.handle.net/10419/25027>
//! 
//! Distributions are specified using one of two forms:
//! - Standard form: S_alpha(sigma, beta, mu) - equivalent to the 'first parameterization' of Wikipedia (<https://en.wikipedia.org/wiki/Stable_distribution>) with c = sigma.
//! - Nolan's form: S^0_alpha(sigma, beta, mu_0) - equivalent to the 'second parameterization' in Wikipedia with delta = mu_0 and gamma = sigma.
pub mod alpha_stable;
pub mod error;
pub mod integrator;
mod bisect;
mod numerical_result;

pub use alpha_stable::{AlphaStable, Tol};
pub use integrator::Integrator;