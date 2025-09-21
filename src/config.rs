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
    width: Constraint { min: 8, max: 50 },
    height: Constraint { min: 5, max: 50 },
    mines: Constraint { min: 1, max: 2500 },
};

pub const DEFAULT_CONFIG: GameConfig = GameConfig {
    width: 20,
    height: 20,
    mines: 25,
};

pub const MAIN_MENU_ITEMS_LIST: [MenuItem; 6] = [
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
        item_type: MenuItemType::Themes,
        name: "Themes",
        config: None,
    },
    MenuItem::Main {
        item_type: MenuItemType::Exit,
        name: "Exit",
        config: None,
    },
];

pub const THEME_MENU_ITEMS_LIST: [MenuItem; 6] = [
    MenuItem::Theme {
        item_type: MenuItemType::Theme,
        name: "Classic",
        color_config: CLASSIC_THEME,
    },
    MenuItem::Theme {
        item_type: MenuItemType::Theme,
        name: "Modern",
        color_config: MODERN_THEME,
    },
    MenuItem::Theme {
        item_type: MenuItemType::Theme,
        name: "Catppuccin Latte",
        color_config: CATPPUCCIN_LATTE_THEME,
    },
    MenuItem::Theme {
        item_type: MenuItemType::Theme,
        name: "Catppuccin Frappe",
        color_config: CATPPUCCIN_FRAPPE_THEME,
    },
    MenuItem::Theme {
        item_type: MenuItemType::Theme,
        name: "Catppuccin Macchiato",
        color_config: CATPPUCCIN_MACCHIATO_THEME,
    },
    MenuItem::Theme {
        item_type: MenuItemType::Theme,
        name: "Catppuccin Mocha",
        color_config: CATPPUCCIN_MOCHA_THEME,
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

pub const CLASSIC_THEME: ColorConfig = ColorConfig {
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

pub const MODERN_THEME: ColorConfig = ColorConfig {
    background: Color::Rgb {
        r: 30,
        g: 34,
        b: 46,
    },
    hidden_cell: Color::Rgb {
        r: 76,
        g: 86,
        b: 106,
    },
    flagged_cell: Color::Rgb {
        r: 235,
        g: 203,
        b: 139,
    },
    mine: Color::Rgb {
        r: 191,
        g: 97,
        b: 106,
    },
    empty_cell: Color::Rgb {
        r: 46,
        g: 52,
        b: 64,
    },
    number: [
        Color::Rgb {
            r: 94,
            g: 129,
            b: 172,
        },
        Color::Rgb {
            r: 143,
            g: 188,
            b: 187,
        },
        Color::Rgb {
            r: 208,
            g: 135,
            b: 112,
        },
        Color::Rgb {
            r: 180,
            g: 142,
            b: 173,
        },
        Color::Rgb {
            r: 191,
            g: 97,
            b: 106,
        },
        Color::Rgb {
            r: 136,
            g: 192,
            b: 208,
        },
        Color::Rgb {
            r: 235,
            g: 203,
            b: 139,
        },
        Color::Rgb {
            r: 236,
            g: 239,
            b: 244,
        },
    ],
    border: Color::Rgb {
        r: 94,
        g: 129,
        b: 172,
    },
    counter: Color::Rgb {
        r: 136,
        g: 192,
        b: 208,
    },
    menu_text: Color::Rgb {
        r: 216,
        g: 222,
        b: 233,
    },
    menu_text_hovered: Color::Rgb {
        r: 235,
        g: 203,
        b: 139,
    },
    menu_title_art: Color::Rgb {
        r: 136,
        g: 192,
        b: 208,
    },
};

// Catppuccin Latte

pub const CATPPUCCIN_LATTE_THEME: ColorConfig = ColorConfig {
    background: Color::Rgb {
        r: 239,
        g: 241,
        b: 245,
    },
    hidden_cell: Color::Rgb {
        r: 172,
        g: 176,
        b: 190,
    },
    flagged_cell: Color::Rgb {
        r: 220,
        g: 138,
        b: 120,
    },
    mine: Color::Rgb {
        r: 210,
        g: 15,
        b: 57,
    },
    empty_cell: Color::Rgb {
        r: 204,
        g: 208,
        b: 218,
    },
    number: [
        Color::Rgb {
            r: 136,
            g: 173,
            b: 244,
        },
        Color::Rgb {
            r: 166,
            g: 218,
            b: 149,
        },
        Color::Rgb {
            r: 237,
            g: 135,
            b: 150,
        },
        Color::Rgb {
            r: 203,
            g: 166,
            b: 247,
        },
        Color::Rgb {
            r: 238,
            g: 153,
            b: 160,
        },
        Color::Rgb {
            r: 142,
            g: 213,
            b: 202,
        },
        Color::Rgb {
            r: 238,
            g: 212,
            b: 159,
        },
        Color::Rgb {
            r: 124,
            g: 196,
            b: 228,
        },
    ],
    border: Color::Rgb {
        r: 188,
        g: 192,
        b: 204,
    },
    counter: Color::Rgb {
        r: 124,
        g: 196,
        b: 228,
    },
    menu_text: Color::Rgb {
        r: 76,
        g: 79,
        b: 105,
    },
    menu_text_hovered: Color::Rgb {
        r: 255,
        g: 179,
        b: 185,
    },
    menu_title_art: Color::Rgb {
        r: 136,
        g: 173,
        b: 244,
    },
};

// Catppuccin Frappe
pub const CATPPUCCIN_FRAPPE_THEME: ColorConfig = ColorConfig {
    background: Color::Rgb {
        r: 48,
        g: 52,
        b: 70,
    },
    hidden_cell: Color::Rgb {
        r: 112,
        g: 120,
        b: 144,
    },
    flagged_cell: Color::Rgb {
        r: 242,
        g: 213,
        b: 207,
    },
    mine: Color::Rgb {
        r: 231,
        g: 130,
        b: 132,
    },
    empty_cell: Color::Rgb {
        r: 98,
        g: 104,
        b: 128,
    },
    number: [
        Color::Rgb {
            r: 141,
            g: 181,
            b: 243,
        },
        Color::Rgb {
            r: 166,
            g: 209,
            b: 137,
        },
        Color::Rgb {
            r: 231,
            g: 130,
            b: 132,
        },
        Color::Rgb {
            r: 203,
            g: 159,
            b: 242,
        },
        Color::Rgb {
            r: 239,
            g: 159,
            b: 162,
        },
        Color::Rgb {
            r: 129,
            g: 200,
            b: 190,
        },
        Color::Rgb {
            r: 229,
            g: 200,
            b: 144,
        },
        Color::Rgb {
            r: 158,
            g: 206,
            b: 226,
        },
    ],
    border: Color::Rgb {
        r: 129,
        g: 135,
        b: 158,
    },
    counter: Color::Rgb {
        r: 158,
        g: 206,
        b: 226,
    },
    menu_text: Color::Rgb {
        r: 198,
        g: 208,
        b: 245,
    },
    menu_text_hovered: Color::Rgb {
        r: 242,
        g: 213,
        b: 207,
    },
    menu_title_art: Color::Rgb {
        r: 141,
        g: 181,
        b: 243,
    },
};

// Catppuccin Macchiato
pub const CATPPUCCIN_MACCHIATO_THEME: ColorConfig = ColorConfig {
    background: Color::Rgb {
        r: 36,
        g: 39,
        b: 58,
    },
    hidden_cell: Color::Rgb {
        r: 108,
        g: 112,
        b: 134,
    },
    flagged_cell: Color::Rgb {
        r: 240,
        g: 198,
        b: 198,
    },
    mine: Color::Rgb {
        r: 237,
        g: 135,
        b: 150,
    },
    empty_cell: Color::Rgb {
        r: 88,
        g: 91,
        b: 112,
    },
    number: [
        Color::Rgb {
            r: 138,
            g: 173,
            b: 244,
        },
        Color::Rgb {
            r: 166,
            g: 218,
            b: 149,
        },
        Color::Rgb {
            r: 237,
            g: 135,
            b: 150,
        },
        Color::Rgb {
            r: 202,
            g: 158,
            b: 242,
        },
        Color::Rgb {
            r: 238,
            g: 153,
            b: 160,
        },
        Color::Rgb {
            r: 135,
            g: 205,
            b: 194,
        },
        Color::Rgb {
            r: 233,
            g: 204,
            b: 139,
        },
        Color::Rgb {
            r: 157,
            g: 204,
            b: 223,
        },
    ],
    border: Color::Rgb {
        r: 130,
        g: 133,
        b: 154,
    },
    counter: Color::Rgb {
        r: 157,
        g: 204,
        b: 223,
    },
    menu_text: Color::Rgb {
        r: 202,
        g: 211,
        b: 245,
    },
    menu_text_hovered: Color::Rgb {
        r: 240,
        g: 198,
        b: 198,
    },
    menu_title_art: Color::Rgb {
        r: 138,
        g: 173,
        b: 244,
    },
};

// Catppuccin Mocha
pub const CATPPUCCIN_MOCHA_THEME: ColorConfig = ColorConfig {
    background: Color::Rgb {
        r: 30,
        g: 30,
        b: 46,
    },
    hidden_cell: Color::Rgb {
        r: 108,
        g: 112,
        b: 134,
    },
    flagged_cell: Color::Rgb {
        r: 245,
        g: 224,
        b: 220,
    },
    mine: Color::Rgb {
        r: 243,
        g: 139,
        b: 168,
    },
    empty_cell: Color::Rgb {
        r: 88,
        g: 91,
        b: 112,
    },
    number: [
        Color::Rgb {
            r: 137,
            g: 180,
            b: 250,
        },
        Color::Rgb {
            r: 166,
            g: 227,
            b: 161,
        },
        Color::Rgb {
            r: 243,
            g: 139,
            b: 168,
        },
        Color::Rgb {
            r: 203,
            g: 166,
            b: 247,
        },
        Color::Rgb {
            r: 250,
            g: 179,
            b: 180,
        },
        Color::Rgb {
            r: 148,
            g: 226,
            b: 213,
        },
        Color::Rgb {
            r: 249,
            g: 226,
            b: 175,
        },
        Color::Rgb {
            r: 186,
            g: 194,
            b: 222,
        },
    ],
    border: Color::Rgb {
        r: 125,
        g: 128,
        b: 150,
    },
    counter: Color::Rgb {
        r: 186,
        g: 194,
        b: 222,
    },
    menu_text: Color::Rgb {
        r: 205,
        g: 214,
        b: 244,
    },
    menu_text_hovered: Color::Rgb {
        r: 245,
        g: 224,
        b: 220,
    },
    menu_title_art: Color::Rgb {
        r: 137,
        g: 180,
        b: 250,
    },
};

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
