#[derive(Debug)]
pub(crate) struct NumericalResult<Error> 
where
    Error: Copy
{
    estimate: f64,
    delta: f64,
    error: Option<Error>,
}

impl<Error> NumericalResult<Error> 
where 
    Error: Copy
{
    pub(crate) fn new(estimate: f64, delta: f64, error: Option<Error>) -> Self {
        NumericalResult { estimate, delta, error }
    }

    #[allow(dead_code)]
    pub(crate) fn has_err(&self) -> bool {
        !self.error.is_none() 
    }

    pub(crate) fn estimate(&self) -> Result<f64, Error> {
        match self.error {
            Some(error) => Err(error),
            None => Ok(self.estimate),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn delta(&self) -> Result<f64, Error> {
        match self.error {
            Some(error) => Err(error),
            None => Ok(self.delta),
        }
    }

    pub(crate) unsafe fn estimate_unchecked(&self) -> f64 {
        self.estimate
    }

    #[allow(dead_code)]
    pub(crate) unsafe fn delta_unchecked(&self) -> f64 {
        self.delta
    }

}