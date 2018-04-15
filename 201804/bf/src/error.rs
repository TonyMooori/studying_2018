// 写経
// REF: https://github.com/mihyaeru21/brainfuck-rs/blob/master/src/main.rs

use std::io;
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error{
    Io(io::Error),
    Memory(&'static str),
    Jump(&'static str)
}

impl fmt::Display for Error{
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result{
        match *self{
            Error::Io(ref e) => write!(f,"IO ERROR: {}",e),
            Error::Memory(ref s) => write!(f,"MEMORY ERROR: {}",s),
            Error::Jump(ref s) => write!(f,"JUMP ERROR: {}",s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str{
        match *self{
            Error::Io(ref e) => e.description(),
            Error::Memory(ref s) => s,
            Error::Jump(ref s) => s,
        }
    }

    fn cause(&self)->Option<&error::Error>{
        match *self{
            Error::Io(ref e)=> Some(e),
            Error::Memory(_)=>None,
            Error::Jump(_)=>None,
        }
    }
}

impl From<io::Error> for Error{
    fn from(e:io::Error) -> Error{
        Error::Io(e)
    }
}