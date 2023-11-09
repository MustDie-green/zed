// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

use crate::{
    Appearance, StatusColorsRefinement, ThemeColorsRefinement, UserTheme, UserThemeFamily,
    UserThemeStylesRefinement,
};

pub fn palenight() -> UserThemeFamily {
    UserThemeFamily {
        name: "Palenight".into(),
        author: "Olaolu Olawuyi (whizkydee)".into(),
        themes: vec![
            UserTheme {
                name: "Palenight".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x282b3bff).into()),
                        border_variant: Some(rgba(0x282b3bff).into()),
                        border_focused: Some(rgba(0x282b3bff).into()),
                        border_selected: Some(rgba(0x282b3bff).into()),
                        border_transparent: Some(rgba(0x282b3bff).into()),
                        border_disabled: Some(rgba(0x282b3bff).into()),
                        elevated_surface_background: Some(rgba(0x292c3eff).into()),
                        surface_background: Some(rgba(0x292c3eff).into()),
                        background: Some(rgba(0x292c3eff).into()),
                        element_background: Some(rgba(0x7d56c1cc).into()),
                        element_hover: Some(rgba(0x0000001a).into()),
                        element_selected: Some(rgba(0x7d56c1ff).into()),
                        drop_target_background: Some(rgba(0x2e3245ff).into()),
                        ghost_element_hover: Some(rgba(0x0000001a).into()),
                        text: Some(rgba(0xffffffff).into()),
                        tab_inactive_background: Some(rgba(0x31364aff).into()),
                        tab_active_background: Some(rgba(0x292c3eff).into()),
                        editor_background: Some(rgba(0x292c3eff).into()),
                        editor_gutter_background: Some(rgba(0x292c3eff).into()),
                        editor_line_number: Some(rgba(0x4c5374ff).into()),
                        editor_active_line_number: Some(rgba(0xbfc7d5ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x676e95ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xff5571ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0xc3e88dff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xffcb6bff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x82aaffff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0xc792eaff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x89ddffff).into()),
                        terminal_ansi_bright_white: Some(rgba(0xffffffff).into()),
                        terminal_ansi_black: Some(rgba(0x676e95ff).into()),
                        terminal_ansi_red: Some(rgba(0xff5571ff).into()),
                        terminal_ansi_green: Some(rgba(0xa9c77dff).into()),
                        terminal_ansi_yellow: Some(rgba(0xffcb6bff).into()),
                        terminal_ansi_blue: Some(rgba(0x82aaffff).into()),
                        terminal_ansi_magenta: Some(rgba(0xc792eaff).into()),
                        terminal_ansi_cyan: Some(rgba(0x89ddffff).into()),
                        terminal_ansi_white: Some(rgba(0xffffffff).into()),
                        ..Default::default()
                    },
                    status: StatusColorsRefinement {
                        deleted: Some(rgba(0xef524fff).into()),
                        error: Some(rgba(0xef524fff).into()),
                        hidden: Some(rgba(0x9199c8ff).into()),
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Palenight Operator".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x282b3bff).into()),
                        border_variant: Some(rgba(0x282b3bff).into()),
                        border_focused: Some(rgba(0x282b3bff).into()),
                        border_selected: Some(rgba(0x282b3bff).into()),
                        border_transparent: Some(rgba(0x282b3bff).into()),
                        border_disabled: Some(rgba(0x282b3bff).into()),
                        elevated_surface_background: Some(rgba(0x292c3eff).into()),
                        surface_background: Some(rgba(0x292c3eff).into()),
                        background: Some(rgba(0x292c3eff).into()),
                        element_background: Some(rgba(0x7d56c1cc).into()),
                        element_hover: Some(rgba(0x0000001a).into()),
                        element_selected: Some(rgba(0x7d56c1ff).into()),
                        drop_target_background: Some(rgba(0x2e3245ff).into()),
                        ghost_element_hover: Some(rgba(0x0000001a).into()),
                        text: Some(rgba(0xffffffff).into()),
                        tab_inactive_background: Some(rgba(0x31364aff).into()),
                        tab_active_background: Some(rgba(0x292c3eff).into()),
                        editor_background: Some(rgba(0x292c3eff).into()),
                        editor_gutter_background: Some(rgba(0x292c3eff).into()),
                        editor_line_number: Some(rgba(0x4c5374ff).into()),
                        editor_active_line_number: Some(rgba(0xbfc7d5ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x676e95ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xff5571ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0xc3e88dff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xffcb6bff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x82aaffff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0xc792eaff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x89ddffff).into()),
                        terminal_ansi_bright_white: Some(rgba(0xffffffff).into()),
                        terminal_ansi_black: Some(rgba(0x676e95ff).into()),
                        terminal_ansi_red: Some(rgba(0xff5571ff).into()),
                        terminal_ansi_green: Some(rgba(0xa9c77dff).into()),
                        terminal_ansi_yellow: Some(rgba(0xffcb6bff).into()),
                        terminal_ansi_blue: Some(rgba(0x82aaffff).into()),
                        terminal_ansi_magenta: Some(rgba(0xc792eaff).into()),
                        terminal_ansi_cyan: Some(rgba(0x89ddffff).into()),
                        terminal_ansi_white: Some(rgba(0xffffffff).into()),
                        ..Default::default()
                    },
                    status: StatusColorsRefinement {
                        deleted: Some(rgba(0xef524fff).into()),
                        error: Some(rgba(0xef524fff).into()),
                        hidden: Some(rgba(0x9199c8ff).into()),
                        ..Default::default()
                    },
                },
            },
            UserTheme {
                name: "Palenight (Mild Contrast)".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x2c2f40ff).into()),
                        border_variant: Some(rgba(0x2c2f40ff).into()),
                        border_focused: Some(rgba(0x2c2f40ff).into()),
                        border_selected: Some(rgba(0x2c2f40ff).into()),
                        border_transparent: Some(rgba(0x2c2f40ff).into()),
                        border_disabled: Some(rgba(0x2c2f40ff).into()),
                        elevated_surface_background: Some(rgba(0x25283aff).into()),
                        surface_background: Some(rgba(0x25283aff).into()),
                        background: Some(rgba(0x292c3eff).into()),
                        element_background: Some(rgba(0x7d56c1cc).into()),
                        element_hover: Some(rgba(0x0000001a).into()),
                        element_selected: Some(rgba(0x7d56c1ff).into()),
                        drop_target_background: Some(rgba(0x2e3245ff).into()),
                        ghost_element_hover: Some(rgba(0x0000001a).into()),
                        text: Some(rgba(0xffffffff).into()),
                        tab_inactive_background: Some(rgba(0x31364aff).into()),
                        tab_active_background: Some(rgba(0x25283aff).into()),
                        editor_background: Some(rgba(0x292c3eff).into()),
                        editor_gutter_background: Some(rgba(0x292c3eff).into()),
                        editor_line_number: Some(rgba(0x4c5374ff).into()),
                        editor_active_line_number: Some(rgba(0xbfc7d5ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x676e95ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xff5571ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0xc3e88dff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xffcb6bff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x82aaffff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0xc792eaff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0x89ddffff).into()),
                        terminal_ansi_bright_white: Some(rgba(0xffffffff).into()),
                        terminal_ansi_black: Some(rgba(0x676e95ff).into()),
                        terminal_ansi_red: Some(rgba(0xff5571ff).into()),
                        terminal_ansi_green: Some(rgba(0xa9c77dff).into()),
                        terminal_ansi_yellow: Some(rgba(0xffcb6bff).into()),
                        terminal_ansi_blue: Some(rgba(0x82aaffff).into()),
                        terminal_ansi_magenta: Some(rgba(0xc792eaff).into()),
                        terminal_ansi_cyan: Some(rgba(0x89ddffff).into()),
                        terminal_ansi_white: Some(rgba(0xffffffff).into()),
                        ..Default::default()
                    },
                    status: StatusColorsRefinement {
                        deleted: Some(rgba(0xef524fff).into()),
                        error: Some(rgba(0xef524fff).into()),
                        hidden: Some(rgba(0x9199c8ff).into()),
                        ..Default::default()
                    },
                },
            },
        ],
    }
}
