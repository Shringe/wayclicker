use std::fmt;

/// Possible commands from the unix socket
pub enum Command {
    IsEnabled,
    SetEnabled { value: bool },
}

/// Errors that can occur when parsing commands
#[derive(Debug)]
pub enum CommandParseError {
    EmptyPacket,
    UnknownCommand(String),
    MissingArgument {
        command: String,
        argument: String,
    },
    InvalidArgument {
        command: String,
        argument: String,
        error: String,
    },
}

impl fmt::Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyPacket => write!(f, "received empty packet"),
            Self::UnknownCommand(cmd) => write!(f, "unknown command: '{}'", cmd),
            Self::MissingArgument { command, argument } => {
                write!(
                    f,
                    "command '{}' missing required argument: {}",
                    command, argument
                )
            }
            Self::InvalidArgument {
                command,
                argument,
                error,
            } => {
                write!(
                    f,
                    "command '{}' has invalid {}: {}",
                    command, argument, error
                )
            }
        }
    }
}

impl std::error::Error for CommandParseError {}

impl Command {
    /// Parses a packet from the unix socket
    pub fn from_packet(packet: String) -> Result<Self, CommandParseError> {
        let mut split_cmd = packet.split_whitespace();

        let command = split_cmd.next().ok_or(CommandParseError::EmptyPacket)?;

        match command {
            "is_enabled" => Ok(Self::IsEnabled),
            "set_enabled" => {
                let value_str =
                    split_cmd
                        .next()
                        .ok_or_else(|| CommandParseError::MissingArgument {
                            command: "set_enabled".to_string(),
                            argument: "value".to_string(),
                        })?;

                let value =
                    value_str
                        .parse::<bool>()
                        .map_err(|e| CommandParseError::InvalidArgument {
                            command: "set_enabled".to_string(),
                            argument: "value".to_string(),
                            error: e.to_string(),
                        })?;

                Ok(Self::SetEnabled { value })
            }
            _ => Err(CommandParseError::UnknownCommand(command.to_string())),
        }
    }
}
