// this stuff is too difficult to implement for me.
//
// For reference please refer this article: https://manishearth.github.io/blog/2017/01/11/rust-tidbits-what-is-a-lang-item/

trait MathOps {
    type OtherVar;
    type Output;
    fn add(&self, other: Self::OtherVar) -> Self::Output;
}

#[derive(Debug)]
struct Integer {
    num: i32,
}

impl MathOps for Integer {
    type OtherVar = Integer;
    type Output = i32;

    fn add(&self, other: Self::OtherVar) -> Self::Output {
        self.num + other.num
    }
}

#[derive(Debug)]
struct Float {
    num: f64,
}

impl MathOps for Float {
    type OtherVar = Float;
    type Output = f64;

    fn add(&self, other: Self::OtherVar) -> Self::Output {
        self.num + other.num
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_int_ypes() {
        let x0 = Integer { num: 1 };
        let x1 = Integer { num: 1 };

        //let act_ans = x0 + x1;
        let act_ans = x0.add(x1);
        let exp_ans = 2;

        assert_eq!(exp_ans, act_ans);
    }

    #[test]
    fn same_float_ypes() {
        let y0 = Float { num: 1.0 };
        let y1 = Float { num: 1.0 };

        //let act_ans = y0 + y1;
        let act_ans = y0.add(y1);
        let exp_ans = 2.0;

        assert_eq!(exp_ans, act_ans);
    }

    // #[test]
    // fn diff_types() {
    //     let x0 = Integer { num: 1 };
    //     let y0 = Float { num: 1.0 };

    //     let act_ans = x0 + y0;
    //     let exp_ans = 1.0;

    //     assert_eq!(exp_ans, act_ans);
    // }
}
