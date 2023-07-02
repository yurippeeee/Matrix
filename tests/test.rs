
use Matrix::NdMatrix;

// 加算演算子のテスト
#[test]
fn test_add_operator() {
    let test = NdMatrix::NDMATRIX::new(&[3,3,3,3,3]);
    let test2 = NdMatrix::NDMATRIX::new(&[5,5,5,5,5]);
    assert!((test+test2).to_array().iter().zip([8,8,8,8,8].iter()).all(|(a,b)| a == b), "Arrays are not equal");
}
