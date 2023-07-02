pub mod NdMatrix{

    use std::ops::Add;


    #[derive(Debug)]
    //  行列計算のベースとなる型
    pub struct NDMATRIX<T, const N: usize>([T; N]) where T:Add+Copy;

    impl<T, const N: usize> NDMATRIX<T,N>
        where T:Add + Copy{
            // 新規行列作成
            pub fn new(m:&[T; N]) -> Self{
                Self(*m)
            }
            // 配列を取得する
            pub fn to_array(self) -> [T; N]{
                self.0
            }
        }

    // ＋演算子実装
    impl<T, const N: usize> Add for NDMATRIX<T,N> 
        where T:Add<Output = T> + Copy{
            type Output = Self;
            fn add(self, other:Self) -> Self::Output{
                let mut output = self.0;
                for i in 0..self.0.len(){
                    output[i] = self.0[i] + other.0[i];
                }
                NDMATRIX(output)
            }
        }
}

