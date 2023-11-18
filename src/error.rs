use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum Error {
    /// Raised when alpha outside allowed range [0,2)
    #[error("alpha ({}) outside allowed range [0,2)", alpha)]
    AlphaError { alpha: f64},

    /// Raised when alpha outside allowed range [-1,1]
    #[error("beta ({}) outside allowed range [-1,1]", beta)]
    BetaError { beta: f64},

    /// Raised by pdf function when initial values of bisection do not bracket a root
    #[error("bisection range ({},{}) does not bracket a root", a, b)]
    BisectionRangeError { a: f64, b: f64},

    /// Raised by pdf function when bisecction fails to find root with required tolerance in specified number of iterations
    #[error("root not found. Exceeded iteration limit of {}", n_max)]
    BisectionIterationsExceededError { n_max: u64 },

    /// Raised by GkQuad library used in pdf function
    #[error("integration error")]
    GkQuad {
        #[from]
        source: gkquad::RuntimeError,
    }
}