#![allow(dead_code)]

use ratatui::style::{Color, Modifier, Style};

// ── Theme variant selector ─────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TemaVariant {
    Oc2Dark,
    Oc2Light,
    Catppuccin,
    Nord,
    Dracula,
}

impl TemaVariant {
    pub fn todos() -> &'static [Self] {
        &[
            Self::Oc2Dark,
            Self::Oc2Light,
            Self::Catppuccin,
            Self::Nord,
            Self::Dracula,
        ]
    }

    pub fn nombre(self) -> &'static str {
        match self {
            Self::Oc2Dark => "OC-2 Dark",
            Self::Oc2Light => "OC-2 Light",
            Self::Catppuccin => "Catppuccin",
            Self::Nord => "Nord",
            Self::Dracula => "Dracula",
        }
    }
}

impl Default for TemaVariant {
    fn default() -> Self {
        Self::Oc2Dark
    }
}

// ── Theme colors struct ────────────────────────────────────────────────

pub struct ThemeColors {
    pub bg_base: Color,
    pub bg_weak: Color,
    pub surface: Color,
    pub border_weak: Color,
    pub text_strong: Color,
    pub text_base: Color,
    pub text_weak: Color,
    pub text_weaker: Color,
    pub primary: Color,
    pub primary_dim: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub teal: Color,
    pub method_get: Color,
    pub method_post: Color,
    pub method_put: Color,
    pub method_patch: Color,
    pub method_delete: Color,
    pub method_head: Color,
    pub method_options: Color,
}

pub fn tema_actual(variant: TemaVariant) -> ThemeColors {
    match variant {
        TemaVariant::Oc2Dark => ThemeColors {
            bg_base: Color::Rgb(16, 16, 16),
            bg_weak: Color::Rgb(30, 30, 30),
            surface: Color::Rgb(35, 35, 35),
            border_weak: Color::Rgb(40, 40, 40),
            text_strong: Color::Rgb(237, 237, 237),
            text_base: Color::Rgb(160, 160, 160),
            text_weak: Color::Rgb(112, 112, 112),
            text_weaker: Color::Rgb(80, 80, 80),
            primary: Color::Rgb(250, 178, 131),
            primary_dim: Color::Rgb(180, 130, 95),
            success: Color::Rgb(18, 201, 5),
            warning: Color::Rgb(252, 213, 58),
            error: Color::Rgb(252, 83, 58),
            teal: Color::Rgb(0, 206, 185),
            method_get: Color::Rgb(18, 201, 5),
            method_post: Color::Rgb(250, 178, 131),
            method_put: Color::Rgb(252, 213, 58),
            method_patch: Color::Rgb(237, 178, 241),
            method_delete: Color::Rgb(252, 83, 58),
            method_head: Color::Rgb(147, 233, 246),
            method_options: Color::Rgb(0, 206, 185),
        },
        TemaVariant::Oc2Light => ThemeColors {
            bg_base: Color::Rgb(250, 250, 250),
            bg_weak: Color::Rgb(240, 240, 240),
            surface: Color::Rgb(235, 235, 235),
            border_weak: Color::Rgb(210, 210, 210),
            text_strong: Color::Rgb(20, 20, 20),
            text_base: Color::Rgb(80, 80, 80),
            text_weak: Color::Rgb(140, 140, 140),
            text_weaker: Color::Rgb(180, 180, 180),
            primary: Color::Rgb(180, 100, 40),
            primary_dim: Color::Rgb(200, 140, 90),
            success: Color::Rgb(10, 150, 20),
            warning: Color::Rgb(180, 140, 10),
            error: Color::Rgb(200, 50, 30),
            teal: Color::Rgb(0, 140, 130),
            method_get: Color::Rgb(10, 150, 20),
            method_post: Color::Rgb(180, 100, 40),
            method_put: Color::Rgb(180, 140, 10),
            method_patch: Color::Rgb(140, 80, 160),
            method_delete: Color::Rgb(200, 50, 30),
            method_head: Color::Rgb(60, 140, 180),
            method_options: Color::Rgb(0, 140, 130),
        },
        TemaVariant::Catppuccin => ThemeColors {
            bg_base: Color::Rgb(30, 30, 46),
            bg_weak: Color::Rgb(49, 50, 68),
            surface: Color::Rgb(69, 71, 90),
            border_weak: Color::Rgb(88, 91, 112),
            text_strong: Color::Rgb(205, 214, 244),
            text_base: Color::Rgb(166, 173, 200),
            text_weak: Color::Rgb(127, 132, 156),
            text_weaker: Color::Rgb(88, 91, 112),
            primary: Color::Rgb(245, 194, 231),
            primary_dim: Color::Rgb(180, 140, 170),
            success: Color::Rgb(166, 227, 161),
            warning: Color::Rgb(249, 226, 175),
            error: Color::Rgb(243, 139, 168),
            teal: Color::Rgb(148, 226, 213),
            method_get: Color::Rgb(166, 227, 161),
            method_post: Color::Rgb(245, 194, 231),
            method_put: Color::Rgb(249, 226, 175),
            method_patch: Color::Rgb(203, 166, 247),
            method_delete: Color::Rgb(243, 139, 168),
            method_head: Color::Rgb(137, 180, 250),
            method_options: Color::Rgb(148, 226, 213),
        },
        TemaVariant::Nord => ThemeColors {
            bg_base: Color::Rgb(46, 52, 64),
            bg_weak: Color::Rgb(59, 66, 82),
            surface: Color::Rgb(67, 76, 94),
            border_weak: Color::Rgb(76, 86, 106),
            text_strong: Color::Rgb(236, 239, 244),
            text_base: Color::Rgb(216, 222, 233),
            text_weak: Color::Rgb(180, 190, 205),
            text_weaker: Color::Rgb(136, 150, 170),
            primary: Color::Rgb(136, 192, 208),
            primary_dim: Color::Rgb(100, 150, 170),
            success: Color::Rgb(163, 190, 140),
            warning: Color::Rgb(235, 203, 139),
            error: Color::Rgb(191, 97, 106),
            teal: Color::Rgb(143, 188, 187),
            method_get: Color::Rgb(163, 190, 140),
            method_post: Color::Rgb(136, 192, 208),
            method_put: Color::Rgb(235, 203, 139),
            method_patch: Color::Rgb(180, 142, 173),
            method_delete: Color::Rgb(191, 97, 106),
            method_head: Color::Rgb(129, 161, 193),
            method_options: Color::Rgb(143, 188, 187),
        },
        TemaVariant::Dracula => ThemeColors {
            bg_base: Color::Rgb(40, 42, 54),
            bg_weak: Color::Rgb(55, 58, 73),
            surface: Color::Rgb(68, 71, 90),
            border_weak: Color::Rgb(80, 83, 100),
            text_strong: Color::Rgb(248, 248, 242),
            text_base: Color::Rgb(189, 191, 183),
            text_weak: Color::Rgb(130, 133, 125),
            text_weaker: Color::Rgb(98, 100, 95),
            primary: Color::Rgb(255, 121, 198),
            primary_dim: Color::Rgb(180, 90, 140),
            success: Color::Rgb(80, 250, 123),
            warning: Color::Rgb(241, 250, 140),
            error: Color::Rgb(255, 85, 85),
            teal: Color::Rgb(139, 233, 253),
            method_get: Color::Rgb(80, 250, 123),
            method_post: Color::Rgb(255, 121, 198),
            method_put: Color::Rgb(241, 250, 140),
            method_patch: Color::Rgb(189, 147, 249),
            method_delete: Color::Rgb(255, 85, 85),
            method_head: Color::Rgb(139, 233, 253),
            method_options: Color::Rgb(80, 250, 123),
        },
    }
}

