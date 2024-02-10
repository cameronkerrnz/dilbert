// Run a command on a system...
use std::process::Command;

pub fn hello() -> String {
    return String::from("Kia ora from execute");
}

#[cfg(test)]
mod tests {

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

        assert!(status.success());
    }
}
