use computer_graphis::user_type::determinant;

fn main() {
    let _vec = Vec::from([1., 2., 3., 4., 5., 6.]);
    if let Some(_determinant) = determinant::Determinant::from_vec(2, 3, false, _vec) {
        _determinant.debug();
        _determinant.t().debug();
    }
    println!("Hello, world!");
}
