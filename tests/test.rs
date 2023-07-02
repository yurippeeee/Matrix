
use Matrix::NdMatrix;

// 加算演算子のテスト
#[test]
fn test_add_operator() {
    let test = NdMatrix::NDMATRIX([3,3,3,3,3]);
    let test2 = NdMatrix::NDMATRIX([5,5,5,5,5]);
    assert!((test+test2).0.iter().zip([8,8,8,8,8].iter()).all(|(a,b)| a == b), "Arrays are not equal");
}
