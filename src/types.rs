use crossterm::style::Color;
use std::fmt::Display;

///! Types for termsweeper.
///!
///! This module defines the core data structures and types used throughout the application.

// --- Core Game State and Board Types ---

/// Represents the overall state of the game.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameState {
    Menu,
    Ongoing,
    Won,
    Lost,
    Exit,
}

/// Represents the state of a cell on the board.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellState {
    Hidden,
    Revealed,
    Flagged,
}

/// Represents the kind of a cell (mine or number).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellKind {
    Mine,
    Number(u8),
}

/// Represents a cell on the board.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellBox {
    pub kind: CellKind,
    pub state: CellState,
}

// --- Configuration-related Types ---

pub struct LayoutConfig {
    pub header_height: u16,
    pub border_height: u16,
    pub footer_height: u16,
    pub cell_width: u16,
    pub border_width: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GameConfig {
    pub width: usize,
    pub height: usize,
    pub mines: usize,
}

impl Display for GameConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}x{} with {} mines",
            self.width, self.height, self.mines
        )
    }
}

pub struct ColorConfig {
    pub background: Color,
    pub hidden_cell: Color,
    pub flagged_cell: Color,
    pub mine: Color,
    pub empty_cell: Color,
    pub number: [Color; 8],
    pub border: Color,
    pub counter: Color,
    pub menu_text: Color,
    pub menu_text_hovered: Color,
    pub menu_title_art: Color,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Constraint {
    pub min: usize,
    pub max: usize,
}

pub struct CustomConstraint {
    pub width: Constraint,
    pub height: Constraint,
    pub mines: Constraint,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuItemType {
    Beginnner,
    Intermediate,
    Expert,
    Custom,
    Exit,
    Width,
    Height,
    Mines,
    Confirm,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuItem {
    Main {
        item_type: MenuItemType,
        name: &'static str,
        config: Option<GameConfig>,
    },
    Custom {
        item_type: MenuItemType,
        name: &'static str,
        value: usize,
        constraints: Constraint,
    },
}
