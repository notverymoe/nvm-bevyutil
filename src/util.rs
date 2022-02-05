
#[macro_export]
macro_rules! try_unwrap_option {
    ($t:expr) => {
        if let Some(v) = $t { v } else { return; }
    };
    ($t:expr, $r:expr) => {
        if let Some(v) = $t { v } else { return $r; }
    };
}

#[macro_export]
macro_rules! try_unwrap_result {
    ($t:expr) => {
        if let Ok(v) = $t { v } else { return; }
    };
    ($t:expr, $r:expr) => {
        if let Ok(v) = $t { v } else { return $r; }
    };
}
