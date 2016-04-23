#[macro_use]
mod common;

fn main() {
    println!("hello there !");
}

fn setup() {
    println!("setup called");

    use std::env;

    // We assume that we are in a valid directory.
    let p = env::current_dir().unwrap();
    println!("The current directory is {}", p.display());
}

fn teardown(_void: ()) {

}

test!(plop {
    assert_eq!(1,1)
});

test!(plip {
    assert_eq!(2,2)
});

it!(should_be_ran {
    assert_eq!("a", &String::from("a"))
});

/*
use std::process;
use std::env;

speculate! {
    describe "network communication" {

        before {
            println!("cwd={:?}", env::current_dir());
            1+1
        }

        it "can add stuff" {
            assert_eq!(1, 1+0)
        }
    }
}
*/
