# 浮点数除法优化

> 在计算密集型的应用中，优化除法运算是有一定必要的，因为编译器可能不如你想象的那么聪明

最近在跟着[`PBRT`]这本书写一个简单的离线物理渲染器，在对光线和实体对象（如球体）求交时，需要频繁地做除法运算将一个空间向量单位化。大家都知道，CPU的除法指令要比乘法指令慢得多，比如对AMD最新的Zen 3架构CPU（Ryzen 5000系列）的测试显示[^1]，一次x87浮点除法运算需要的时钟周期大约是乘法运算所需时钟周期的6倍（见下图）。

[`PBRT`]: https://www.pbr-book.org/3ed-2018/contents

| x87浮点指令 | 延迟 | 1 / 吞吐量 |
| :-:         | :-:  | :-:        |
| FMUL        | 6-7  | 1          |
| FDIV        | 15   | 6          |

在实现向量对标量的除法时，书上建议先求出被除标量的倒数，然后再将向量的每一元素都乘以这个倒数，来得出单位向量。问题来了，对于这种简单而直接的优化方式，现代编译器不应该会自动优化吗？这种手动优化是否有必要呢？

为避免不必要的过度优化，我们可以写两个基准测试对比手动优化版和无修改版的性能表现，来确定编译器是否会帮我们做相应优化。

## 测试过程

首先我们按上面所说的方法来实现两种不同的向量对标量的除法运算，这里我们选择用三维向量来进行测试。

```rust
#[derive(Copy, Clone)]
pub struct V1(pub f32, pub f32, pub f32);

// 求倒后乘法
impl core::ops::Div<f32> for V1 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        // 将三次除法替换成一次除法和三次乘法
        let inv = 1. / rhs;
        Self(self.0 * inv, self.1 * inv, self.2 * inv)
    }
}

#[derive(Copy, Clone)]
pub struct V2(pub f32, pub f32, pub f32);

// 直接相除法
impl core::ops::Div<f32> for V2 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        // 直接进行三次除法
        Self(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}
```

下面给出的是测试用例，为简化代码我们这里直接使用Rust的bench test模块。为防止出现下溢或上溢的情况，这里选择在bencher外初始化一个向量，向量的每个元素的初始值都设为`f32::MAX`,然后每次循环都除以一个非常小的增量1.01。

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
               x = x / 1.01;
           }
       });
    }

    #[bench]
    fn test_direct_div(bencher: &mut Bencher) {
       let mut x = V2(f32::MAX, f32::MAX, f32::MAX);
       bencher.iter(|| {
           for _ in 0..10000 {
               x = x / 1.01;
           }
       });
    }
}
```

在我的笔记本（CPU型号为AMD Ryzen 7 4800U）上运行的测试结果如下：

```bash
test bench::test_cached_div ... bench:      15,020 ns/iter (+/- 963)
test bench::test_direct_div ... bench:      47,348 ns/iter (+/- 578)
```

从测试结果我们可以看到，直接除法比优化的除法慢将近3倍，说明编译器并没有对除法运算做优化。

## 原因分析

事实上，编译器之所以没有进行上面所说的除法优化，是由于这种优化很可能会破坏程序的正确性（相对于性能而言，正确性是编译器在优化代码时首先要保证的）。IEEE浮点数标准要求，对于所有的\\( x \\), 有

\\[ \frac{1}{x} \cdot x = 1 \\]

由于浮点数求倒数有可能发生精度丢失，我们如果将计算的\\( \frac{1}{x} \\)保存起来再乘\\( x \\)的话，并不能保证结果还是1。编译器为了符合IEEE浮点数标准的要求，被限制执行这类的转换。也就是说，如果我们要打破这一保证以换取更高性能表现的话，需要自己手动做这方面的转换。

### 参考资料

[^1]: [Agner Fog指令测试表](https://agner.org/optimize/instruction_tables.pdf)
