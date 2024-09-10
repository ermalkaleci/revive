use inkwell::{context::Context, memory_buffer::MemoryBuffer, module::Module, support::LLVMString};

include!(concat!(env!("OUT_DIR"), "/polkavm_exports.rs"));

/// Creates a LLVM module from the [BITCODE].
/// The module exports `call` and `deploy` functions (which are named thereafter).
/// Returns `Error` if the bitcode fails to parse, which should never happen.
pub fn module<'context>(
    context: &'context Context,
    module_name: &str,
) -> Result<Module<'context>, LLVMString> {
    let buf = MemoryBuffer::create_from_memory_range(BITCODE, module_name);
    Module::parse_bitcode_from_buffer(&buf, context)
}

#[cfg(test)]
mod tests {
    use crate::polkavm_exports;

    #[test]
    fn it_works() {
        inkwell::targets::Target::initialize_riscv(&Default::default());
        let context = inkwell::context::Context::create();
        let module = polkavm_exports::module(&context, "polkavm_exports").unwrap();

        assert!(module.get_function("call").is_some());
        assert!(module.get_function("deploy").is_some());
    }
}