macro_rules! make_fn {
    ($input:item) => {
        #[async_syn::parse_fn]
        fn outer() {
            $input
        }
    }
}

make_fn!(async fn inner() {});

fn main() {
    println!("Hello, world!");
}
