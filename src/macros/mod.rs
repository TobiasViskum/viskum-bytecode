mod debug_macros;

#[macro_export]
macro_rules! perform_op {
    ($vm:expr, $op:expr) => {

        match std::panic::catch_unwind(|| $op) {
            Ok(val) => val,
            Err(err) => {
                let msg = if let Some(s) = err.downcast_ref::<&str>() {
                    s
                } else if let Some(s) = err.downcast_ref::<String>() {
                    s.as_str()
                } else {
                    "Failed to perform operation"
                };
                $vm.runtime_error(msg);
                return Value::Null
            }
        }
    };
}
