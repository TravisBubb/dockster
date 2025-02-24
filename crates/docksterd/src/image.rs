use std::io;

// TODO
fn run_command(_cmd: &str) -> Result<String, io::Error> {
    Err(io::Error::new(io::ErrorKind::Other, "Not implemented"))
}

// TODO
fn copy_file(_src: &str, _dest: &str) -> Result<(), io::Error> {
    Err(io::Error::new(io::ErrorKind::Other, "Not implemented"))
}

// TODO
fn create_base_image(_image_name: &str) -> Result<(), io::Error> {
    Err(io::Error::new(io::ErrorKind::Other, "Not implemented"))
}

// TODO
fn package_image(_image_name: &str) -> Result<(), io::Error> {
    Err(io::Error::new(io::ErrorKind::Other, "Not implemented"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_command() {
        let result = run_command("echo Hello Dockster");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello Dockster\n");
    }

    #[test]
    fn test_run_invalid_command() {
        let result = run_command("invalid_command_that_fails");
        assert!(result.is_err())
    }

    #[test]
    fn test_copy_file() {
        let src = "test_data/sample.txt";
        let dest = "/tmp/dockster_test/sample.txt";

        std::fs::write(src, "Test content").unwrap();

        let result = copy_file(src, dest);
        assert!(result.is_ok());
        assert!(std::fs::metadata(dest).is_ok());

        std::fs::remove_file(src).unwrap();
        std::fs::remove_file(dest).unwrap();
    }

    #[test]
    fn test_copy_nonexistent_file() {
        let result = copy_file("nonexistent.txt", "/tmp/destination.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_create_base_image() {
        let result = create_base_image("ubuntu:latest");
        assert!(result.is_ok());
        assert!(std::fs::metadata("/var/lib/dockster/images/ubuntu_latest").is_ok());
    }

    #[test]
    fn test_package_image() {
        let result = package_image("ubuntu:latest");
        assert!(result.is_ok());
        assert!(
            std::fs::metadata("/var/lib/dockster/images/ubuntu_latest/ubuntu_latest.tar.gz")
                .is_ok()
        );
    }
}
