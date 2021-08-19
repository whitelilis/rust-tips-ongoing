#[cfg(test)]
mod test {
    use std::process::Command;

    #[test]
    fn run_system_cmd() {
        let output = Command::new("/usr/bin/touch")
            .arg("/tmp/file.txt")
            .output()
            .expect("failed to execute process");

        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));

        assert!(output.status.success());
    }
}
