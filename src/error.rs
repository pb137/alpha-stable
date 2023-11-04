use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum Error {
    #[error("alpha ({}) outside allowed range [0,2)", alpha)]
    AlphaError { alpha: f64},

    #[error("beta ({}) outside allowed range [-1,1]", beta)]
    BetaError { beta: f64},

    #[error("bisection range ({},{}) does not bracket a root", a, b)]
    BisectionRangeError { a: f64, b: f64},

    #[error("root not found. Exceeded iteration limit of {}", n_max)]
    BisectionIterationsExceededError { n_max: u64 },

    #[error("integration error")]
    GkQuad {
        #[from]
        source: gkquad::RuntimeError,
    }
}