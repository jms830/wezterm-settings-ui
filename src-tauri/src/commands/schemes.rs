// Color scheme commands - list available WezTerm color schemes

use serde::{Deserialize, Serialize};

/// Information about a color scheme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorSchemeInfo {
    pub name: String,
    pub category: String,
}

/// Get a list of popular built-in WezTerm color schemes
/// WezTerm includes 700+ schemes from iTerm2-Color-Schemes, base16, Gogh, etc.
/// This returns a curated list of popular ones, organized by category.
#[tauri::command]
pub fn get_builtin_color_schemes() -> Vec<ColorSchemeInfo> {
    let mut schemes = Vec::new();

    // Catppuccin variants (very popular)
    for variant in ["Catppuccin Latte", "Catppuccin Frappe", "Catppuccin Macchiato", "Catppuccin Mocha"] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "Catppuccin".to_string(),
        });
    }

    // Dracula variants
    for variant in ["Dracula", "Dracula+", "Dracula (Official)"] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "Dracula".to_string(),
        });
    }

    // Nord variants
    for variant in ["Nord", "Nord (Gogh)", "Nord Light"] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "Nord".to_string(),
        });
    }

    // Gruvbox variants
    for variant in [
        "Gruvbox Dark",
        "Gruvbox dark, hard (base16)",
        "Gruvbox dark, medium (base16)",
        "Gruvbox dark, pale (base16)",
        "Gruvbox dark, soft (base16)",
        "Gruvbox Light",
        "Gruvbox light, hard (base16)",
        "Gruvbox light, medium (base16)",
        "Gruvbox light, soft (base16)",
        "Gruvbox Material (Gogh)",
    ] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "Gruvbox".to_string(),
        });
    }

    // Solarized variants
    for variant in [
        "Solarized (dark) (terminal.sexy)",
        "Solarized (light) (terminal.sexy)",
        "Solarized Dark - Patched",
        "Solarized Dark (Gogh)",
        "Solarized Dark Higher Contrast",
        "Solarized Darcula",
        "Solarized Light (Gogh)",
    ] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "Solarized".to_string(),
        });
    }

    // Tokyo Night variants
    for variant in [
        "Tokyo Night",
        "Tokyo Night (Gogh)",
        "Tokyo Night Day",
        "Tokyo Night Light",
        "Tokyo Night Moon",
        "Tokyo Night Storm",
        "Tokyo Night Storm (Gogh)",
        "tokyonight",
        "tokyonight_day",
        "tokyonight_moon",
        "tokyonight_night",
        "tokyonight_storm",
    ] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "Tokyo Night".to_string(),
        });
    }

    // One Dark / One Half variants
    for variant in [
        "One Dark (Gogh)",
        "OneDark",
        "OneHalfDark",
        "OneHalfLight",
        "One Half Black",
        "One Half Light",
    ] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "One".to_string(),
        });
    }

    // Monokai variants
    for variant in [
        "Monokai (base16)",
        "Monokai Dark (Gogh)",
        "Monokai Pro (Gogh)",
        "Monokai Remastered",
        "Monokai Soda",
        "Monokai Vivid",
    ] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "Monokai".to_string(),
        });
    }

    // GitHub variants
    for variant in [
        "GitHub Dark",
        "GitHub Dark Colorblind (Gogh)",
        "GitHub Dark Default (Gogh)",
        "GitHub Dark Dimmed (Gogh)",
        "GitHub Dark High Contrast (Gogh)",
        "GitHub Dark Tritanopia (Gogh)",
        "GitHub Light",
        "GitHub Light Colorblind (Gogh)",
        "GitHub Light Default (Gogh)",
        "GitHub Light High Contrast (Gogh)",
        "GitHub Light Tritanopia (Gogh)",
    ] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "GitHub".to_string(),
        });
    }

    // Kanagawa variants
    for variant in [
        "Kanagawa (Gogh)",
        "Kanagawa Dragon (Gogh)",
        "Kanagawa Wave (Gogh)",
    ] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "Kanagawa".to_string(),
        });
    }

    // Rose Pine variants
    for variant in [
        "rose-pine",
        "rose-pine-dawn",
        "rose-pine-moon",
        "Rosé Pine (Gogh)",
        "Rosé Pine Dawn (Gogh)",
        "Rosé Pine Moon (Gogh)",
    ] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "Rose Pine".to_string(),
        });
    }

    // Everforest variants
    for variant in [
        "Everforest Dark (Gogh)",
        "Everforest Dark Hard (Gogh)",
        "Everforest Dark Medium (Gogh)",
        "Everforest Dark Soft (Gogh)",
        "Everforest Light (Gogh)",
        "Everforest Light Hard (Gogh)",
        "Everforest Light Medium (Gogh)",
        "Everforest Light Soft (Gogh)",
    ] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "Everforest".to_string(),
        });
    }

    // Material variants
    for variant in [
        "Material",
        "Material (base16)",
        "Material (Gogh)",
        "Material Darker (base16)",
        "Material Dark (Gogh)",
        "MaterialDark",
        "MaterialDesignColors",
        "MaterialOcean",
        "Palenight (Gogh)",
    ] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "Material".to_string(),
        });
    }

    // Ayu variants
    for variant in [
        "Ayu Dark (Gogh)",
        "Ayu Light (Gogh)",
        "Ayu Mirage",
        "Ayu Mirage (Gogh)",
    ] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "Ayu".to_string(),
        });
    }

    // Nightfox variants
    for variant in [
        "carbonfox",
        "dawnfox",
        "dayfox",
        "duskfox",
        "nightfox",
        "nordfox",
        "terafox",
    ] {
        schemes.push(ColorSchemeInfo {
            name: variant.to_string(),
            category: "Nightfox".to_string(),
        });
    }

    // Classic/Popular standalone themes
    for (name, category) in [
        ("3024 Day", "Classic"),
        ("3024 Night", "Classic"),
        ("Afterglow", "Classic"),
        ("Alabaster", "Classic"),
        ("Andromeda", "Classic"),
        ("Arthur", "Classic"),
        ("AtelierSulphurpool", "Classic"),
        ("Atom", "Classic"),
        ("Atom One Light", "Classic"),
        ("Batman", "Fun"),
        ("Blazer", "Classic"),
        ("BlueBerryPie", "Fun"),
        ("BlueDolphin", "Fun"),
        ("Borland", "Retro"),
        ("Breeze", "Classic"),
        ("Campbell", "Windows"),
        ("Campbell Powershell", "Windows"),
        ("Chalk", "Classic"),
        ("Chester", "Classic"),
        ("Cobalt Neon", "Classic"),
        ("Cobalt2", "Classic"),
        ("CrayonPonyFish", "Fun"),
        ("cyberpunk", "Cyberpunk"),
        ("Cyberpunk (Gogh)", "Cyberpunk"),
        ("Dark Pastel", "Dark"),
        ("Dark+", "VSCode"),
        ("Darkside", "Dark"),
        ("Default Dark (base16)", "Default"),
        ("Default Light (base16)", "Default"),
        ("Desert", "Classic"),
        ("Doom One", "Doom"),
        ("Doom Peacock", "Doom"),
        ("DotGov", "Classic"),
        ("Duotone Dark", "Duotone"),
        ("Espresso", "Classic"),
        ("Fairyfloss", "Fun"),
        ("Firewatch", "Artistic"),
        ("Flat", "Flat"),
        ("FlatLand", "Flat"),
        ("Floraverse", "Artistic"),
        ("Forest Blue", "Nature"),
        ("Foxnightly", "Firefox"),
        ("FrontEndDelight", "Classic"),
        ("Galaxy", "Space"),
        ("Glacier", "Nature"),
        ("Grape", "Purple"),
        ("Grass", "Nature"),
        ("Hardcore", "Dark"),
        ("Harper", "Classic"),
        ("Hemisu Dark", "Classic"),
        ("Hemisu Light", "Classic"),
        ("Highway", "Classic"),
        ("Hipster Green", "Fun"),
        ("Homebrew", "Retro"),
        ("Horizon Dark", "Classic"),
        ("Horizon Light", "Classic"),
        ("Hopscotch", "Fun"),
        ("HotKey", "Classic"),
        ("Hybrid", "Classic"),
        ("IC Green PPL", "Retro"),
        ("IC Orange PPL", "Retro"),
        ("iceberg-dark", "Classic"),
        ("iceberg-light", "Classic"),
        ("IR Black", "Dark"),
        ("Japanesque", "Artistic"),
        ("Jellybeans", "Classic"),
        ("JetBrains Darcula", "JetBrains"),
        ("Kibble", "Classic"),
        ("Kolorit", "Classic"),
        ("Konsolas", "Classic"),
        ("Lab Fox", "Firefox"),
        ("Laser", "Cyberpunk"),
        ("Later This Evening", "Classic"),
        ("Lavandula", "Purple"),
        ("LiquidCarbon", "Dark"),
        ("LiquidCarbonTransparent", "Dark"),
        ("LiquidCarbonTransparentInverse", "Light"),
        ("lovelace", "Classic"),
        ("Man Page", "Classic"),
        ("Marrakesh", "Artistic"),
        ("Mathias", "Classic"),
        ("Matrix", "Retro"),
        ("Medallion", "Classic"),
        ("Melange", "Classic"),
        ("Mellow Purple", "Purple"),
        ("Midnight In Mojave", "macOS"),
        ("Mirage", "Classic"),
        ("Miu", "Classic"),
        ("Molokai", "Classic"),
        ("MonaLisa", "Artistic"),
        ("Moonfly", "Dark"),
        ("Moonlight II", "Classic"),
        ("N0tch2k", "Classic"),
        ("Neon Night (Gogh)", "Neon"),
        ("Neopolitan", "Classic"),
        ("Neon", "Neon"),
        ("NightLion v1", "Dark"),
        ("NightLion v2", "Dark"),
        ("Night Owl", "Classic"),
        ("Night Owlish Light", "Light"),
        ("Novel", "Classic"),
        ("Obsidian", "Dark"),
        ("Ocean", "Nature"),
        ("OceanicMaterial", "Material"),
        ("Ollie", "Classic"),
        ("Operator Mono Dark", "Classic"),
        ("Overnight Slumber", "Dark"),
        ("Panda", "Classic"),
        ("Pandora", "Classic"),
        ("Paper (Gogh)", "Light"),
        ("Papercolor Dark (Gogh)", "Light"),
        ("Papercolor Light (Gogh)", "Light"),
        ("Paraiso Dark", "Classic"),
        ("PaperColor Dark (base16)", "Light"),
        ("PaperColor Light (base16)", "Light"),
        ("Pencil Dark", "Classic"),
        ("Pencil Light", "Light"),
        ("Peppermint", "Fun"),
        ("Piatto Light", "Light"),
        ("Poimandres", "Classic"),
        ("Poimandres Storm", "Classic"),
        ("PowerShell", "Windows"),
        ("Pro", "Classic"),
        ("Pro Light", "Light"),
        ("Purple Rain", "Purple"),
        ("Rapture", "Classic"),
        ("Raycast Dark", "Classic"),
        ("Raycast Light", "Light"),
        ("Rebecca", "Purple"),
        ("Red Sands", "Classic"),
        ("Relaxed", "Classic"),
        ("Rippedcasts", "Classic"),
        ("Rouge 2", "Classic"),
        ("Royal", "Classic"),
        ("Ryuuko", "Classic"),
        ("Sakura", "Artistic"),
        ("Sandcastle", "Classic"),
        ("Seafoam Pastel", "Pastel"),
        ("SeaShells", "Fun"),
        ("Seti", "Classic"),
        ("Shaman", "Classic"),
        ("Slate", "Classic"),
        ("Smyck", "Classic"),
        ("Snazzy", "Classic"),
        ("Soft Server", "Classic"),
        ("SoftServer", "Classic"),
        ("Sonokai (Gogh)", "Classic"),
        ("Sourcerer", "Classic"),
        ("Spacegray", "Classic"),
        ("Spacegray Eighties", "Classic"),
        ("SpaceGray Eighties Dull", "Classic"),
        ("Spiderman", "Fun"),
        ("Spring", "Nature"),
        ("Square", "Classic"),
        ("Sublime Snazzy", "Classic"),
        ("Sundried", "Classic"),
        ("Symfonic", "Classic"),
        ("Tango", "GNOME"),
        ("Tango (terminal.sexy)", "GNOME"),
        ("Tango Dark", "GNOME"),
        ("Tango Light", "GNOME"),
        ("Teerb", "Classic"),
        ("Tempus Winter", "Classic"),
        ("Terminal Basic", "Default"),
        ("Terminix Dark", "Classic"),
        ("TheHulk", "Fun"),
        ("Thayer Bright", "Classic"),
        ("Tomorrow", "Tomorrow"),
        ("Tomorrow (dark) (terminal.sexy)", "Tomorrow"),
        ("Tomorrow (light) (terminal.sexy)", "Tomorrow"),
        ("Tomorrow Night", "Tomorrow"),
        ("Tomorrow Night Blue", "Tomorrow"),
        ("Tomorrow Night Bright", "Tomorrow"),
        ("Tomorrow Night Burns", "Tomorrow"),
        ("Tomorrow Night Eighties", "Tomorrow"),
        ("Toy Chest", "Fun"),
        ("Treehouse", "Nature"),
        ("Twilight", "Classic"),
        ("Ubuntu", "Ubuntu"),
        ("Ultima Thule", "Classic"),
        ("UnderTheSea", "Fun"),
        ("Unikitty", "Fun"),
        ("Urple", "Purple"),
        ("Vaughn", "Classic"),
        ("VibrantInk", "Classic"),
        ("Violet Dark", "Purple"),
        ("Violet Light", "Purple"),
        ("Warp", "Classic"),
        ("WarmNeon", "Neon"),
        ("Wez", "WezTerm"),
        ("Whimsy", "Fun"),
        ("Wildcherry", "Fun"),
        ("Wombat", "Classic"),
        ("Wryan", "Classic"),
        ("Zenburn", "Classic"),
        ("ZenWritten", "Classic"),
        ("zenwritten_dark", "Classic"),
        ("zenwritten_light", "Light"),
    ] {
        schemes.push(ColorSchemeInfo {
            name: name.to_string(),
            category: category.to_string(),
        });
    }

    schemes
}
