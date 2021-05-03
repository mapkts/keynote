# RAII与资源管理

如果要给计算机史上最糟糕的技术词汇命名弄一个排行榜的话，由C++之父创造的RAII[^1]肯定榜上有名。首先是发音问题，到底该念Ray? Rai? 还是R.A.I.I呢？没有人知道。其次，作为一种有关资源获取和释放的管理机制，其全称**资源获取即初始化**(Resource Acquisition is Initialization)只描述了RAII机制关于资源初始化的部分，而事实上更为关键的**资源释放即析构**(Resource Release is Destruction)却选择性地被忽略了，导致很多像我这样的小白刚接触的时候就被其高大上唬住了，不敢知其所以然。

为了更好地理解RAII机制，我们先来聊聊什么是资源。需要注意的是，在不同的语境下资源有不同的含义，我们上面提到的资源实际上是指操作系统提供给应用程序的内存、文件句柄、socket和锁等数量有限的东西，这里为方便理解我们从资源管理方式角度将其分为下图三类：

![system resources](./img/system_resources.png)

第一类是静态分配的内存，也就是栈内存、全局变量区和静态变量区等在程序运行一开始就会分配，程序结束时才会被释放的内存。这类内存的分配和释放，编译器在程序编译和链接阶段就已经确定，不需要用户手动管理。由于Rust的内存模型目前还未稳定下来[^2]，这里也就不具体展开来说了，怕被打脸。

第二类是系统根据程序的需要在运行时动态分配的堆内存，这类内存由程序自行申请，用完后需要手动释放，否则会发生内存泄漏。
堆上内存的管理，C++、Rust等比较偏底层的语言选择使用RAII机制来管理，而Go、Java等比较偏应用层的静态类型语言和大部分动态类型基本都通过GC来管理。RAII和GC虽然都能自动管理堆上内存，解放程序员的双手，但是两者的思想是很不一样的，这个我们留后面再讲。

第三类是诸如文件句柄/描述符、锁、socket等从操作系统获取的资源，这类资源必须由用户显示地关闭才能被操作系统回收，单单清除内存里的持有对象仍然会导致内存泄漏（甚至可能导致更严重程序逻辑错误，如忘记释放锁）。GC语言可以让用户不用操心内存的释放问题，但对于这类资源的管理就无能为力了。

RAII要解决的问题，就是让第三类资源和第二类内存的处理方式统一起来，具体来说就是：

- Resource Acquisition is Initialization: 在持有对象被初始化时完成资源的分配
- Resource Release is Destruction: 在持有对象被析构时完成资源的释放

RAII通过这种将资源的生命周期和持有对象的生命周期强绑定的方式，保证了只要对象能被正确地析构（内存安全），就不会出现资源泄漏的问题（资源安全）。

作为一种比GC更通用的资源管理机制，RAII不仅具有能保持代码的简洁性，减少开发者的负担的优势，而且可以很好地保证代码的异常安全性。比如下面没有RAII的代码中，如果在Mutex获取之后，被释放之前发生了异常的话，释放锁的代码不会被执行，会导致资源泄漏和可能的死锁问题。

```rust
# ![feature(mutex_unlock)]
# use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(0); 

    let mut guard = mutex.lock().unwrap();

    // 这里出现了异常！

    Mutex::unlock(guard); // 因为出现了异常，这里的解锁语句不会再执行
}
```

而对于支持RAII的语言（比如Rust会锁的释放），即使发生了异常，Mutex也会被在栈展开的过程中被析构，保证了异常安全。

```rust
# ![feature(mutex_unlock)]
# use std::sync::Mutex;

fn main() {
    let mutex = Mutex::new(0); 

    let mut guard = mutex.lock().unwrap();

    // 这里出现了异常！

    // 当离开作用域时，mutex会被自动析构 (不管是否抛出了异常)
}
```

对于GC语言来说，为了保证资源管理的异常安全，通常会提供一个语句保证资源在出现异常时也能被正确地释放，比如Java提供的finally语句和Golang的defer语句。以下是一个在Java中用finally来释放socket的例子：

```java
void foo() {
    Socket socket;
    try {
        socket = new Socket();
        access(socket);
    } finally {
        socket.close(); // 不管有没有发生异常这里都会执行
    }
}
```

## Rust中RAII的实现方式

TODO：

- Rust enforces RAII
- RAII和所有权机制完美契合
- 通过Drop Trait实现
- 通过Move语义和drop()可提前释放资源，不必等到函数结束

## 为什么GC语言不能实现RAII

TODO:

- 待思考和查阅资料

### 参考资料

[^1]: [https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization)

[^2]: [https://doc.rust-lang.org/stable/reference/memory-model.html](https://doc.rust-lang.org/stable/reference/memory-model.html)

[^3]: [https://doc.rust-lang.org/rust-by-example/scope/raii.html](https://doc.rust-lang.org/rust-by-example/scope/raii.html)
