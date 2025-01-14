# sleep

在同步编程中，我们可以使用 `std::thread::sleep` 来实现等待一段时间。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::sleep(Duration::from_secs(1));
    println!("Hello, world!");
}
```

然而，在异步编程中，我们不能使用 `std::thread::sleep`，因为它会阻塞当前线程，而异步编程的目的是不阻塞当前线程。

我们可以使用 `tokio::time::sleep` 来实现等待一段时间。

```rust
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        println!("Task started.");
        sleep(Duration::from_secs(1)).await; // 非阻塞延时
        println!("Task finished.");
    });
    handle.await.unwrap();
}
```
