#[allow(unused_imports)]
pub use builder::*;

pub trait Builder<T> {
    type Builder;

    fn builder() -> Self::Builder;
}

#[macro_use]
pub mod functional {
/// # Examples
/// Single variant
/// ```
///    use rusty_cake::constr;
///
///    struct A {
///        value: String
///    }
///
///    let mut a_var = A { value: "Test".to_owned() };
///    let a_apply = constr!(a_var = value: "Work!".to_owned());
///    assert_eq!(a_apply.value, "Work!");
/// ```
/// Multiple variant
/// ```
///    use rusty_cake::constr;
///    struct B {
///        value: String,
///        value_next: i32
///    }
///
///    let mut var = B { value: "Test".to_owned(), value_next: 100 };
///    let apply = constr!(var = value: "Work!".to_owned(), value_next: 1);
///    assert_eq!(apply.value, "Work!");
///    assert_eq!(apply.value_next, 1);
///
/// ```
    #[macro_export]
    macro_rules! constr {
        ($rec:ident = $($var:ident : $exp:expr),*) => ({
            $($rec.$var = $exp;)*
            $rec
        });
    }
/// # Examples
/// Call block
/// ```
///    use rusty_cake::call;
///
///    struct A {
///        value: String
///    }
///
///    impl A {
///        fn print_ok(&self) {
///            println!("OK")
///        }
///
///        fn print_num(&self, num: i32) {
///            println!("{}", num)
///        }
///    }
///
///    let mut f = A { value: "TEST".to_owned() };
///    let mut d = call!(f ->
///        print_ok(),
///        print_num(10)
///    );
///
/// ```
    #[macro_export]
    macro_rules! call {
        ($rec:ident -> $($b:ident$args:tt),*) => {{
            $($rec.$b$args;)*
            $rec
        }};
    }

/// # Examples
/// Also block
/// ```
///    use rusty_cake::functional::Also;
///
///    struct A {
///        value: String
///    }
///
///    impl A {
///        fn print_ok(&self) {
///            println!("OK")
///        }
///
///        fn print_num(&self, num: i32) {
///            println!("{}", num)
///        }
///    }
///
///    let mut a = A { value:"Ok test".to_owned() };
///    a.also(|it|{
///        it.print_ok();
///        it.print_num(4i32);
///    });
///
/// ```
    pub trait Also: Sized {
        type Out;

        fn also(self, block: impl FnMut(&mut Self::Out) -> ()) -> Self::Out;
    }

    impl <T> Also for T {
        type Out = T;

        fn also(mut self, mut block: impl FnMut(&mut T) -> ()) -> T {
            block(&mut self);
            self
        }
    }
}