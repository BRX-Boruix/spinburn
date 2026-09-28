# spinburn

BORUIX 的**压测程序**，用于验证正在运行的进程被强制终止（SIGKILL）时的处理路径。

[English](README.en.md)

## 做什么

启动后进入一个循环：先做少量 CPU 计算，然后睡眠一段时间，再醒来继续。**它永不自然退出**，只能被
强制终止。

这样设计的目的是让进程在**任何时刻被终止**时，都处于某个真实状态——计算中（运行）、就绪、或
睡眠中（阻塞）。协调端在任意时刻发出终止信号，都能命中一个真实存在、可能正在本核或别核上被调度
的进程。

## 为什么不写成一个纯自旋循环

纯无限自旋会**占满 CPU**。在单线程模拟环境下，这会把整个系统拖到几乎无法响应，测试本身反而跑不
起来。加入睡眠后，进程多数时间处于阻塞态、让出 CPU，既保持"永不退出"，又不饿死其他进程。

> 需要"始终处于就绪态、从不让出"的对照场景，见 [`yielder`](https://github.com/BRX-Boruix/yielder)。

## 用法

由协调端派生后，在任意时刻发出 `SIGKILL`。程序本身没有输出判定，**观察方是协调端**：它检查进程
是否被正确终止。

## 输出

```
[spinburn] start (sleep-loop)
```

启动后不再输出——持续打印会给被测的终止路径制造无关负担。

## 构建

```bash
cargo build --release
```

## 文件结构

```
spinburn/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 计算-睡眠循环
```

## 相关项目

- [`yielder`](https://github.com/BRX-Boruix/yielder) —— 纯忙-让出压测（始终就绪的对照场景）
- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 用户态系统调用封装

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。
