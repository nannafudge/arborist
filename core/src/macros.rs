/*################################
          Common Macros
################################*/
#[macro_export]
macro_rules! unwrap_enum {
    ($var:expr, $return:expr, $default:expr, $subcase:pat $(, $subcases:pat )*) => {
        match $var {
            $subcase $(| $subcases)* => $return,
            _ => $default
        }
    };
    ($var:expr, $return:expr, $subcase:pat $(, $subcases:pat )*) => {
        match $var {
            $subcase $(|$subcases)* => $return
        }
    };
    ($var:expr, $default:expr, $( $arm:pat => $body:expr ),+) => {
        match $var {
            $($arm => $body,)+
            _ => $default
        }
    };
    ($var:expr, $( $arm:pat => $body:expr ),+) => {
        match $var {
            $($arm => $body,)+
        }
    };
}

#[macro_export]
macro_rules! require {
    ($($clause:expr)+, $err:expr) => {
        if !($($clause)+) {
            return Err($err);
        }
    };
}