// Run a command on a system...
use std::process::Command;

pub fn hello() -> String {
    return String::from("Kia ora from execute");
}

#[cfg(test)]
mod tests {

    use core::time;
    use std::{os::unix::process::ExitStatusExt, process::Stdio};

    use super::*;

    #[test]
    fn test_can_run_ps() {
        let out = Command::new("ps")
            .args(["-o", "ppid,pid,comm"])
            .output()
            .expect("`ps` process should have returned output");

        println!("Output {:?}", out);
    }

    #[test]
    fn test_can_get_status() {
        let status = Command::new("true")
            .status()
            .expect("/bin/true should have returned a status");

        println!("Return status: {}", status);

        assert!(status.success());
        assert!(status.signal().is_none());
    }

    #[test]
    fn test_can_get_signal() {
        let status = Command::new("sh")
            .args(["-c", "kill -9 $$"])
            .status()
            .expect("should be terminated due to signal");

        println!("Return status: {}", status);

        assert!(!status.success());
        assert!(status.signal().is_some_and(|x| x == 9));
    }

    #[test]
    fn test_stdout_stderr_status() {
        let child = Command::new("sh")
            .args(["-c", "echo goodness; echo badness >&2; exit 2"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("should have spawned a process");

        println!("Child is {:?}", child);

        let output = child
            .wait_with_output()
            .expect("should have returned with output");

        println!("Output is {:?}", output);

        assert!(!output.status.success());
        assert!(output.status.code().is_some_and(|x| x == 2));
        assert!(output.status.signal().is_none());
        assert_eq!(b"goodness\n", output.stdout.as_slice());
        assert_eq!("goodness\n", String::from_utf8(output.stdout).unwrap());
        assert_eq!(b"badness\n", output.stderr.as_slice());
    }

    #[test]
    fn test_kill_after_timeout() {
        let mut child = Command::new("sh")
            .args(["-c", "while sleep 0.01; do echo out; echo err >&2; done"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("should have created a process");

        let pid = child.id();

        println!("Process ID is {}", pid);

        // Check it every 0.1 seconds; kill it after 0.5 seconds.
        let mut run_time = time::Duration::from_millis(0);
        let poll_interval = time::Duration::from_millis(10);
        let kill_after = time::Duration::from_millis(50);

        loop {
            std::thread::sleep(poll_interval);
            run_time += poll_interval;  // bit sloppy; better to get the runtime from the OS
            
            match child.try_wait() {
                Ok(Some(status)) => {
                    // process exited (early in this case, as we expected to kill it)
                    assert!(run_time > kill_after);
                    break;
                }
                Ok(None) => {
                    println!("Process has been executing for {}ms", run_time.as_millis());
                    if run_time > kill_after {
                        println!("Process has been running longer than {}ms, killing", kill_after.as_millis());
                        child.kill().expect("expected to kill process");
                        break;
                    }
                }
                Err(e) => {
                    println!("error attempting to check process: {}", e);
                }
            }
        }

        let output = child
            .wait_with_output()
            .expect("expected to collect process status");

        println!("Output is {}", output.status);

        assert!(output.status.signal().is_some());

    }
}
