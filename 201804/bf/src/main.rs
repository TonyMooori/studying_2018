// 写経
// REF: https://github.com/mihyaeru21/brainfuck-rs/blob/master/src/main.rs

extern crate bf;

use std::io;
use bf::interpreter::Interpreter;

fn main() {
    let src = ">+++++++++[<++++++++>-]<.>+++++++[<++++>-]<+.+++++++..+++.[-]>++++++++[<++
++>-]<.>+++++++++++[<+++++>-]<.>++++++++[<+++>-]<.+++.------.--------.[-]>
++++++++[<++++>-]<+.[-]++++++++++.";
    let mut input = io::stdin();
    let mut output = io::stdout();
    let mut interpreter = Interpreter::new(30000,&mut input,&mut output);

    if let Err(e) = interpreter.run(src){
        println!("{}",e);
    }
}
