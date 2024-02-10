// Run a command on a system...
use std::process::Command;

pub fn hello() -> String {
    return String::from("Kia ora from execute");
}

#[cfg(test)]
mod tests {

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
}
