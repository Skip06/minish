use crate::parser::Pipeline;
use std::process::Command;

//shell is a process running , the cmds like ls or whatever these are its child processes. Command::new() does that for us here. 

// .spawn()=> Behavior: It forks the current process and executes the command asynchronously in the background. It does not wait for the command to finish.
// Return Value: It returns a Result<Child>, giving you a handle to the child process so you can check its status, stream its output, or force-kill it.

pub fn exec (pipeline: Pipeline)-> Result<(), String>{
    //if it was echo hello we got the program name echo inside new() , other items except 1st one are arguments 
    let mut child_process = Command::new(&pipeline.commands[0].argv[0])
        .args(&pipeline.commands[0].argv[1..])
        .spawn()
        .map_err(|e| e.to_string())?;

    child_process.wait().map_err(|e| e.to_string())?;
    Ok(())

    
}

//so i think if the program is pwd then rust is asking os to run this prog named pwd and it finds the executable in the path env var and runs it . (confirmed)
