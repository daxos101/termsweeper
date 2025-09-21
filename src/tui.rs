//! Terminal UI rendering and setup for Termsweeper.
//!
//! This module contains functions for rendering the Minesweeper game board,
//! menus, and handling terminal setup/cleanup using `crossterm`.
use crate::error::TermsweeperError;
use crate::game_logic::Board;
use crate::menu::Menu;
use crate::types::{COLOR_CONFIG, CellKind, CellState, MenuItem};
use crossterm::{
    cursor::{MoveTo, RestorePosition},
    event::{self, DisableMouseCapture, EnableMouseCapture},
    style::*,
    terminal::{self, Clear, EnterAlternateScreen, LeaveAlternateScreen},
    {execute, queue},
};
use std::io::{Stdout, Write};

/// Set up and clean up section
pub fn setup_terminal(mut stdout: &Stdout) -> Result<(), TermsweeperError> {
    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    return Ok(());
}

/// set styles
pub fn set_styles(mut stdout: &Stdout) -> Result<(), TermsweeperError> {
    execute!(
        stdout,
        SetBackgroundColor(COLOR_CONFIG.background),
        SetForegroundColor(Color::Black),
        Clear(terminal::ClearType::All)
    )?;
    return Ok(());
}

/// Restore terminal to original state
pub fn cleanup_terminal(mut stdout: &Stdout) -> Result<(), TermsweeperError> {
    while event::poll(std::time::Duration::from_millis(1))? {
        let _ = event::read(); // Clear any pending events
    }
    execute!(
        stdout,
        DisableMouseCapture,
        LeaveAlternateScreen,
        ResetColor,
        RestorePosition,
    )?;
    terminal::disable_raw_mode()?;
    return Ok(());
}

/// Overlay ASCII art above the finished game board for win/lose screens.
/// If there is room, place it above the board; otherwise, center in terminal.
pub fn overlay_ascii_art(
    stdout: &mut Stdout,
    board: &Board,
    win: bool,
) -> Result<(), TermsweeperError> {
    let win_art = [
        "__     __          __          ___",
        "\\ \\   / /          \\ \\        / (_)",
        " \\ \\_/ /__  _   _   \\ \\  /\\  / / _ _ __",
        "  \\   / _ \\| | | |   \\ \\/  \\/ / | | '_ \\",
        "   | | (_) | |_| |    \\  /\\  /  | | | | |",
        "   |_|\\___/ \\__,_|     \\/  \\/   |_|_| |_|",
    ];
    let lose_art = [
        "  _____                         ____       ",
        " / ____|                       / __ \\      ",
        "| |  __  __ _ _ __ ___   ___  | |  | |_   _____ _ __ ",
        "| | |_ |/ _` | '_ ` _ \\ / _ \\ | |  | \\ \\ / / _ \\ '__|",
        "| |__| | (_| | | | | | |  __/ | |__| |\\ V /  __/ |   ",
        " \\_____|\\__,_|_| |_| |_|\\___|  \\____/  \\_/ \\___|_|",
    ];
    let art = if win { &win_art } else { &lose_art };
    let color = if win { Color::Green } else { Color::Red };

    let (cols, rows) = crossterm::terminal::size()?;
    let art_width = art.iter().map(|s| s.len()).max().unwrap_or(0) as u16;
    let art_height = art.len() as u16;
    let art_x = (cols.saturating_sub(art_width)) / 2;

    let (_, board_start_y) = board.get_board_start_pos()?;
    let art_y = if board_start_y >= art_height + 1 {
        board_start_y - art_height - 3
    } else {
        (rows.saturating_sub(art_height)) / 2
    };
    queue!(stdout, SetAttribute(Attribute::Bold))?;
    for (i, line) in art.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch != ' ' {
                queue!(
                    stdout,
                    SetForegroundColor(color),
                    MoveTo(art_x + j as u16, art_y + i as u16),
                    Print(ch),
                )?;
            }
        }
    }
    queue!(
        stdout,
        SetBackgroundColor(COLOR_CONFIG.background),
        // SetAttribute(Attribute::NoBold)
    )?;
    stdout.flush()?;
    Ok(())
}

