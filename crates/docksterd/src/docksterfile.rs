use std::{error::Error, io};

pub enum DocksterInstruction {
    From(String),
    Run(String),
    Copy { src: String, dest: String },
    Cmd(String),
}

/// Represents a parsed Docksterfile object
pub struct Docksterfile {
    instructions: Vec<DocksterInstruction>,
}

impl Docksterfile {
    pub fn new(path: &str) -> Result<Docksterfile, Box<dyn Error>> {
        let df = Docksterfile {
            instructions: vec![],
        };

        Ok(df)
    }
}

// TODO
fn parse_docksterfile(_input: &str) -> Result<Vec<DocksterInstruction>, io::Error> {
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_path_returns_error() {
        let path = "/invalid_path/Docksterfile";
        let result = Docksterfile::new(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_docksterfile_string() {
        let input = r#"
        FROM ubuntu:latest
        RUN apt update && apt install -y curl
        COPY ./app /app
        CMD ["/app/run.sh"]
        "#;

        let instructions = parse_docksterfile(input).unwrap();

        assert_eq!(instructions.len(), 4);
        assert!(matches!(&instructions[0], DocksterInstruction::From(ref img) if img == "ubuntu:latest"));
        assert!(
            matches!(&instructions[1], DocksterInstruction::Run(ref cmd) if cmd == "apt update && apt install -y curl")
        );
        assert!(
            matches!(&instructions[2], DocksterInstruction::Copy { src, dest } if src == "./app" && dest == "/app")
        );
        assert!(
            matches!(&instructions[3], DocksterInstruction::Cmd(ref cmd) if cmd == "[\"/app/run.sh\"]")
        );
    }
}
