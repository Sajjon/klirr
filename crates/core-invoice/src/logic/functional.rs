/// Curry a function that takes two arguments into a function that takes one argument and returns another function.
/// This is useful for partially applying functions in a functional programming style.
pub fn curry2<T, U, R>(f: impl FnOnce(T, U) -> R, u: U) -> impl FnOnce(T) -> R {
    move |t| f(t, u)
}

/// Curry a function that takes one argument into a function that takes zero argument and returns another function.
/// This is useful for partially applying functions in a functional programming style.
pub fn curry1<T, R>(f: impl FnOnce(T) -> R, t: T) -> impl FnOnce() -> R {
    move || f(t)
}

pub trait ResultExt<T, E>: Sized {
    fn map_to_void(self) -> Result<(), E>;
}

impl<T, E> ResultExt<T, E> for Result<T, E> {
    fn map_to_void(self) -> Result<(), E> {
        self.map(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curry2() {
        let add = |x: i32, y: i32| x + y;
        let add_five = curry2(add, 5);
        assert_eq!(add_five(3), 8);
    }

    #[test]
    fn test_curry1() {
        let greet = |name: &str| format!("Hello, {}!", name);
        let greet_john = curry1(greet, "John");
        assert_eq!(greet_john(), "Hello, John!");
    }

    #[test]
    fn test_result_ext() {
        let ok_result: Result<i32, &str> = Ok(42);
        let err_result: Result<i32, &str> = Err("Error");

        assert_eq!(ok_result.map_to_void(), Ok(()));
        assert_eq!(err_result.map_to_void(), Err("Error"));
    }
}
