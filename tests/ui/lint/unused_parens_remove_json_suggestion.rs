// compile-flags: --error-format json
// error-pattern:unnecessary parentheses
// run-rustfix

// The output for humans should just highlight the whole span without showing
// the suggested replacement, but we also want to test that suggested
// replacement only removes one set of parentheses, rather than naïvely
// stripping away any starting or ending parenthesis characters—hence this
// test of the JSON error format.

#![deny(unused_parens)]
#![allow(unreachable_code)]

fn main() {

    let _b = false;

    if (_b) {
        println!("hello");
    }

    f();

}

fn f() -> bool {
    let c = false;

    if(c) {
        println!("next");
    }

    if (c){
        println!("prev");
    }

    while (false && true){
        if (c) {
            println!("norm");
        }

    }

    while(true && false) {
        for _ in (0 .. 3){
            println!("e~")
        }
    }

    for _ in (0 .. 3) {
        while (true && false) {
            println!("e~")
        }
    }


    loop {
        if (break { return true }) {
        }
    }
    false
}
