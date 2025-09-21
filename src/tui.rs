//! Terminal UI rendering and setup for Termsweeper.
//!
//! This module contains the Tui struct, which handles all terminal-related
//! operations, including setup, rendering of different game states (menu, board),
//! and cleanup.
use crate::error::TermsweeperError;
use crate::game_logic::Board;
use crate::menu::Menu;
use crate::types::{COLOR_CONFIG, CellBox, CellKind, CellState, GameState, MenuItem};
use crossterm::{
    cursor::{MoveTo, RestorePosition},
    event::{self, DisableMouseCapture, EnableMouseCapture},
    style::*,
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
    {execute, queue},
};
use std::io::{Stdout, Write, stdout};

/// A module to hold ASCII art constants.
mod art {
    pub const WIN_ART: [&str; 6] = [
        "__     __          __          ___",
        "\\ \\   / /          \\ \\        / (_)",
        " \\ \\_/ /__  _   _   \\ \\  /\\  / / _ _ __",
        "  \\   / _ \\| | | |   \\ \\/  \\/ / | | '_ \\",
        "   | | (_) | |_| |    \\  /\\  /  | | | | |",
        "   |_|\\___/ \\__,_|     \\/  \\/   |_|_| |_|",
    ];
    pub const LOSE_ART: [&str; 6] = [
        "  _____                         ____       ",
        " / ____|                       / __ \\      ",
        "| |  __  __ _ _ __ ___   ___  | |  | |_   _____ _ __ ",
        "| | |_ |/ _` | '_ ` _ \\ / _ \\ | |  | \\ \\ / / _ \\ '__|",
        "| |__| | (_| | | | | | |  __/ | |__| |\\ V /  __/ |   ",
        " \\_____|\\__,_|_| |_| |_|\\___|  \\____/  \\_/ \\___|_|",
    ];
    pub const TITLE_ART: [&str; 5] = [
        "  _____ _____ ____  __  __ ______        _______ _____ ____  _____ ____  ",
        " |_   _| ____|  _ \\|  \\/  / ___\\ \\      / / ____| ____|  _ \\| ____|  _ \\ ",
        "   | | |  _| | |_) | |\\/| \\___ \\\\ \\ /\\ / /|  _| |  _| | |_) |  _| | |_) |",
        "   | | | |___|  _ <| |  | |___) |\\ V  V / | |___| |___|  __/| |___|  _ < ",
        "   |_| |_____|_| \\_\\_|  |_|____/  \\_/\\_/  |_____|_____|_|   |_____|_| \\_\\",
    ];
}

/// A module for UI layout constants.
mod layout {
    pub const HEADER_HEIGHT: u16 = 3;
    pub const BORDER_HEIGHT: u16 = 2; // Top and bottom borders
    pub const FOOTER_HEIGHT: u16 = 1;
    pub const CELL_WIDTH: u16 = 2;
    pub const BORDER_WIDTH: u16 = 2; // Left and right borders
}

/// Manages the terminal user interface, including setup, rendering, and cleanup.
pub struct Tui {
    stdout: Stdout,
}

impl Tui {
    /// Creates a new `Tui`, enters the alternate screen, enables raw mode,
    /// and sets initial styles.
    pub fn new() -> Result<Self, TermsweeperError> {
        let mut stdout = stdout();
        terminal::enable_raw_mode()?;
        execute!(
            stdout,
            EnterAlternateScreen,
            EnableMouseCapture,
            SetBackgroundColor(COLOR_CONFIG.background),
            SetForegroundColor(Color::Black),
            Clear(terminal::ClearType::All)
        )?;
        Ok(Tui { stdout })
    }

    /// Renders the entire UI based on the current game state.
    /// This is the main drawing function that dispatches to other methods.
    pub fn render(
        &mut self,
        game_state: &GameState,
        board: &Board,
        menu: &Menu,
        mouse_pos: (u16, u16),
    ) -> Result<(), TermsweeperError> {
        match game_state {
            GameState::Menu => self.render_game_menu(menu)?,
            GameState::Ongoing => self.render_game_board(board)?,
            GameState::Won | GameState::Lost => {
                self.render_game_board(board)?;
                self.overlay_ascii_art(board, *game_state == GameState::Won)?;
            }
            GameState::Exit => { /* No rendering needed, will exit loop. */ }
        }
        // Restore the cursor to the last known mouse position for a better feel.
        queue!(self.stdout, MoveTo(mouse_pos.0, mouse_pos.1))?;
        self.stdout.flush()?;
        Ok(())
    }

    /// Renders the game board, including headers, borders, cells, and footers.
    fn render_game_board(&mut self, board: &Board) -> Result<(), TermsweeperError> {
        let (cols, rows) = terminal::size()?;
        let required_width = board.width as u16 * layout::CELL_WIDTH + layout::BORDER_WIDTH;
        let required_height = board.height as u16
            + layout::HEADER_HEIGHT
            + layout::BORDER_HEIGHT
            + layout::FOOTER_HEIGHT;

        queue!(self.stdout, Clear(terminal::ClearType::All))?;

        if cols < required_width || rows < required_height {
            return self.render_terminal_too_small(cols, rows);
        }

        let (start_x, start_y) = board.get_board_start_pos()?;
        self.render_header(
            board,
            start_x,
            start_y - layout::HEADER_HEIGHT,
            required_width,
        )?;
        self.render_board_content(board, start_x, start_y)?;
        self.render_footer(cols, start_y + board.height as u16 + 1)?;

        Ok(())
    }

