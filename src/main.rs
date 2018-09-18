#![recursion_limit="512"]
#![feature(trace_macros)]

extern crate aerosol;
#[macro_use]
extern crate tt_call;
extern crate failure;

#[macro_export]
macro_rules! tt_debug2 {
    {
        $(
            $output:ident = [{ $($tokens:tt)* }]
        )*
    } => {
        $(
            println!("{}",
                concat!(
                    stringify!($output),
                    " = [{ ",
                    stringify!($($tokens)*),
                    " }]",
                )
            );
        )*
    }
}

aerosol::define_interface!(
    TestInterface {
        fn test_get(&self) -> Vec<u8>;
    }
);

struct FooFactory;
#[derive(Clone, Debug)]
struct Foo;
#[derive(Clone, Debug)]
struct Bar;

impl aerosol::Factory for FooFactory {
    type Object = Foo;
    fn build() -> Result<Foo, failure::Error> { Ok(Foo) }
}

aerosol::define_context!(
    TestContext {
        foo: Foo [FooFactory],
        bar: Bar
    }
);

fn main() {

    //trace_macros!(true);
    //aerosol::test_macro!();
    tt_call! {
        macro = [{ aerosol::private_define_interface }]
        input = [{ TestInterface {
            fn test_get(&self) -> Vec<u8>;
        } }]
        ~~> tt_debug2
    }
    tt_call! {
        macro = [{ aerosol::private_define_context }]
        input = [{ TestContext {
            db: MyDatabase [PostgresFactory<MyDatabase>],
            pusher: PusherClient
        } }]
        ~~> tt_debug2
    }
}