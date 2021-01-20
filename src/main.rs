extern crate argmin;
use argmin::prelude::*;
use argmin::solver::linesearch::HagerZhangLineSearch;

struct cost_function {}

// f(x)=(x−3) x^{3} (x−6)^{4}
pub fn cf(x: f64) -> f64 {
    let value: f64 = (x - 3 as f64) * x.powi(3) * (x - 6 as f64).powi(4);
    value
}

// df(x) = (x - 6)**5 *
// (x**3*(x - 6)*(x - 3)**(x**4)*(x + 4*(x - 3)*log(x - 3)) + 6*(x - 3)**(x**4 + 1)) /
// (x - 3)
pub fn cf_deriv(x: f64) -> f64 {
    let value: f64 = (x - 6 as f64).powi(5)
        * (x.powi(3)
            * (x - 6 as f64)
            * (x - 3 as f64).powf(x.powi(4))
            * (x + 4 as f64 * (x - 3 as f64) * (x - 3 as f64).ln())
            + 6 as f64 * (x - 3 as f64).powf(x.powi(4) + 1 as f64))
        / (x - 3 as f64);
    value
}

impl ArgminOp for cost_function {
    type Param = f64;
    type Output = f64;
    type Float = f64;
    type Jacobian = ();
    type Hessian = ();

    fn apply(&self, param: &Self::Param) -> Result<Self::Output, Error> {
        Ok(cf(*param))
    }

    fn gradient(&self, param: &Self::Param) -> Result<Self::Param, Error> {
        Ok(cf_deriv(*param))
    }
}

fn main() {}
