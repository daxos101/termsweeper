use crate::config::art;
use crate::config::{CUSTOM_MENU_ITEMS_LIST, MAIN_MENU_ITEMS_LIST, THEME_MENU_ITEMS_LIST};
use crate::error::TermsweeperError;
use crate::game_logic::Board;
use crate::types::{ColorConfig, GameConfig, GameState, MenuItem, MenuItemType};
use crossterm::event::{Event, KeyCode, MouseButton, MouseEvent, MouseEventKind};
use crossterm::terminal;

/// Represents a menu in the Minesweeper game.
///
/// Holds the list of menu items, the currently hovered index, and the selected item.
#[derive(Debug, Clone)]
pub struct Menu {
    /// List of menu items displayed in the menu.
    pub items: Vec<MenuItem>,
    /// Index of the currently hovered menu item.
    pub hovered_index: usize,
    /// The currently selected menu item, if any.
    pub selected: Option<MenuItem>,
}

impl Menu {
    /// Creates a new menu from a list of menu items.
    pub fn new(items: Vec<MenuItem>) -> Menu {
        Menu {
            items,
            hovered_index: 0,
            selected: None,
        }
    }

    /// Creates the main menu with predefined items.
    pub fn new_main_menu() -> Menu {
        Menu::new(MAIN_MENU_ITEMS_LIST.to_vec())
    }

    /// Creates the custom configuration menu.
    pub fn new_custom_menu() -> Menu {
        Menu::new(CUSTOM_MENU_ITEMS_LIST.to_vec())
    }

    /// Creates the themes menu.
    pub fn new_themes_menu() -> Menu {
        Menu::new(THEME_MENU_ITEMS_LIST.to_vec())
    }

    /// Returns a reference to the currently hovered menu item.
    pub fn get_hovered_item(&self) -> &MenuItem {
        &self.items[self.hovered_index]
    }

    /// Selects the currently hovered menu item.
    pub fn select(&mut self) {
        let selected_item = self.get_hovered_item().clone();
        self.selected = Some(selected_item);
    }

    /// Moves the hovered index to the next item.
    pub fn next(&mut self) {
        self.hovered_index = (self.hovered_index + 1) % self.items.len();
    }

    /// Moves the hovered index to the previous item.
    pub fn previous(&mut self) {
        if self.hovered_index == 0 {
            self.hovered_index = self.items.len() - 1;
        } else {
            self.hovered_index -= 1;
        }
    }

