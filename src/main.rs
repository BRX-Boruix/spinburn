//! BORUIX spinburn: 长驻(不自然退出)子进程，专用于跨核 SIGKILL terminate 压测。
//!
//! 循环做少量 CPU 工作后 sleep（进入 Blocked），再醒来继续——进程永不自然退出，
//! 但多数时间阻塞/sleep，不会像纯无限自旋那样饿死 QEMU 单线程 TCG。协调端
//! spawn 后在任意时刻 kill(SIGKILL)，都能命中一个真实存在、在本核或别核调度的
//! 进程（terminate_locked 处理 Running/Ready/Blocked 三种状态）。
#![no_std]
#![no_main]

use libsys::write;
use libsys::STDOUT;

#[unsafe(no_mangle)]
pub extern "C" fn user_main(_argc: isize, _argv: *const *const u8) -> i32 {
    let _ = write(STDOUT, b"[spinburn] start (sleep-loop)\n");
    let mut acc: u64 = 0x1234_5678;
    let mut i: u32 = 0;
    loop {
        // 短 CPU 突发（使进程真实处于 Running 可被杀），随后 sleep 让出核心。
        for _ in 0..200_000 {
            acc = acc.wrapping_mul(6364136223846793005).wrapping_add(1);
        }
        i = i.wrapping_add(1);
        // 每轮 sleep 40ms：进程在 Blocked 态；被 SIGKILL 时终止路径处理之。
        let _ = libsys::sleep(40_000_000);
    }
    // unreachable: 只能经 SIGKILL 退出
}