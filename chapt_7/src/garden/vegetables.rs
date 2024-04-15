#[derive(Debug)]
pub struct Asparagus{}

pub mod some_inner_module{
    #[derive(Debug)]
    pub struct Species{
        pub name: String,
        pub latin_name: String
    }
}