#![no_std]
#![feature(const_str_as_bytes)]

use core::sync::atomic::{AtomicBool, Ordering};

use serde::Serialize;
use serde_json_core;

use linux_kernel_module::sysctl::Sysctl;
use linux_kernel_module::Mode;
use linux_kernel_module::println;

struct JsonSysctlModule {
    a: Sysctl<AtomicBool>,
    b: Sysctl<AtomicBool>,
    c: Sysctl<AtomicBool>,
}

#[derive(Serialize)]
struct Output {
    a: bool,
    b: bool,
    c: bool,
}

impl linux_kernel_module::KernelModule for JsonSysctlModule {
    fn init() -> linux_kernel_module::KernelResult<Self> {
        Ok(JsonSysctlModule {
            a: Sysctl::register(
                "json-sysctl\x00",
                "a\x00",
                AtomicBool::new(false),
                Mode::from_int(0o666),
            )?,
            b: Sysctl::register(
                "json-sysctl\x00",
                "b\x00",
                AtomicBool::new(false),
                Mode::from_int(0o666),
            )?,
            c: Sysctl::register(
                "json-sysctl\x00",
                "c\x00",
                AtomicBool::new(false),
                Mode::from_int(0o666),
            )?,
        })
    }
}

impl Drop for JsonSysctlModule {
    fn drop(&mut self) {
        let o = Output {
            a: self.a.get().load(Ordering::Relaxed),
            b: self.b.get().load(Ordering::Relaxed),
            c: self.c.get().load(Ordering::Relaxed),
        };
        println!("{}", serde_json_core::to_string::<typenum::U32, _>(&o).unwrap());
    }
}

linux_kernel_module::kernel_module!(
    JsonSysctlModule,
    author: "Alex Gaynor and Geoffrey Thomas",
    description: "Use JSON serialization in kernelspace",
    license: "GPL"
);