    /// Renders the main game menu with its title and items.
    fn render_game_menu(&mut self, menu: &Menu) -> Result<(), TermsweeperError> {
        let (cols, rows) = terminal::size()?;
        let art_width = art::TITLE_ART.iter().map(|s| s.len()).max().unwrap_or(0) as u16;
        let art_height = art::TITLE_ART.len() as u16;
        let art_y = (rows.saturating_sub(art_height + menu.len() as u16 + 2)) / 2;
        let art_x = (cols.saturating_sub(art_width)) / 2;

        queue!(self.stdout, Clear(terminal::ClearType::All))?;

        // Draw ASCII art title
        queue!(self.stdout, SetForegroundColor(Color::Black))?;
        for (i, line) in art::TITLE_ART.iter().enumerate() {
            queue!(self.stdout, MoveTo(art_x, art_y + i as u16), Print(line))?;
        }

        self.render_menu_items(menu, cols, art_y + art_height + 1)
    }
}

/// Private implementation block for component rendering.
impl Tui {
    /// Renders the header containing the mine counter and timer.
    fn render_header(
        &mut self,
        board: &Board,
        start_x: u16,
        start_y: u16,
        board_pixel_width: u16,
    ) -> Result<(), TermsweeperError> {
        // Draw bombs counter
        let bombs_left_str = format!("💣: {}", board.get_remaining_mines());
        let counter_box = format_counter_box(&bombs_left_str);
        for (i, line) in counter_box.iter().enumerate() {
            queue!(
                self.stdout,
                SetForegroundColor(COLOR_CONFIG.counter),
                MoveTo(start_x, start_y + i as u16),
                Print(line),
            )?;
        }
        // Draw timer
        let timer_str = format!("⏰: {:02}", board.timer.get_elapsed_seconds());
        let timer_box = format_counter_box(&timer_str);
        let timer_x = start_x + board_pixel_width - (timer_str.len() as u16 + 2);
        for (i, line) in timer_box.iter().enumerate() {
            queue!(
                self.stdout,
                SetForegroundColor(COLOR_CONFIG.counter),
                MoveTo(timer_x, start_y + i as u16),
                Print(line),
            )?;
        }
        Ok(())
    }

    /// Renders the board's frame (borders) and the grid of cells.
    fn render_board_content(
        &mut self,
        board: &Board,
        start_x: u16,
        start_y: u16,
    ) -> Result<(), TermsweeperError> {
        let border_color = SetForegroundColor(COLOR_CONFIG.border);
        // Draw top border
        queue!(
            self.stdout,
            border_color,
            MoveTo(start_x, start_y),
            Print("┌")
        )?;
        for _ in 0..board.width {
            queue!(self.stdout, Print("──"))?;
        }
        queue!(self.stdout, Print("─┐"))?;

        // Draw board rows with cells and side borders
        for y in 0..board.height {
            queue!(
                self.stdout,
                MoveTo(start_x, start_y + 1 + y as u16),
                border_color,
                Print("│ "),
            )?;
            for x in 0..board.width {
                if let Some(cell) = board.get_cell(x as isize, y as isize) {
                    self.render_cell(cell)?;
                }
            }
            queue!(self.stdout, border_color, Print("│"))?;
        }

        // Draw bottom border
        queue!(
            self.stdout,
            border_color,
            MoveTo(start_x, start_y + 1 + board.height as u16),
            Print("└"),
        )?;
        for _ in 0..board.width {
            queue!(self.stdout, Print("──"))?;
        }
        queue!(self.stdout, Print("─┘"))?;

        Ok(())
    }

    /// Renders a single cell on the board.
    fn render_cell(&mut self, cell: &CellBox) -> Result<(), TermsweeperError> {
        let (symbol_str, color) = match cell.state {
            CellState::Hidden => ("■", COLOR_CONFIG.hidden_cell),
            CellState::Flagged => ("⚑", COLOR_CONFIG.flagged_cell),
            CellState::Revealed => match cell.kind {
                CellKind::Mine => ("💣", COLOR_CONFIG.mine),
                CellKind::Number(0) => (" ", COLOR_CONFIG.empty_cell),
                CellKind::Number(n) => {
                    let number_color = COLOR_CONFIG.number[(n as usize).saturating_sub(1).min(7)];
                    (&n.to_string()[..], number_color)
                }
            },
        };

        // Ensure every symbol is formatted to take up 2 character spaces for alignment.
        let display_str = if symbol_str == "💣" {
            format!("{:^1}", symbol_str)
        } else {
            format!("{:^2}", symbol_str)
        };
        queue!(self.stdout, SetForegroundColor(color), Print(display_str))?;

        Ok(())
    }