// put into seperate function to avoid code duplication and make more readable
/// Render the game board to the terminal using crossterm.
/// Handles terminal resizing and displays a warning if the terminal is too small.
pub fn render_game_board(board: &Board, stdout: &mut Stdout) -> Result<(), TermsweeperError> {
    let (cols, rows) = crossterm::terminal::size()?;
    // Should remove magic numbers here.
    let required_width = 2 + board.width * 2;
    let required_height = 6 + board.height; // 3 for counters, 1 for top border, 1 for bottom border, 1 for controls message

    if cols < required_width as u16 || rows < required_height as u16 {
        // Terminal is too small, show warning message
        let msg = "Terminal too small! Resize and try again.";
        let x = (cols.saturating_sub(msg.len() as u16)) / 2;
        let y = rows / 2;
        queue!(
            stdout,
            Clear(terminal::ClearType::All),
            MoveTo(x, y),
            SetForegroundColor(Color::Red),
            Print(msg),
            ResetColor
        )?;
        stdout.flush()?;
        return Ok(());
    }

    let (board_start_x, board_start_y) = board.get_board_start_pos()?;
    queue!(
        stdout,
        SetBackgroundColor(COLOR_CONFIG.background),
        Clear(terminal::ClearType::All),
        MoveTo(board_start_x, board_start_y)
    )?;
    // Draw bombs counter
    let bombs_left = board.get_remaining_mines();
    let bombs_left_str = format!("💣: {}", bombs_left);
    let counter_box = format_box_with_value(&bombs_left_str);
    for (i, line) in counter_box.iter().enumerate() {
        queue!(
            stdout,
            SetForegroundColor(COLOR_CONFIG.counter),
            MoveTo(board_start_x, board_start_y - 3 + i as u16),
            Print(line),
        )?;
    }
    // Draw timer
    let elapsed_seconds = board.timer.get_elapsed_seconds();
    let timer_str = format!("⏰: {:02}", elapsed_seconds);
    let timer_box = format_box_with_value(&timer_str);
    for (i, line) in timer_box.iter().enumerate() {
        queue!(
            stdout,
            SetForegroundColor(COLOR_CONFIG.counter),
            MoveTo(
                board_start_x + required_width as u16 - timer_str.len() as u16 - 2,
                board_start_y - 3 + i as u16
            ),
            Print(line),
        )?;
    }
    // Draw top border
    queue!(
        stdout,
        SetForegroundColor(COLOR_CONFIG.border),
        MoveTo(board_start_x, board_start_y),
        Print("┌"),
    )?;
    for _ in 0..board.width {
        queue!(stdout, Print("──"))?;
    }
    queue!(stdout, Print("─┐"))?;

    // Draw board rows with left/right borders
    for y in 0..board.height {
        queue!(
            stdout,
            MoveTo(board_start_x, board_start_y + 1 + y as u16),
            Print("│ "),
        )?;
        for x in 0..board.width {
            // Render cells with offset for border
            if let Some(cell) = board.get_cell(x as isize, y as isize) {
                let (symbol, color) = match cell.state {
                    CellState::Hidden => ("■", COLOR_CONFIG.hidden_cell),
                    CellState::Flagged => ("⚑", COLOR_CONFIG.flagged_cell),
                    CellState::Revealed => match cell.kind {
                        CellKind::Mine => ("💣", COLOR_CONFIG.mine),
                        CellKind::Number(0) => ("  ", COLOR_CONFIG.empty_cell),
                        CellKind::Number(n) => (
                            &n.to_string()[..],
                            COLOR_CONFIG.number[(n as usize).saturating_sub(1).min(7)],
                        ),
                    },
                };
                let symbol = if symbol == "💣" {
                    format!("{:>1}", symbol)
                } else {
                    format!("{:^2}", symbol)
                };
                queue!(stdout, SetForegroundColor(color), Print(symbol))?;
            }
        }
        queue!(stdout, SetForegroundColor(COLOR_CONFIG.border), Print("│"))?;
    }

    // Draw bottom border
    queue!(
        stdout,
        SetForegroundColor(COLOR_CONFIG.border),
        MoveTo(board_start_x, board_start_y + 1 + board.height as u16),
        Print("└"),
    )?;
    for _ in 0..board.width {
        queue!(stdout, Print("──"))?;
    }
    queue!(stdout, Print("─┘"))?;
    let controls_msg = "Press [m] for menu [r] to restart or [ctrl+c] to exit.";
    let controls_x = (cols.saturating_sub(controls_msg.len() as u16)) / 2 + 1;
    queue!(
        stdout,
        MoveTo(controls_x, board_start_y + 2 + board.height as u16),
        SetForegroundColor(Color::DarkGrey),
        Print(controls_msg),
        SetBackgroundColor(COLOR_CONFIG.background),
        SetForegroundColor(Color::Black),
    )?;
    stdout.flush()?;
    Ok(())
}

