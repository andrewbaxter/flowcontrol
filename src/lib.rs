/// A break shed. This defines a stopping point for `break` in the enclosed code,
/// as if you had wrapped it in `loop {}`.
///
/// Use like:
///
/// ```rust
/// shed!{
///     ...
///     break;
/// }
/// ```
///
/// or, labeled:
///
/// ```rust
/// shed!{
///     'x _;
///     ...
///     break 'x;
/// }
/// ```
#[macro_export]
macro_rules! shed{
    // With a break label, like `bb!{ 'label _; ... }`
    ($l: lifetime _; $($t: tt) *) => {
        $l: loop {
            #[allow(unreachable_code)] break {
                $($t) *
            };
        }
    };
    // No label, `bb!{ ... }`
    ($($t: tt) *) => {
        loop {
            #[allow(unreachable_code)] break {
                $($t) *
            };
        }
    };
}

/// Execute the second block where the first block breaks with the label, like the
/// first block is a giant `if` condition.
///
/// Use like:
///
/// ```rust
/// superif!({
///     ...
///     break 'then;
/// } 'then {
///     ...
/// });
/// ```
///
/// If the first block exits without breaking to the label the 2nd block won't be
/// entered.
///
/// There's an alternative form if you want to pass data from the conditional block:
///
/// ```rust
/// superif!({
///     ...
///     break 'then 39;
/// } data = 'then {
///     println!("{}", data);
/// });
/// ```
#[macro_export]
macro_rules! superif{
    ({
        $($t: tt) *
    } $l: lifetime {
        $($t2: tt) *
    }) => {
        superif!({
            $($t) *
        } _unused = $l {
            $($t2) *
        })
    };
    ({
        $($t: tt) *
    } $i: ident = $l: lifetime {
        $($t2: tt) *
    }) => {
        $crate:: shed ! {
            'superif _;
            // Condition
            let $i = shed ! {
                $l _;
                break 'superif $crate:: shed ! {
                    $($t) *
                };
            };
            // Else
            $($t2) *
        }
    }
}

#[test]
fn test_superif() {
    superif!({
        if true {
            break 'then;
        }
    } 'then {
    });
}

/// Type-assert return type: explicitly communicate the `Ok` return type for the
/// block to the compiler via unreachable code.
///
/// Use like `ta_return!(i32, MyError);`
#[macro_export]
macro_rules! ta_return{
    ($t: ty, $e: ty) => {
        if false {
            fn unreachable_value<T>() -> T {
                panic!();
            }
            return std:: result:: Result::< $t,
            $e >:: Ok(unreachable_value());
        }
    }
}
