use core::time;
use nix::sched::{clone, CloneFlags};
use std::{process::Command, thread};

// TODO: Update this struct to accept the executable and other configuration
// options instead of hardcoding it.
pub struct Container {}

impl Container {
    pub fn new() -> Container {
        return Container {};
    }

    pub unsafe fn run(&self) -> Result<(), ()> {
        const STACK_SIZE: usize = 1024 * 1024;
        let ref mut stack: [u8; STACK_SIZE] = [0; STACK_SIZE];

        let cb = Box::new(|| {
            Command::new("touch")
                .arg("testing.txt")
                .spawn()
                .expect("failed to execute command");

            // temp sleep for testing purposes
            thread::sleep(time::Duration::from_secs(120));

            0
        });

        let flags = CloneFlags::CLONE_NEWPID
            | CloneFlags::CLONE_NEWNS
            | CloneFlags::CLONE_NEWCGROUP
            | CloneFlags::CLONE_NEWIPC
            | CloneFlags::CLONE_NEWNET;

        let signal = None;

        match clone(cb, stack, flags, signal) {
            Ok(pid) => {
                println!("Successful clone. PID: {}", pid);
                Ok(())
            }
            Err(error) => {
                println!("Failed clone: {}", error);
                Err(())
            }
        }
    }
}
