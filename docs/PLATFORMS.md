# Platform contract and evidence

Day-one design admits Linux, Windows, BSD, macOS, Android and iOS without OS
assumptions in core. The default facade/core is no_std and no allocator.
The compiler adapter runs on the host, even when emitted code targets mobile.
Cross-target `cargo check` does not link an app or exercise a device runtime.

| Family | Initial targets | Configured evidence | Remaining runtime evidence |
| --- | --- | --- | --- |
| Linux | x86_64-unknown-linux-gnu, aarch64-unknown-linux-gnu | Native Linux CI and portable cross check | ARM native fixtures as behavior lands |
| Windows | x86_64-pc-windows-msvc | Native CI and portable cross check | Hosted CI result pending |
| macOS | aarch64-apple-darwin, x86_64-apple-darwin | Native macOS CI and cross check | Hosted CI result pending |
| BSD | x86_64-unknown-freebsd | Portable cross check | FreeBSD native runner before runtime support claim; other BSDs need separate evidence |
| Android | aarch64-linux-android | Portable cross check | NDK/device or emulator consumer before runtime support claim |
| iOS | aarch64-apple-ios | Portable cross check | Apple SDK/simulator/device consumer before runtime support claim |
| No OS | thumbv7em-none-eabihf, riscv32imac-unknown-none-elf | no_std/no-alloc and alloc compile probes | No device execution claim |
| Aesynx | No available Rust target contract yet | Architecture review only | Toolchain/ABI/runtime unavailable; deliberately future |

No existing hosted CI run is claimed by configuration files. Record local results
in [setup verification](verification/setup.md). Target-specific APIs belong in
host boundaries. Portable storage uses caller buffers and checked sizes; avoid
OS handles, filesystem, atomics or pointer-width assumptions in public core types.
Aesynx support requires a dedicated future pass once its compiler/ABI exists and
is not a fabricated cross-compilation gate today.

Before 1.0, each advertised runtime/profile must have actual runtime evidence;
unsupported combinations remain explicit. Broader BSD/architecture coverage can
be added without changing the portable model.
