// 写経
// REF: https://github.com/mihyaeru21/brainfuck-rs/blob/master/src/main.rs

extern crate bf;

use bf::interpreter::Interpreter;

#[test]
fn run(){
    let mut input = "hello".as_bytes();
    let mut output : Vec<u8> = vec![];
    let src = "+++++++++[>++++++++>+++++++++++>+++++<<<-]>.>++.+++++++..+++.>-.\
                     ------------.<++++++++.--------.+++.------.--------.>+.,.,.,.,.";
    let _ = Interpreter::new(20,&mut input,&mut output).run(&src);
    let output_str = std::str::from_utf8(&output).unwrap();
    assert_eq!(output_str,"Hello, world!hell");
}

#[test]
fn run2(){
    let src = "++++>++><<[-  >[->>+<<]  >>[-<+<+>>]  <<<]>>++++++++++++++++++++++++++++++++++++++++++++++++.";
    let mut input = "".as_bytes();
    let mut output : Vec<u8> = vec![];
    let _ = Interpreter::new(20,&mut input,&mut output).run(&src);
    let output_str = std::str::from_utf8(&output).unwrap();
    assert_eq!(output_str,"8");
}