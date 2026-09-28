# spinburn

**简体中文** | [English](#english)

BORUIX 的一个**测试夹具程序**——它存在的唯一目的是被杀掉。

它启动后进入一个永远不会自然退出的循环：做一小段 CPU 计算，然后睡眠，醒来继续。任何时刻向它
发送终止信号，都能命中一个真实存在的、正在运行或正在等待的进程。

```
[spinburn] start (sleep-loop)
```

---

## 为什么需要这样一个程序

要验证**进程终止**这条路径是可靠的，就需要一个"可以被杀掉的目标"。

但一个合适的目标并不容易构造，因为终止路径必须处理进程的**三种不同状态**：

| 状态 | 含义 |
| --- | --- |
| 运行中 | 正在占用 CPU |
| 就绪 | 已可运行，等待被调度 |
| 阻塞 | 正在等待某件事（如睡眠） |

如果测试目标只在其中一种状态停留，那么另外两种状态的终止路径就**测不到**。

`spinburn` 的设计就是为了**同时覆盖这三种状态**：它交替进行 CPU 计算与睡眠，因此无论在哪
一刻被终止，都会落在其中某个状态上。

## 为什么不是简单的死循环

一个纯粹的无限循环（`loop {}`）确实一直在"运行中"，但它有个实际问题：**把 CPU 占满**。

在模拟环境中，这会拖慢甚至饿死其他所有东西——测试还没跑完，环境本身已经卡住了。

所以 `spinburn` 采用的是**短促计算 + 睡眠**的模式：

- 计算阶段让它**真实地处于运行中**（这样才能被在运行状态下终止）
- 睡眠阶段让它**让出 CPU**，同时进入阻塞状态（这样才覆盖阻塞态的终止路径）

两者交替，既覆盖了状态，又不至于把机器拖死。

## 名字的由来

"spin"（自旋）指它持续活动，"burn"（燃烧）指它消耗 CPU。合起来就是它的行为：烧着 CPU 转圈，
但转得有节制。

## 怎么用它

它不是一个给用户使用的命令，而是**测试流程的一部分**：测试程序启动它，然后在任意时刻向它发送
终止信号，验证终止是否干净完成——进程被正确回收、不留下僵尸、其他进程不受影响。

持续运行多个实例并反复终止，可以压出并发终止路径上的问题。

## 输出

启动时打印一行标识，之后不再输出。

**它永远不打印"结束"**——因为设计上它只能被强制终止，不会走到退出路径。这是刻意的：如果它
自己退出了，测试就无法区分"终止成功"和"进程自己跑了"。

## 观察方式

它本身不报告结果。验证终止是否成功，靠的是**从外部观察**：

- 进程列表中不再出现该进程
- 终止后系统仍然正常响应
- 反复终止不会导致系统异常

## 构建

```bash
cargo build --release
```

编译产物部署为 BORUIX 系统中的用户态程序，由测试流程拉起。

## 文件结构

```
spinburn/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 计算与睡眠交替的循环
```

## 相关项目

- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 用户态系统调用封装
- [`selftest`](https://github.com/BRX-Boruix/selftest) —— 系统自检，包含终止路径的压测

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。

---

# English

[简体中文](#spinburn) | **English**

A **test fixture program** for BORUIX — its sole purpose is to be killed.

After starting, it enters a loop that never exits on its own: a short burst of CPU work, then sleep,
then wake and repeat. Sending it a termination signal at any moment hits a process that genuinely
exists and is either running or waiting.

```
[spinburn] start (sleep-loop)
```

---

## Why such a program is needed

Verifying that the **process termination** path is reliable requires a target that can be killed.

A suitable target is not easy to construct, because the termination path must handle three distinct
process states:

| State | Meaning |
| --- | --- |
| Running | Currently occupying the CPU |
| Ready | Runnable, waiting to be scheduled |
| Blocked | Waiting on something (a sleep, say) |

If the test target only ever sits in one of these, the termination path for the other two goes
**untested**.

`spinburn` is designed to **cover all three at once**: it alternates CPU work with sleep, so
whenever it is terminated it lands in one of those states.

## Why not a simple infinite loop

A plain infinite loop (`loop {}`) certainly stays "running", but it has a practical problem: **it
saturates the CPU**.

In an emulated environment that slows down or even starves everything else — the test would not
finish because the environment itself would be stuck.

So `spinburn` uses a **short burst plus sleep** pattern:

- The compute phase puts it **genuinely running** (so termination from the running state is exercised)
- The sleep phase **yields the CPU** and enters the blocked state (so termination from the blocked
  state is exercised)

Alternating between the two covers the states without dragging the machine down.

## Where the name comes from

"Spin" for its continuous activity, "burn" for the CPU it consumes. Together they describe the
behaviour: burning CPU in circles, but with restraint.

## How it is used

It is not a command for users but **part of the test flow**: the test starts it, then sends it a
termination signal at an arbitrary moment and verifies that termination completes cleanly — the
process is reaped, no zombie is left, and other processes are unaffected.

Running several instances continuously and killing them repeatedly shakes out problems on concurrent
termination paths.

## Output

It prints one identifying line at startup and nothing thereafter.

**It never prints "finished"** — by design it can only be force-terminated and never reaches an exit
path. That is deliberate: if it could exit by itself, the test could not distinguish "termination
succeeded" from "the process ran off on its own".

## How it is observed

It reports no results itself. Whether termination succeeded is established **by external
observation**:

- The process no longer appears in the process list
- The system still responds normally after termination
- Repeated termination does not leave the system misbehaving

## Building

```bash
cargo build --release
```

The artifact is deployed as a user-space program in a BORUIX system, started by the test flow.

## Layout

```
spinburn/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # the alternating compute-and-sleep loop
```

## Related projects

- [`libsys`](https://github.com/BRX-Boruix/libsys) — the user-space syscall wrapper
- [`selftest`](https://github.com/BRX-Boruix/selftest) — the system self-test, including the termination stress test

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
