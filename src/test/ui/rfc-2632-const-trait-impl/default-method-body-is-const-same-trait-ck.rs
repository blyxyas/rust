#![feature(const_trait_impl)]

#[const_trait]
pub trait Tr {
    fn a(&self) {}

    fn b(&self) {
        ().a()
        //~^ ERROR the trait bound
        //~| ERROR cannot call
    }
}

impl Tr for () {}

fn main() {}