    /// Returns the number of items in the menu.
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns a custom game configuration if all values are set.
    pub fn get_custom_config(&self) -> Result<GameConfig, TermsweeperError> {
        let mut config = GameConfig {
            width: 0,
            height: 0,
            mines: 0,
        };
        for item in &self.items {
            if let MenuItem::Custom {
                item_type, value, ..
            } = item
            {
                match item_type {
                    MenuItemType::Width => config.width = *value,
                    MenuItemType::Height => config.height = *value,
                    MenuItemType::Mines => config.mines = *value,
                    _ => {}
                }
            }
        }
        if config.width > 0 && config.height > 0 && config.mines > 0 {
            Ok(config)
        } else {
            // right now it is complete by default, so this should never happen
            // but in the future if we change the logic it might
            Err(TermsweeperError::IncompleteCustomConfig)
        }
    }
    /// Handles keyboard events for menu navigation and selection.
    ///
    /// Supports up/down navigation, selection, and value adjustment for custom menu items.
    pub fn handle_menu_event(&mut self, event: &Event) {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Down => {
                    self.next();
                }
                KeyCode::Up => {
                    self.previous();
                }
                KeyCode::Enter => {
                    self.select();
                }
                KeyCode::Right => {
                    // Increase value for custom menu item
                    if let MenuItem::Custom {
                        item_type,
                        name,
                        value,
                        constraints,
                    } = self.items[self.hovered_index]
                    {
                        let new_value = value.saturating_add(1).min(constraints.max);
                        self.items[self.hovered_index] = MenuItem::Custom {
                            item_type,
                            name,
                            value: new_value,
                            constraints,
                        };
                    }
                }
                KeyCode::Left => {
                    // Decrease value for custom menu item
                    if let MenuItem::Custom {
                        item_type,
                        name,
                        value,
                        constraints,
                    } = self.items[self.hovered_index]
                    {
                        let new_value = value.saturating_sub(1).max(constraints.min);
                        self.items[self.hovered_index] = MenuItem::Custom {
                            item_type,
                            name,
                            value: new_value,
                            constraints,
                        };
                    }
                }
                _ => {}
            }
        }
        if let Event::Mouse(mouse_event) = event {
            self.handle_mouse_event(mouse_event);
        }
    }

    fn handle_mouse_event(&mut self, event: &MouseEvent) {
        match event.kind {
            MouseEventKind::Down(MouseButton::Left) => self.select(),
            MouseEventKind::Moved => {
                if let Ok((cols, rows)) = terminal::size() {
                    let art_height = art::TITLE_ART.len() as u16;
                    let start_y = (rows.saturating_sub(art_height + self.len() as u16 + 2)) / 2
                        + art_height
                        + 1;

                    for (i, item) in self.items.iter().enumerate() {
                        let label = match item {
                            MenuItem::Main { name, .. } => name.to_string(),
                            MenuItem::Custom { name, value, .. } => {
                                format!("{}: {}", name, value)
                            }
                            MenuItem::Theme { name, .. } => name.to_string(),
                        };

                        let item_y = start_y + i as u16;
                        let menu_x = (cols.saturating_sub(label.len() as u16)) / 2;

                        // This is the full clickable area including prefixes like "➤ " or "< "
                        let start_x = menu_x.saturating_sub(2);
                        // And suffixes like " >"
                        let end_x = menu_x + label.len() as u16 + 2;

                        if event.row == item_y && event.column >= start_x && event.column < end_x {
                            self.hovered_index = i;
                            return;
                        }
                    }
                }
            }
            MouseEventKind::ScrollUp => {
                if let MenuItem::Custom {
                    item_type,
                    name,
                    value,
                    constraints,
                } = self.items[self.hovered_index]
                {
                    let new_value = value.saturating_add(1).min(constraints.max);
                    self.items[self.hovered_index] = MenuItem::Custom {
                        item_type,
                        name,
                        value: new_value,
                        constraints,
                    };
                }
            }
            MouseEventKind::ScrollDown => {
                if let MenuItem::Custom {
                    item_type,
                    name,
                    value,
                    constraints,
                } = self.items[self.hovered_index]
                {
                    let new_value = value.saturating_sub(1).max(constraints.min);
                    self.items[self.hovered_index] = MenuItem::Custom {
                        item_type,
                        name,
                        value: new_value,
                        constraints,
                    };
                }
            }
            _ => {}
        }
    }
}

/// Processes the selected menu item and updates the game state accordingly.
///
/// Handles starting new games, switching to custom menu, exiting, and confirming custom configuration.
pub fn process_menu_selection(
    current_menu: &mut Menu,
    board: &mut Board,
    game_state: &mut GameState,
    current_color_config: &mut ColorConfig,
) -> Result<bool, TermsweeperError> {
    let mut theme_changed = false;
    if let Some(item) = current_menu.selected {
        match item {
            MenuItem::Main {
                item_type, config, ..
            } => match item_type {
                MenuItemType::Beginnner | MenuItemType::Intermediate | MenuItemType::Expert => {
                    *board = Board::new_with_config(config.unwrap())?;
                    *game_state = GameState::Ongoing;
                }
                MenuItemType::Custom => {
                    *current_menu = Menu::new_custom_menu();
                }
                MenuItemType::Themes => {
                    *current_menu = Menu::new_themes_menu();
                }
                MenuItemType::Exit => {
                    *game_state = GameState::Exit; // Or some exit state
                }
                MenuItemType::Confirm => {
                    if let Ok(config) = current_menu.get_custom_config() {
                        *board = Board::new_with_config(Board::clamp_config(
                            config.width,
                            config.height,
                            config.mines,
                        ))?;
                        *game_state = GameState::Ongoing;
                    } else {
                        // Invalid config, maybe reset selection or show error
                        current_menu.selected = None; // Clear selection to stay in menu
                    }
                }
                _ => {}
            },
            MenuItem::Custom { .. } => {
                // Do nothing for now
            }
            MenuItem::Theme { color_config, .. } => {
                *current_color_config = color_config;
                *current_menu = Menu::new_main_menu();
                theme_changed = true;
            }
        }
    }
    Ok(theme_changed)
}
