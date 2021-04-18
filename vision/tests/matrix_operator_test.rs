
use opencv::core::*;

#[test]
fn mul_matexpr_example() {
    let a_rmex = Mat::eye(4, 4, f64::typ());
    let b_rmex = Mat::eye(4, 4, f64::typ());

    let _c_rmex = a_rmex.and_then(|a_mex| {
        b_rmex.and_then(|b_mex| {
            a_mex.mul_matexpr(&b_mex, 1.)
        })
    });
}


#[test]
fn add_matexpr_example() {
    let a_rmex = Mat::eye(4, 4, f64::typ());
    let b_rmex = Mat::eye(4, 4, f64::typ());


    let _c_rmex = a_rmex.and_then(|a_mex| {
        b_rmex.and_then(|b_mex| {
            add_matexpr_matexpr(&a_mex, &b_mex)
        })
    });
}

#[test]
fn add_matexpr_example2() {
    let a_rmex = Mat::eye(4, 4, f64::typ());
    let b_rmex = Mat::eye(4, 4, f64::typ());

    let _c_rmex = match (a_rmex, b_rmex) {
        (Ok(a_mex), Ok(b_mex)) => add_matexpr_matexpr(&a_mex, &b_mex),
        _ => panic!("invalid operation")
    };
}
