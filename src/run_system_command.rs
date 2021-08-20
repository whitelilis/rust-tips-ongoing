#[cfg(test)]
mod test {
    use std::process::Command;

    #[test]
    fn run_system_cmd() {
        let output = Command::new("/bin/sh")
            .arg("-c")
            .arg("python3 /tmp/aaa/main.py")
            .output()
            .expect("failed to execute process");

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        assert!(output.status.success());
    }
}
