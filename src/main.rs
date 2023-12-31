use std::ops::Add;

#[derive(Debug)]

//  行列計算のベースとなる型
struct NDMATRIX<T, const N: usize>([T; N]) where T:Add+Copy;

// ＋演算子実装
impl<T, const N: usize> Add for NDMATRIX<T,N> where T:Add<Output = T> + Copy{
    type Output = Self;
    fn add(self, other:Self) -> Self::Output{
        let mut output = self.0;
        for i in 0..self.0.len(){
            output[i] = self.0[i] + other.0[i];
        }
        NDMATRIX(output)
    }
}

fn main() {
    let test = NDMATRIX([3,3,3,3,3]);
    let test2 = NDMATRIX([5,5,5,5,5]);
    println!("{:#?}",test + test2)
}
