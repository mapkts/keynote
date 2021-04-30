# 浮点除法运算有必要手动优化吗？ 

> 在计算密集型的应用中，手动优化除法是有一定必要的，因为编译器可能不如你想象的那么聪明。

最近在跟着PBRT这本书写一个离线渲染器，在光线和实体对象（如球体）求交时，需要频繁将一个向量`vector`除以其长度`magnitude`得出单位向量`unit vector`。大家都知道在现代CPU中除法运算一般都比乘法运算慢很多，PBRT这本书上也推荐在将一个向量除以一个标量`scalar`时，先求出被除标量的倒数`recipocal`，然后再使该向量的每一元素都乘以这个倒数, 得出单位向量。其实现代编译器，特别是像Rustc这种用llvm做后端的编译器，是特别擅长优化用户代码的, 完全有可能会自动帮我们优化除法运算。为避免不必要的过度优化`premature optimization`，我们可以写两个测试，一个是手动优化版本的，一个是未优化版本的，通过测试两者的性能表现，我们可以确定编译器是优化除法运算。

## 性能测试

首先我们先将向量和向量和标量的除法运算定义好，这里我们选择三维向量用于测试。

```rust
#[derive(Copy, Clone)]
pub struct V1(pub f32, pub f32, pub f32);

// 手动优化的实现
impl core::ops::Div<f32> for V1 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        let inv = 1. / rhs;
        Self(self.0 * inv, self.1 * inv, self.2 * inv)
    }
}

#[derive(Copy, Clone)]
pub struct V2(pub f32, pub f32, pub f32);

// 未优化的实现
impl core::ops::Div<f32> for V2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
```

下面给出的是测试用例，为简化代码我们直接使用Rust官方的bench模块。

```rust
#![feature(test)]

extern crate test;

#[cfg(test)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn test_cached_div(bencher: &mut Bencher) {
       let mut x = V1(f32::MAX, f32::MAX, f32::MAX);
       bencher.iter(|| {
           for _ in 0..10000 {
               x = x / 0.99;
           }
       });
    }

    #[bench]
    fn test_direct_div(bencher: &mut Bencher) {
       let mut x = V2(f32::MAX, f32::MAX, f32::MAX);
       bencher.iter(|| {
           for _ in 0..10000 {
               x = x / 0.99;
           }
       });
    }
}
```

在我的机器上基准测试结果如下：

```
test bench::test_cached_div ... bench:      10,293 ns/iter (+/- 66)
test bench::test_direct_div ... bench:      31,562 ns/iter (+/- 263)
```

在循环一万次的情形下，手动优化的除法（将三个除法替换成一个除法和三个乘法）所需的时间是直接进行的除法（直接进行三次除法）所需时间的1 / 3，说明编译器是不会进行相应优化的。

这时候你可能会问，为什么?
