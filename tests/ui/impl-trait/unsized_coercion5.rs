//! This test checks that opaque types get unsized instead of
//! constraining their hidden type to a trait object.

//@ revisions: next old
//@[next] compile-flags: -Znext-solver

#![feature(trait_upcasting)]

trait Trait {}

impl Trait for u32 {}

fn hello() -> Box<impl Trait + ?Sized> {
    if true {
        let x = hello();
        let y: Box<dyn Send> = x as Box<dyn Trait + Send>;
        //[old]~^ ERROR: the size for values of type `impl Trait + ?Sized` cannot be know
        //[old]~| ERROR: cannot check whether the hidden type of opaque type satisfies auto traits
        //~^^^ ERROR: mismatched types
    }
    Box::new(1u32)
}

fn main() {}
