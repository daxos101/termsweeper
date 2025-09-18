use crate::types::GameConfig;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TermsweeperError {
    #[error("Terminal I/O error")]
    Io(#[from] io::Error),

    #[error("Invalid game configuration: {0}")]
    InvalidConfig(GameConfig),

    #[error("Custom game configuration is incomplete")]
    IncompleteCustomConfig,

    #[error("Clicked outside the board boundaries")]
    ClickOutOfBounds,
}
