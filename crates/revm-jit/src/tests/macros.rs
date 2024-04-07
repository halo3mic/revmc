macro_rules! matrix_tests {
    ($run:ident) => {
        #[cfg(feature = "llvm")]
        mod llvm {
            use super::*;

            fn run_llvm(jit: &mut JitEvm<JitEvmLlvmBackend<'_>>) {
                set_test_dump(jit, module_path!());
                $run(jit);
            }

            #[test]
            fn unopt() {
                with_llvm_backend_jit(OptimizationLevel::None, run_llvm);
            }

            #[test]
            fn opt() {
                with_llvm_backend_jit(OptimizationLevel::Aggressive, run_llvm);
            }
        }
    };

    ($name:ident = | $jit:ident | $e:expr) => {
        mod $name {
            use super::*;

            fn run_generic<B: Backend>($jit: &mut JitEvm<B>) {
                $e;
            }

            matrix_tests!(run_generic);
        }
    };
    ($name:ident = $run:ident) => {
        mod $name {
            use super::*;

            matrix_tests!($run);
        }
    };
}

macro_rules! build_push32 {
    ($code:ident[$i:ident], $x:expr) => {{
        $code[$i] = op::PUSH32;
        $i += 1;
        $code[$i..$i + 32].copy_from_slice(&$x.to_be_bytes::<32>());
        $i += 32;
    }};
}

macro_rules! tests {
    ($($group:ident { $($t:tt)* })*) => { uint! {
        $(
            mod $group {
                use super::*;

                tests!(@cases $($t)*);
            }
        )*
    }};

    (@cases $( $name:ident($($t:tt)*) ),* $(,)?) => {
        $(
            matrix_tests!($name = |jit| run_test_case(tests!(@case $($t)*), jit));
        )*
    };

    (@case @raw { $($fields:tt)* }) => { &TestCase { $($fields)* ..Default::default() } };

    (@case $op:expr $(, $args:expr)* $(,)? => $($ret:expr),* $(,)? $(; op_gas($op_gas:expr))?) => {
        &TestCase {
            bytecode: &tests!(@bytecode $op, $($args),*),
            expected_stack: &[$($ret),*],
            expected_gas: tests!(@gas $op $(, $op_gas)?; $($args),*),
            ..Default::default()
        }
    };

    (@bytecode $op:expr, $a:expr) => { bytecode_unop($op, $a) };
    (@bytecode $op:expr, $a:expr, $b:expr) => { bytecode_binop($op, $a, $b) };
    (@bytecode $op:expr, $a:expr, $b:expr, $c:expr) => { bytecode_ternop($op, $a, $b, $c) };

    (@gas $op:expr; $($args:expr),+) => {
        tests!(@gas
            $op,
            DEF_OPINFOS[$op as usize].static_gas().expect(stringify!($op)) as u64;
            $($args),+
        )
    };
    (@gas $op:expr, $op_gas:expr; $($args:expr),+) => {
        $op_gas + tests!(@gas_base $($args),+)
    };
    (@gas_base $a:expr) => { 3 };
    (@gas_base $a:expr, $b:expr) => { 6 };
    (@gas_base $a:expr, $b:expr, $c:expr) => { 9 };
}