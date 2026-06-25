use crate::parser::Pipeline;
use std::process::Command;

//shell is a process running , the cmds like ls or whatever these are its child processes. Command::new() does that for us here. 

pub fn exec (pipeline: Pipeline){
    let some = Command::new("ls");
}