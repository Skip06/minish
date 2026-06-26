use crate::parser::Pipeline;
use std::{fs::File, process::{Command, Stdio}};

//shell is a process running , the cmds like ls or whatever these are its child processes. Command::new() does that for us here. 

// .spawn()=> Behavior: It forks the current process and executes the command asynchronously in the background. It does not wait for the command to finish.
// Return Value: It returns a Result<Child>, giving you a handle to the child process so you can check its status, stream its output, or force-kill it.

pub fn exec (pipeline: Pipeline)-> Result<(), String>{

    if pipeline.commands.len() == 1{   // not ls | wc 
        
        let cmd = &pipeline.commands[0];
        match cmd.argv[0].as_str() {                   // builtin cd 
            "cd" => {
            
                std::env::set_current_dir(&pipeline.commands[0].argv[1])
                    .map_err(|e| e.to_string())?;
            
                return Ok(());
            }
            _ => {}
        }
    
        
        
        //if it was echo hello we got the program name echo inside new() , other items except 1st one are arguments 
        // the if else ladder is for "sort < name.txt > file.txt"
        if pipeline.commands[0].stdin_from.is_some() && !pipeline.commands[0].stdout_to.is_some(){
            let file = File::open(pipeline.commands[0].stdin_from.clone().unwrap()).map_err(|e| e.to_string())?;
            let mut child_process = Command::new(&cmd.argv[0])
                .args(&pipeline.commands[0].argv[1..])
                .stdin(Stdio::from(file))
                .spawn()
                .map_err(|e| e.to_string())?;
        
            child_process.wait().map_err(|e| e.to_string())?;
        }
        
        else if !pipeline.commands[0].stdin_from.is_some() && pipeline.commands[0].stdout_to.is_some(){
            let file = File::create(pipeline.commands[0].stdout_to.clone().unwrap()).map_err(|e| e.to_string())?;
            let mut child_process = Command::new(&cmd.argv[0])
                .args(&pipeline.commands[0].argv[1..])
                .stdout(Stdio::from(file))
                .spawn()
                .map_err(|e| e.to_string())?;
        
            child_process.wait().map_err(|e| e.to_string())?;
        }
        
        else if pipeline.commands[0].stdin_from.is_some() && pipeline.commands[0].stdout_to.is_some(){
            let input_file = File::open(pipeline.commands[0].stdin_from.clone().unwrap()).map_err(|e| e.to_string())?;
            let output_file = File::create(pipeline.commands[0].stdout_to.clone().unwrap()).map_err(|e| e.to_string())?;
            let mut child_process = Command::new(&cmd.argv[0])
                .args(&pipeline.commands[0].argv[1..])
                .stdin(Stdio::from(input_file))
                .stdout(Stdio::from(output_file))
                .spawn()
                .map_err(|e| e.to_string())?;
        
            child_process.wait().map_err(|e| e.to_string())?;
        }
        
        else{
            let mut child_process = Command::new(&cmd.argv[0])
                .args(&pipeline.commands[0].argv[1..])
                .spawn()
                .map_err(|e| e.to_string())?;
        
            child_process.wait().map_err(|e| e.to_string())?;
        }
    }
    else {      // ls | wc

        let mut children = Vec::new();
        let mut current_pipe = None;
        
        for i in 0..pipeline.commands.len() {
            let cmd = &pipeline.commands[i];
        
            let mut process = Command::new(&cmd.argv[0]);
        
            process.args(&cmd.argv[1..]);
        
            if let Some(pipe) = current_pipe.take() { // connect previous command's output
                process.stdin(Stdio::from(pipe));
            }
        
            
            if i != pipeline.commands.len() - 1 {  // we can create pipe if its not last commnsd
                process.stdout(Stdio::piped());
            }
        
            let mut child = process
                .spawn()
                .map_err(|e| e.to_string())?;
        
            
            current_pipe = child.stdout.take(); //  saveing this command's output for next command
        
            children.push(child);
        }
        
        for mut child in children {
            child.wait().map_err(|e| e.to_string())?;
        }
    }
        
    Ok(())
}

//so i think if the program is pwd then rust is asking os to run this prog named pwd and it finds the executable in the path env var and runs it . (confirmed)
//but pwd and cd and exit are builtin => not child processes.. and logically should not be 
// also each process has .stdin() and .stdout() and .stderr() now they are set to their defaults[keyboaed and terminal] but for < > files should be creaeted and specified.