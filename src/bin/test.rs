use computer_graphis::user_type::determinant;

fn main() {
    let _vec = Vec::from([1., 2., 3., 4., 5., 6.]);
    let _vec2 = Vec::from([1., 1., 5., 7., 0., 3.]);

    let _d1 = determinant::Determinant::from_vec(2, 3, false, _vec).unwrap();
    let _d2 = determinant::Determinant::from_vec(3, 2, true, _vec2).unwrap();

    if let Some(_determinant) =  _d1 + _d2{
        _determinant.debug();
        _determinant.t().debug();
    }
    else {
        println!("failed");
    }
    println!("Hello, world!");
}
