use nix::unistd::{fork, ForkResult};
use std::io::{self, BufRead, Write};
use std::process::{exit, Command};

pub fn forker(){
    let stdin = io::stdin(); //input
    print!("$: ");
    let _ = io::stdout().flush(); // to print before reading
    loop {
        // infinite loop
        let line = stdin
            .lock()
            .lines()
            .next()
            .expect("there was no next line")
            .expect("the line could not be read");

        let mut v: Vec<&str> = line.split(' ').collect(); //input line

        let str: &str = v[0]; //first argument

        v.remove(0); //remove first argument

        let _ = match unsafe{fork()} {
            // child
            Ok(ForkResult::Child) => {
                // exec
                let mut child = Command::new(str)
                    .args(&v)
                    .spawn()
                    .expect("command failed to start");
                let _result = child.wait();
                print!("$: ");
                exit(0);
            }

            // parent
            Ok(ForkResult::Parent { child: _, .. }) => {}

            //error
            Err(err) => {
                panic!("[main] fork() failed: {}", err);
            }
        };
    }
}