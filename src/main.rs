use std::os::raw::{c_int, c_void};
use libc::{fork, waitpid, WIFEXITED, WEXITSTATUS};
use std::process;

const SIZE: usize = 5;

fn main() {
    let mut nums: [i32; SIZE] = [0, 1, 2, 3, 4];

    let pid = unsafe { fork() };

    if pid == 0 {
        for i in 0..SIZE {
            nums[i] *= -(i as i32);
            println!("CHILD: {}", nums[i]);
        }
    } else if pid > 0 {
        let mut status: c_int = 0;
        let result = unsafe { waitpid(pid, &mut status, 0) };

        if result > 0 && unsafe { WIFEXITED(status) } {
            println!("Child process exited with status: {}", unsafe { WEXITSTATUS(status) });
        }

        for i in 0..SIZE {
            println!("PARENT: {}", nums[i]);
        }
    }
}