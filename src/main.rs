extern crate nalgebra as na; //行列とかベクトルとかのライブラリ

fn main(){
    // 多分DMatrixのほうが使いやすい? staticは定数な印象
    let mat = na::DMatrix::from_iterator(3,3,[
        1.,2.,3.,
        4.,5.,6.,
        7.,8.,9.
            ].iter().cloned());

    let mat2 = na::DMatrix::from_iterator(3,3,[
        1.,3.,5.,
        2.,3.,8.,
        2.,1.,5.
    ].iter().cloned());
    let v = na::Vector3::new(1, 2, 3);
    let dm1 = na::DMatrix::from_diagonal_element(3, 3, 2.0);
    let dm2 = na::DMatrix::from_diagonal_element(3, 3, 2.0);
    println!("- - - mat - - -\n{}",mat); // DMatrixだと表示がいい感じになる
    // println!("- - - inv(mat) - - -\n{}",mat.try_inverse().unwrap()); // panicked at 'called `Option::unwrap()` on a `None` value',
    println!("- - - mat2 - - -\n{}",mat2);
    println!("- - - inv(mat2) - - - \n{}",mat2.try_inverse().unwrap()); //inverce matrix Optionalがついてるのでunwrap()で取る.

    // http://www.geisya.or.jp/~mwm48961/linear_algebra/simul_eq1.htm の 例1 を解く

    println!("========================= solve equations !!! =========================");

    let a = na::DMatrix::from_iterator(3,3,[
        1.,1.,0.,
        2.,-1.,1.,
        3.,2.,1.
    ].iter().cloned());
    println!("A\n{}",a);

    let b = na::DVector::from_iterator(3,[
        0.,3.,-1.
    ].iter().cloned());
    println!("B\n{}",b);

    let invA = a.try_inverse().unwrap();
    println!("invA\n{}",invA);

    println!("x\n{}", invA*b)
}