    /// Renders the footer message with game controls.
    fn render_footer(&mut self, term_width: u16, footer_y: u16) -> Result<(), TermsweeperError> {
        let controls_msg = "Press [m] for menu [r] to restart or [ctrl+c] to exit.";
        let controls_x = (term_width.saturating_sub(controls_msg.len() as u16)) / 2;
        queue!(
            self.stdout,
            MoveTo(controls_x, footer_y + 1),
            SetForegroundColor(Color::DarkGrey),
            Print(controls_msg),
            // Reset colors for the next frame
            SetBackgroundColor(COLOR_CONFIG.background),
            SetForegroundColor(Color::Black),
        )?;
        Ok(())
    }

    /// Renders a warning message when the terminal window is too small.
    fn render_terminal_too_small(&mut self, cols: u16, rows: u16) -> Result<(), TermsweeperError> {
        let msg = "Terminal too small! Please resize.";
        let x = (cols.saturating_sub(msg.len() as u16)) / 2;
        let y = rows / 2;
        queue!(
            self.stdout,
            Clear(terminal::ClearType::All),
            MoveTo(x, y),
            SetForegroundColor(Color::Red),
            Print(msg),
            ResetColor
        )?;
        Ok(())
    }

    /// Renders the selectable items in a menu, highlighting the hovered one.
    fn render_menu_items(
        &mut self,
        menu: &Menu,
        cols: u16,
        start_y: u16,
    ) -> Result<(), TermsweeperError> {
        for (i, item) in menu.items.iter().enumerate() {
            let (label, is_adjustable) = match item {
                MenuItem::Main { name, .. } => (name.to_string(), false),
                MenuItem::Custom { name, value, .. } => (format!("{}: {}", name, value), true),
            };
            let menu_y = start_y + i as u16;
            let menu_x = (cols.saturating_sub(label.len() as u16)) / 2;
            let is_hovered = item == menu.get_hovered_item();

            queue!(self.stdout, MoveTo(menu_x.saturating_sub(2), menu_y))?;

            if is_hovered {
                queue!(self.stdout, SetForegroundColor(Color::Yellow))?;
                if is_adjustable {
                    queue!(self.stdout, Print("< "), Print(label), Print(" >"))?;
                } else {
                    queue!(self.stdout, Print("➤ "), Print(label))?;
                }
            } else {
                queue!(
                    self.stdout,
                    SetForegroundColor(Color::DarkGrey),
                    Print("  "),
                    SetForegroundColor(Color::Black),
                    Print(label)
                )?;
            }
        }
        Ok(())
    }

    /// Overlays ASCII art for win/lose screens.
    fn overlay_ascii_art(&mut self, board: &Board, win: bool) -> Result<(), TermsweeperError> {
        let (cols, rows) = terminal::size()?;

        // Check if terminal is too small for the board.
        let required_width = board.width as u16 * layout::CELL_WIDTH + layout::BORDER_WIDTH;
        let required_height = board.height as u16
            + layout::HEADER_HEIGHT
            + layout::BORDER_HEIGHT
            + layout::FOOTER_HEIGHT;

        if cols < required_width || rows < required_height {
            return Ok(());
        }

        let art = if win { art::WIN_ART } else { art::LOSE_ART };
        let color = if win { Color::Green } else { Color::Red };

        let art_width = art.iter().map(|s| s.len()).max().unwrap_or(0) as u16;
        let art_height = art.len() as u16;
        let art_x = (cols.saturating_sub(art_width)) / 2;

        let (_, board_start_y) = board.get_board_start_pos()?;
        let art_y = if board_start_y >= art_height + 1 {
            board_start_y - art_height - 3
        } else {
            (rows.saturating_sub(art_height)) / 2
        };

        for (i, line) in art.iter().enumerate() {
            // To avoid coloring the blank spaces, we draw char by char
            for (j, ch) in line.chars().enumerate() {
                if !ch.is_whitespace() {
                    queue!(
                        self.stdout,
                        SetForegroundColor(color),
                        MoveTo(art_x + j as u16, art_y + i as u16),
                        Print(ch),
                    )?;
                }
            }
        }
        Ok(())
    }
}

/// The `Drop` trait implementation ensures that the terminal is restored to its
/// original state when the `Tui` instance goes out of scope.
impl Drop for Tui {
    fn drop(&mut self) {
        // Clear any pending events that might interfere with cleanup.
        while let Ok(true) = event::poll(std::time::Duration::from_millis(1)) {
            let _ = event::read();
        }
        // The result is ignored because we cannot handle an error during a drop.
        let _ = execute!(
            self.stdout,
            DisableMouseCapture,
            LeaveAlternateScreen,
            ResetColor,
            RestorePosition,
        );
        let _ = terminal::disable_raw_mode();
    }
}

/// Formats a string value into a simple ASCII box.
fn format_counter_box(value: &str) -> Vec<String> {
    let len = value.chars().count() + 3;
    let top = format!("┌{}┐", "─".repeat(len));
    let middle = format!("│ {} │", value);
    let bottom = format!("└{}┘", "─".repeat(len));
    vec![top, middle, bottom]
}
