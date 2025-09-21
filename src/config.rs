use crate::types::{
    ColorConfig, Constraint, CustomConstraint, GameConfig, LayoutConfig, MenuItem, MenuItemType,
};
use crossterm::style::Color;

pub const LAYOUT_CONFIG: LayoutConfig = LayoutConfig {
    header_height: 3,
    border_height: 2,
    footer_height: 1,
    cell_width: 2,
    border_width: 2,
};

pub const CUSTOM_CONSTRAINT: CustomConstraint = CustomConstraint {
    width: Constraint { min: 5, max: 50 },
    height: Constraint { min: 5, max: 50 },
    mines: Constraint { min: 1, max: 2500 },
};

pub const DEFAULT_CONFIG: GameConfig = GameConfig {
    width: 20,
    height: 20,
    mines: 25,
};

pub const COLOR_CONFIG_CLASSIC: ColorConfig = ColorConfig {
    background: Color::White,
    hidden_cell: Color::DarkGrey,
    flagged_cell: Color::DarkRed,
    mine: Color::Black,
    empty_cell: Color::Black,
    number: [
        Color::Blue,
        Color::Green,
        Color::Red,
        Color::Magenta,
        Color::DarkYellow,
        Color::Cyan,
        Color::White,
        Color::Grey,
    ],
    border: Color::Black,
    counter: Color::Blue,
    menu_text: Color::Black,
    menu_text_hovered: Color::Yellow,
    menu_title_art: Color::Black,
};

// just an idea not implimented yet
pub const COLOR_CONFIG_MODERN: ColorConfig = ColorConfig {
    background: Color::Black,
    hidden_cell: Color::DarkGrey,
    flagged_cell: Color::Yellow,
    mine: Color::DarkRed,
    empty_cell: Color::Black,
    number: [
        Color::Blue,
        Color::Green,
        Color::Red,
        Color::Magenta,
        Color::DarkCyan,
        Color::DarkYellow,
        Color::DarkBlue,
        Color::White,
    ],
    border: Color::Grey,
    counter: Color::Cyan,
    menu_text: Color::Grey,
    menu_text_hovered: Color::Yellow,
    menu_title_art: Color::Blue,
};

pub const MAIN_MENU_ITEMS_LIST: [MenuItem; 5] = [
    MenuItem::Main {
        item_type: MenuItemType::Beginnner,
        name: "Beginner",
        config: Some(GameConfig {
            width: 9,
            height: 9,
            mines: 10,
        }),
    },
    MenuItem::Main {
        item_type: MenuItemType::Intermediate,
        name: "Intermediate",
        config: Some(GameConfig {
            width: 16,
            height: 16,
            mines: 40,
        }),
    },
    MenuItem::Main {
        item_type: MenuItemType::Expert,
        name: "Expert",
        config: Some(GameConfig {
            width: 30,
            height: 16,
            mines: 99,
        }),
    },
    MenuItem::Main {
        item_type: MenuItemType::Custom,
        name: "Custom",
        config: None,
    },
    MenuItem::Main {
        item_type: MenuItemType::Exit,
        name: "Exit",
        config: None,
    },
];

pub const CUSTOM_MENU_ITEMS_LIST: [MenuItem; 4] = [
    MenuItem::Custom {
        item_type: MenuItemType::Width,
        name: "Width",
        value: 30,
        constraints: CUSTOM_CONSTRAINT.width,
    },
    MenuItem::Custom {
        item_type: MenuItemType::Height,
        name: "Height",
        value: 30,
        constraints: CUSTOM_CONSTRAINT.height,
    },
    MenuItem::Custom {
        item_type: MenuItemType::Mines,
        name: "Mines",
        value: 150,
        constraints: CUSTOM_CONSTRAINT.mines,
    },
    MenuItem::Main {
        item_type: MenuItemType::Confirm,
        name: "Confirm",
        config: None,
    },
];

pub mod art {
    pub const WIN_ART: [&str; 6] = [
        r"__     __          __          ___",
        r"\ \   / /          \ \        / (_)",
        r" \ \_/ /__  _   _   \ \  /\  / / _ _ __",
        r"  \   / _ \| | | |   \ \/  \/ / | | '_ \\",
        r"   | | (_) | |_| |    \  /\  /  | | | | |",
        r"   |_|\___/ \__,_|     \/  \/   |_|_| |_|",
    ];
    pub const LOSE_ART: [&str; 6] = [
        r"  _____                         ____       ",
        r" / ____|                       / __ \      ",
        r"| |  __  __ _ _ __ ___   ___  | |  | |_   _____ _ __ ",
        r"| | |_ |/ _` | '_ ` _ \ / _ \ | |  | \ \ / / _ \ '__|",
        r"| |__| | (_| | | | | | |  __/ | |__| |\ V /  __/ |   ",
        r" \_____|\__,_|_| |_| |_|\___|  \____/  \_/ \___|_|",
    ];
    pub const TITLE_ART: [&str; 5] = [
        r"  _____ _____ ____  __  __ ______        _______ _____ ____  _____ ____  ",
        r" |_   _| ____|  _ \|  \/  / ___\ \      / / ____| ____|  _ \| ____|  _ \ ",
        r"   | | |  _| | |_) | |\/| \___ \\ \ /\ / /|  _| |  _| | |_) |  _| | |_) |",
        r"   | | | |___|  _ <| |  | |___) |\ V  V / | |___| |___|  __/| |___|  _ < ",
        r"   |_| |_____|_| \_\_|  |_|____/  \_/\_/  |_____|_____|_|   |_____|_| \_\",
    ];
}