// ── Active theme (thread-local for simplicity) ─────────────────────────

use std::cell::RefCell;

thread_local! {
    static ACTIVE_THEME: RefCell<TemaVariant> = RefCell::new(TemaVariant::Oc2Dark);
}

pub fn set_active_theme(variant: TemaVariant) {
    ACTIVE_THEME.with(|t| *t.borrow_mut() = variant);
}

pub fn get_active_theme() -> TemaVariant {
    ACTIVE_THEME.with(|t| *t.borrow())
}

fn with_colors<F, R>(f: F) -> R
where
    F: FnOnce(&ThemeColors) -> R,
{
    let variant = get_active_theme();
    let colors = tema_actual(variant);
    f(&colors)
}

// ── Public style API ───────────────────────────────────────────────────

pub fn method_color(method: &str) -> Color {
    with_colors(|c| match method {
        "GET" => c.method_get,
        "POST" => c.method_post,
        "PUT" => c.method_put,
        "PATCH" => c.method_patch,
        "DELETE" => c.method_delete,
        "HEAD" => c.method_head,
        "OPTIONS" => c.method_options,
        _ => c.text_base,
    })
}

pub fn title() -> Style {
    with_colors(|c| Style::default().fg(c.primary).add_modifier(Modifier::BOLD))
}

pub fn focused() -> Style {
    with_colors(|c| Style::default().fg(c.primary))
}

pub fn normal() -> Style {
    with_colors(|c| Style::default().fg(c.text_strong))
}

pub fn base() -> Style {
    with_colors(|c| Style::default().fg(c.text_base))
}

pub fn muted() -> Style {
    with_colors(|c| Style::default().fg(c.text_weak))
}

pub fn sidebar_selected() -> Style {
    with_colors(|c| {
        Style::default()
            .fg(c.text_strong)
            .bg(c.surface)
            .add_modifier(Modifier::BOLD)
    })
}

pub fn border(active: bool) -> Style {
    with_colors(|c| {
        if active {
            Style::default().fg(c.primary)
        } else {
            Style::default().fg(c.border_weak)
        }
    })
}

pub fn status_style() -> Style {
    with_colors(|c| Style::default().fg(c.text_weak).bg(c.bg_weak))
}

// ── Direct color access for renderer ───────────────────────────────────

pub fn bg_base() -> Color {
    with_colors(|c| c.bg_base)
}

pub fn bg_weak() -> Color {
    with_colors(|c| c.bg_weak)
}

pub fn surface_color() -> Color {
    with_colors(|c| c.surface)
}

pub fn border_weak() -> Color {
    with_colors(|c| c.border_weak)
}

pub fn text_strong() -> Color {
    with_colors(|c| c.text_strong)
}

pub fn text_base_color() -> Color {
    with_colors(|c| c.text_base)
}

pub fn text_weak_color() -> Color {
    with_colors(|c| c.text_weak)
}

pub fn text_weaker_color() -> Color {
    with_colors(|c| c.text_weaker)
}

pub fn primary_color() -> Color {
    with_colors(|c| c.primary)
}

pub fn primary_dim_color() -> Color {
    with_colors(|c| c.primary_dim)
}

pub fn success_color() -> Color {
    with_colors(|c| c.success)
}

pub fn warning_color() -> Color {
    with_colors(|c| c.warning)
}

pub fn error_color() -> Color {
    with_colors(|c| c.error)
}

pub fn teal_color() -> Color {
    with_colors(|c| c.teal)
}
