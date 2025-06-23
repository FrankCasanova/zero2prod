
fn main() {
    println!("thi is the started project");
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    #[test]
    fn test_main_output() {
        let output = Command::new(std::env::current_exe().unwrap())
            .output()
            .expect("Failed to execute process");
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("thi is the started project"));
    }
}