fn format_box_with_value(value: &str) -> Vec<String> {
    let mut len = value.len();
    if value.contains("⏰") {
        len += 1;
    }
    let top_bottom = format!("┌{}┐\n", "─".repeat(len));
    let middle = format!("│ {} │\n", value);
    let bottom = format!("└{}┘", "─".repeat(len));
    vec![top_bottom, middle, bottom]
}

/// Render the main game menu with ASCII art title and menu items.
/// Centers the menu in the terminal and highlights the hovered item.
pub fn render_game_menu(stdout: &mut Stdout, menu: &Menu) -> Result<(), TermsweeperError> {
    let title_art = [
        "  _____ _____ ____  __  __ ______        _______ _____ ____  _____ ____  ",
        " |_   _| ____|  _ \\|  \\/  / ___\\ \\      / / ____| ____|  _ \\| ____|  _ \\ ",
        "   | | |  _| | |_) | |\\/| \\___ \\\\ \\ /\\ / /|  _| |  _| | |_) |  _| | |_) |",
        "   | | | |___|  _ <| |  | |___) |\\ V  V / | |___| |___|  __/| |___|  _ < ",
        "   |_| |_____|_| \\_\\_|  |_|____/  \\_/\\_/  |_____|_____|_|   |_____|_| \\_\\",
    ];

    let (cols, rows) = crossterm::terminal::size()?;
    let art_width = title_art.iter().map(|s| s.len()).max().unwrap_or(0) as u16;
    let art_height = title_art.len() as u16;
    let art_y = (rows.saturating_sub(art_height + menu.len() as u16 + 2)) / 2;
    let art_x = (cols.saturating_sub(art_width)) / 2;

    queue!(
        stdout,
        Clear(terminal::ClearType::All),
        SetForegroundColor(Color::Black),
    )?;
    // Draw ASCII art title
    for (i, line) in title_art.iter().enumerate() {
        queue!(stdout, MoveTo(art_x, art_y + i as u16), Print(line),)?;
    }
    render_menu(stdout, menu, cols, art_height, art_y)?;
    Ok(())
}

/// Render the menu items, highlighting the hovered item.
/// Centers the menu items below the ASCII art title.
pub fn render_menu(
    stdout: &mut Stdout,
    menu: &Menu,
    cols: u16,
    art_height: u16,
    art_y: u16,
) -> Result<(), TermsweeperError> {
    for (i, item) in menu.items.iter().enumerate() {
        let (label, is_adjustable) = match item {
            MenuItem::Main { name, .. } => (name.to_string(), false),
            MenuItem::Custom { name, value, .. } => (format!("{}: {}", name, value), true),
        };
        let menu_y = art_y + art_height + 1 + i as u16;
        let menu_x = (cols.saturating_sub(label.len() as u16)) / 2;
        let is_hovered = item == menu.get_hovered_item();
        if is_hovered {
            // menu_x - 2 because of the arrow needs a place
            queue!(
                stdout,
                MoveTo(menu_x - 2, menu_y),
                SetForegroundColor(Color::Yellow),
            )?;
            if is_adjustable {
                queue!(stdout, Print("< "), Print(label), Print(" >"))?;
            } else {
                queue!(stdout, Print("➤ "), Print(label),)?;
            }
        } else {
            queue!(
                stdout,
                MoveTo(menu_x - 2, menu_y),
                SetForegroundColor(Color::DarkGrey),
                Print("  "),
                SetForegroundColor(Color::Black),
                Print(label),
            )?;
        }
    }
    stdout.flush()?;
    Ok(())
}
