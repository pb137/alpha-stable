use crate::error::Error;
use crate::numerical_result::NumericalResult;

pub(crate) fn bisect(f: &dyn Fn(f64) -> f64, mut a: f64, mut b:f64, eps: f64, n_max: u64) -> NumericalResult<Error> {

    if a == b {
        return NumericalResult::new(0.0,0.0, Some(Error::BisectionRangeError { a, b }));    
    }

    if a > b {
        (a, b) = (b, a);
    }

    let mut x = 0.0;
    let mut fa = f(a);
    let fb = f(b);

    if fa.signum() == fb.signum() {
        return NumericalResult::new(0.0,0.0, Some(Error::BisectionRangeError { a, b }));    
    }

    for _ in 0..n_max {
        
        x = 0.5 * (a + b);
        let fx = f(x);

        if fx == 0.0 || 0.5*(b-a) < eps {
            return NumericalResult::new(x,0.5*(b-a), None);  
        }

        if fx.signum() == fa.signum() {
            a = x;
            fa = fx;            
        } else {
            b = x;
        }
    }
    return NumericalResult::new(x,0.5*(b-a), Some(Error::BisectionIterationsExceededError { n_max }));    
}

#[cfg(test)]
mod tests {
    use super::bisect;
    use crate::alpha_stable::close;

    #[test]
    fn test_linear() {
        let result = bisect(&|x| { x - 3.0 }, 1.0, 5.0, 1e-10, 1);

        match result.estimate() {
            Ok(estimate) => {
                assert!(close(estimate, 3.0 , 1e-6));
            },
            Err(_) => assert!(false),
        }
        assert!(!result.has_err());
    }

    #[test]
    fn test_quadratic() {
        let result = bisect(&|x| { x*x - 6.0 }, 1.0, 5.0, 1e-6, 30);

        match result.estimate() {
            Ok(estimate) => {
                assert!(close(estimate, 6.0_f64.sqrt() , 1e-6));
            },
            Err(_) => assert!(false),
        }
        assert!(!result.has_err());
    }
}
