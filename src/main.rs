use termsweeper::{
    config::CLASSIC_THEME,
    error::TermsweeperError,
    game_logic::Board,
    menu::{self, Menu},
    tui::Tui,
    types::GameState,
};

use crossterm::cursor::MoveTo;
use crossterm::event::{self, Event, KeyCode, KeyModifiers, MouseButton};
use crossterm::execute;
use std::io::stdout;

fn should_exit(event: &Event) -> bool {
    matches!(
        event,
        Event::Key(key_event) if key_event.code == KeyCode::Esc
            || (key_event.code == KeyCode::Char('c') && key_event.modifiers == KeyModifiers::CONTROL)
    )
}

fn should_restart(event: &Event) -> bool {
    matches!(
        event,
        Event::Key(key_event) if key_event.code == KeyCode::Char('r')
    )
}

fn should_menu(event: &Event) -> bool {
    matches!(
        event,
        Event::Key(key_event) if key_event.code == KeyCode::Char('m')
    )
}

fn main() -> Result<(), TermsweeperError> {
    let mut stdout = stdout();
    let mut current_color_config = &mut CLASSIC_THEME.clone();
    let mut tui = Tui::new(current_color_config)?;

    // The main logic is wrapped in a closure so we can handle its result
    // before the final cleanup.
    let result = (|| {
        let mut board = Board::new();
        let mut game_state = GameState::Menu;
        let mut mouse_position = (0, 0);
        let main_menu = Menu::new_main_menu();
        let mut current_menu = Box::new(main_menu);
        'game_loop: loop {
            tui.render(
                &game_state,
                &board,
                &current_menu,
                mouse_position,
                current_color_config,
            )?;
            execute!(&mut stdout, MoveTo(mouse_position.0, mouse_position.1))?;
            // Wait for event, but only up to 100ms
            if let Ok(false) = event::poll(std::time::Duration::from_millis(100)) {
                if game_state == GameState::Ongoing {
                    continue;
                }
            }
            let event = event::read()?;
            if should_exit(&event) {
                break 'game_loop;
            }
            if should_menu(&event) && game_state != GameState::Menu {
                game_state = GameState::Menu;
                current_menu = Box::new(menu::Menu::new_main_menu());
                continue;
            }
            if should_restart(&event) && game_state == GameState::Ongoing {
                board.reset();
                continue;
            }
            match game_state {
                GameState::Menu => {
                    menu::handle_menu_event(&event, &mut *current_menu);
                    if menu::process_menu_selection(
                        &mut *current_menu,
                        &mut board,
                        &mut game_state,
                        &mut current_color_config,
                    )? {
                        tui.clear_term()?; // clear old theme
                    };
                    if game_state == GameState::Ongoing {
                        continue;
                    }
                    if game_state == GameState::Lost {
                        break 'game_loop;
                    }
                }
                GameState::Ongoing => {
                    if let Event::Mouse(mouse_event) = event {
                        match mouse_event.kind {
                            event::MouseEventKind::Down(MouseButton::Left) => {
                                if let Some(new_state) = board.handle_mouse_left(mouse_event) {
                                    game_state = new_state;
                                }
                            }
                            event::MouseEventKind::Down(MouseButton::Right) => {
                                board.handle_mouse_right(mouse_event);
                            }
                            _ => {}
                        }
                    }
                }
                GameState::Won | GameState::Lost => {
                    board.reveal_all_mines();
                    if should_restart(&event) {
                        game_state = GameState::Ongoing;
                        board.reset();
                    } else if should_menu(&event) {
                        game_state = GameState::Menu;
                        current_menu = Box::new(menu::Menu::new_main_menu());
                    }
                }
                GameState::Exit => {
                    break 'game_loop;
                }
            }
            if let Event::Mouse(mouse_event) = &event {
                mouse_position = (mouse_event.column, mouse_event.row);
            }
        }
        Ok(())
    })(); // Immediately execute the closure

    result // Return the result of the game loop.
}
