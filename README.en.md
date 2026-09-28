# spinburn

A **stress program** for BORUIX, used to verify how a running process is handled when it is forcibly terminated (`SIGKILL`).

[简体中文](README.md)

## What it does

After starting it enters a loop: a short burst of CPU work, then a sleep, then wake and repeat. **It never exits on its own**; only a forced termination ends it.

The design intent is that at **whatever moment it is terminated**, it is in some genuine state — computing (running), runnable, or sleeping (blocked). The coordinating side can send the termination signal at any time and still hit a process that really exists and may be scheduled on this core or another.

## Why not a pure spin loop

A pure infinite spin **saturates the CPU**. Under a single-threaded emulated environment that drags the whole system to near-unresponsiveness and the test itself stops running. Adding the sleep keeps the process blocked and yielding most of the time, preserving "never exits" without starving other processes.

> For the contrasting case of "always ready, never yields", see [`yielder`](https://github.com/BRX-Boruix/yielder).

## Usage

The coordinating side spawns it and then sends `SIGKILL` at an arbitrary moment. The program has no verdict of its own: **the observer is the coordinating side**, which checks that the process was terminated correctly.

## Output

```
[spinburn] start (sleep-loop)
```

Nothing further after startup — printing continuously would impose irrelevant load on the termination path under test.

## Building

```bash
cargo build --release
```

## Layout

```
spinburn/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # the compute-sleep loop
```

## Related projects

- [`yielder`](https://github.com/BRX-Boruix/yielder) — the pure busy-yield stress program (the always-ready contrast)
- [`libsys`](https://github.com/BRX-Boruix/libsys) — the user-space syscall wrapper

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
