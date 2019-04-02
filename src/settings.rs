use crate::color::{Color, ImpactColor};
use crate::key::{Category, CategoryId, Key};
use crate::DEFAULT_CONFIG_PATH;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub enum Mode {
    Wave,
    Impact,
    Off
}

impl FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "wave" || s == "w" {
            Ok(Mode::Wave)
        } else if s == "impact" || s == "i" {
            Ok(Mode::Impact)
        } else if s == "off" || s == "o" {
            Ok(Mode::Off)
        } else {
            Err(format!("Invalid mode identifier : {}", s))
        }
    }
}

#[derive(Serialize, Deserialize)]
struct ImpactSettings {
    fixed_keys: Vec<Key>,
    fixed_categories: Vec<Category>,
    color_idx: [ImpactColor; 7],
}

impl Default for ImpactSettings {
    fn default() -> Self {
        let main: CategoryId = 1;
        let digits: CategoryId = 2;
        let functions: CategoryId = 3;
        let specials: CategoryId = 4;
        let specials2: CategoryId = 5;
        let specials3: CategoryId = 6;

        let categories = vec![
            Category::new(main, "Main color[letters]", Color::from(10, 20, 30)),
            Category::new(digits, " Digits color", Color::from(0, 100, 30)),
            Category::new(functions, "Functions color[F1, F2..]", Color::from(255, 85, 0)),
            Category::new(specials, "Specials color [CTRL, ALT, TAB, SHIFT..]", Color::from(0, 20, 20)),
            Category::new(specials2, "Specials 2 color [ENTER, GRAVE..]", Color::from(255, 0, 0)),
            Category::new(specials3, "Specials 3 color [ESC, ARROWS, META..]", Color::from(255, 255, 255)),
        ];

        let keys: Vec<Key> = vec![
            Key::new("A".to_owned(), main, None),
            Key::new("Z".to_owned(), main, None),
            Key::new("E".to_owned(), main, None),
            Key::new("R".to_owned(), main, None),
            Key::new("T".to_owned(), main, None),
            Key::new("Y".to_owned(), main, None),
            Key::new("U".to_owned(), main, None),
            Key::new("I".to_owned(), main, None),
            Key::new("O".to_owned(), main, None),
            Key::new("P".to_owned(), main, None),
            Key::new("Q".to_owned(), main, None),
            Key::new("S".to_owned(), main, None),
            Key::new("D".to_owned(), main, None),
            Key::new("F".to_owned(), main, None),
            Key::new("G".to_owned(), main, None),
            Key::new("H".to_owned(), main, None),
            Key::new("J".to_owned(), main, None),
            Key::new("K".to_owned(), main, None),
            Key::new("L".to_owned(), main, None),
            Key::new("M".to_owned(), main, None),
            Key::new("W".to_owned(), main, None),
            Key::new("X".to_owned(), main, None),
            Key::new("C".to_owned(), main, None),
            Key::new("V".to_owned(), main, None),
            Key::new("B".to_owned(), main, None),
            Key::new("N".to_owned(), main, None),
            Key::new("SEMICOLON".to_owned(), main, None),
            Key::new("COMMA".to_owned(), main, None),
            Key::new("DOT".to_owned(), main, None),
            Key::new("SLASH".to_owned(), main, None),
            Key::new("APOSTROPHE".to_owned(), main, None),
            Key::new("BACKSLASH".to_owned(), main, None),
            Key::new("LEFTBRACE".to_owned(), main, None),
            Key::new("RIGHTBRACE".to_owned(), main, None),
            Key::new("MINUS".to_owned(), main, None),
            Key::new("EQUAL".to_owned(), main, None),
            Key::new("TAB".to_owned(), specials, None),
            Key::new("CAPSLOCK".to_owned(), specials, None),
            Key::new("LEFTSHIFT".to_owned(), specials, None),
            Key::new("RIGHTSHIFT".to_owned(), specials, None),
            Key::new("LEFTCTRL".to_owned(), specials, None),
            Key::new("RIGHTCTRL".to_owned(), specials, None),
            Key::new("LEFTALT".to_owned(), specials, None),
            Key::new("RIGHTALT".to_owned(), specials, None),
            Key::new("COMPOSE".to_owned(), specials, None),
            Key::new("BACKSPACE".to_owned(), specials, None),
            Key::new("SPACE".to_owned(), specials, None),
            Key::new("KPDOT".to_owned(), specials, None),
            Key::new("KPPLUS".to_owned(), specials, None),
            Key::new("KPMINUS".to_owned(), specials, None),
            Key::new("KPASTERISK".to_owned(), specials, None),
            Key::new("KPSLASH".to_owned(), specials, None),
            Key::new("NUMLOCK".to_owned(), specials, None),
            Key::new("SYSRQ".to_owned(), specials, None),
            Key::new("SCROLLLOCK".to_owned(), specials, None),
            Key::new("PAUSE".to_owned(), specials, None),
            Key::new("INSERT".to_owned(), specials, None),
            Key::new("HOME".to_owned(), specials, None),
            Key::new("PAGEUP".to_owned(), specials, None),
            Key::new("PAGEDOWN".to_owned(), specials, None),
            Key::new("END".to_owned(), specials, None),
            Key::new("DELETE".to_owned(), specials, None),
            Key::new("0".to_owned(), digits, None),
            Key::new("1".to_owned(), digits, None),
            Key::new("2".to_owned(), digits, None),
            Key::new("3".to_owned(), digits, None),
            Key::new("4".to_owned(), digits, None),
            Key::new("5".to_owned(), digits, None),
            Key::new("6".to_owned(), digits, None),
            Key::new("7".to_owned(), digits, None),
            Key::new("8".to_owned(), digits, None),
            Key::new("9".to_owned(), digits, None),
            Key::new("KP0".to_owned(), digits, None),
            Key::new("KP1".to_owned(), digits, None),
            Key::new("KP2".to_owned(), digits, None),
            Key::new("KP3".to_owned(), digits, None),
            Key::new("KP4".to_owned(), digits, None),
            Key::new("KP5".to_owned(), digits, None),
            Key::new("KP6".to_owned(), digits, None),
            Key::new("KP7".to_owned(), digits, None),
            Key::new("KP8".to_owned(), digits, None),
            Key::new("KP9".to_owned(), digits, None),
            Key::new("F1".to_owned(), functions, None),
            Key::new("F2".to_owned(), functions, None),
            Key::new("F3".to_owned(), functions, None),
            Key::new("F4".to_owned(), functions, None),
            Key::new("F5".to_owned(), functions, None),
            Key::new("F6".to_owned(), functions, None),
            Key::new("F7".to_owned(), functions, None),
            Key::new("F8".to_owned(), functions, None),
            Key::new("F9".to_owned(), functions, None),
            Key::new("F10".to_owned(), functions, None),
            Key::new("F11".to_owned(), functions, None),
            Key::new("F12".to_owned(), functions, None),
            Key::new("ENTER".to_owned(), specials2, None),
            Key::new("KPENTER".to_owned(), specials2, None),
            Key::new("102ND".to_owned(), specials2, None),
            Key::new("GRAVE".to_owned(), specials2, None),
            Key::new("ESC".to_owned(), specials3, None),
            Key::new("UP".to_owned(), specials3, None),
            Key::new("DOWN".to_owned(), specials3, None),
            Key::new("RIGHT".to_owned(), specials3, None),
            Key::new("LEFT".to_owned(), specials3, None),
            Key::new("LEFTMETA".to_owned(), specials3, None),
        ];
        /*

        colorIdx    R      G      B  Desc
        ------------------------------------------------
        0           0      0    119  Base keyboard color (dark blue)
        1        2303      0   -255  Typing color, initial key (over-red, under-blue)
        2        2303      0   -143  Typing color, first neighbor key
        3        2303      0      0  Typing color, second neighbor key
        4         187      0    204  Ghost typing color, initial key
        5         153      0    187  Ghost typing color, first neighbor key
        6          85      0    170  Ghost typing color, second neighbor key
        7           0      0      0  (null)
        8           0      0      0  (null)
        9           0      0      0  (null)

        */
        ImpactSettings {
            fixed_keys: keys,
            fixed_categories: categories,
            color_idx: [
                ImpactColor::from(0, 0, 119),
                ImpactColor::from(2303, 0, -255),
                ImpactColor::from(2303, 0, -143),
                ImpactColor::from(2303, 0, 0),
                ImpactColor::from(187, 0, 187),
                ImpactColor::from(153, 0, 187),
                ImpactColor::from(85, 0, 170),
            ],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Settings {
    impact_settings: ImpactSettings,
    wave_speed: u8,
    mode: Mode,
    config_path: PathBuf,
}

impl Settings {
    pub fn from_ron(path: &Path) -> Result<Self, String> {
        // TODO Better errors handling
        let file = File::open(path).map_err(|err| err.to_string())?;
        ron::de::from_reader(file).map_err(|err| err.to_string())
    }

    pub fn save(&self) -> Result<(), String> {
        // TODO Better errors handling
        let mut file = File::create(&self.config_path).map_err(|err| err.to_string())?;
        file.write_all(
            ron::ser::to_string_pretty(self, PrettyConfig::default())
                .map_err(|err| err.to_string())?
                .as_bytes(),
        )
        .map_err(|err| err.to_string())
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn launch(&self, verbose: bool) {
        let mut cmd = Command::new("roccat-vulcan");

        match self.mode {
            Mode::Wave => {
                cmd.args(&["-w", &format!("{}", self.wave_speed)]);
            }
            Mode::Impact => {
                for key in self.impact_settings.fixed_keys.iter() {
                    let cat = self
                        .impact_settings
                        .fixed_categories
                        .iter()
                        .find(|c| c.id() == key.category());
                    let color = match key.custom_color() {
                        None => match cat {
                            None => {
                                eprintln!("Key color not defined for key {}", key.id());
                                continue;
                            }
                            Some(cat) => cat.color(),
                        },
                        Some(custom_color) => custom_color,
                    };

                    cmd.args(&["-k", &format!("KEY_{}:{}", key.id(), color)]);
                }

                for (i, color) in self.impact_settings.color_idx.iter().enumerate() {
                    cmd.args(&["-c", &format!("{}:{}", i, color)]);
                }
            }
            Mode::Off => {
                for key in self.impact_settings.fixed_keys.iter() {
                    let cat = self
                        .impact_settings
                        .fixed_categories
                        .iter()
                        .find(|c| c.id() == key.category());
                    let color = Color::default();

                    cmd.args(&["-k", &format!("KEY_{}:{}", key.id(), color)]);
                }
            }
        }

        if verbose {
            cmd.arg("-v");
        }

        println!("{}", cmd.status().expect("Fail"));
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            impact_settings: ImpactSettings::default(),
            wave_speed: 6,
            mode: Mode::Impact,
            config_path: DEFAULT_CONFIG_PATH.into(),
        }
    }
}
