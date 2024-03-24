#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), warn(unused_extern_crates))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate revm_jit_core;

use revm_primitives::U256;

mod compiler;
pub use compiler::JitEvm;

mod gas;
pub use gas::*;

mod bytecode;
pub use bytecode::*;

#[doc(inline)]
pub use revm_jit_core::*;

#[cfg(feature = "llvm")]
#[doc(no_inline)]
pub use llvm::JitEvmLlvmBackend;
#[cfg(feature = "llvm")]
#[doc(inline)]
pub use revm_jit_llvm as llvm;

#[cfg(feature = "cranelift")]
#[doc(no_inline)]
pub use cranelift::JitEvmCraneliftBackend;
#[cfg(feature = "cranelift")]
#[doc(inline)]
pub use revm_jit_cranelift as cranelift;

#[allow(dead_code)]
const MINUS_1: U256 = U256::MAX;
#[allow(dead_code)]
const I256_MIN: U256 = U256::from_limbs([
    0x0000000000000000,
    0x0000000000000000,
    0x0000000000000000,
    0x8000000000000000,
]);
#[allow(dead_code)]
const I256_MAX: U256 = U256::from_limbs([
    0xFFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFF,
    0xFFFFFFFFFFFFFFFF,
    0x7FFFFFFFFFFFFFFF,
]);

// Enable for `cargo-asm`.
#[cfg(any())]
pub fn generate_all_assembly() -> JitEvm<JitEvmLlvmBackend<'static>> {
    let cx = Box::leak(Box::new(llvm::inkwell::context::Context::create()));
    let mut jit = JitEvm::new(JitEvmLlvmBackend::new(cx, OptimizationLevel::Aggressive).unwrap());
    let _ = jit.compile(&[], primitives::SpecId::ARROW_GLACIER).unwrap();
    unsafe { jit.free_all_functions().unwrap() };
    jit
}
