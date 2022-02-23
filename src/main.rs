use core::fmt::Display;
use std::fmt::{self, Formatter};
use std::io::stdout;
use std::io::Write;
use std::process;

use getch::Getch;
use rand::seq::SliceRandom;

use crate::parsing::load_into_vec;
mod parsing;

// fancy ANSI formatting
enum ANSI {
    Bold,
    Fore256(u8),
    Back256(u8),
    MoveCursor(i32, i32),
    Back,
    Clear,
    ClearScreen,
    Move(Loc),
    Preset(Preset),
}
enum Loc {
    Home,
    Start,
    Word(i32),
    Bottom,
}

enum Preset {
    Green,
    Yellow,
}

impl Display for ANSI {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ANSI::Bold => f.write_str("\x1b[1m"),
            ANSI::Fore256(id) => write!(f, "\x1b[38;5;{}m", id),
            ANSI::Back256(id) => write!(f, "\x1b[48;5;{}m", id),
            ANSI::MoveCursor(y, x) => write!(f, "\x1b[{};{}H", y, x),
            ANSI::Back => write!(f, "\x1b[1D"),
            ANSI::Clear => write!(f, "\x1b[0m"),
            ANSI::ClearScreen => write!(f, "\x1b[2J"),

            ANSI::Move(location) => {
                // hardcoded :lol:
                let pos: (i32, i32) = match location {
                    Loc::Home => (0, 0),         // (0,0), the home position
                    Loc::Start => (10, 0),       // after the title
                    Loc::Word(n) => (11 + n, 5), // go to the start of a specific word
                    Loc::Bottom => (17, 2),
                };
                let (y, x) = pos;
                write!(f, "\x1b[{};{}H", y, x)
            }
            ANSI::Preset(preset) => match preset {
                Preset::Green => write!(f, "{}{}", ANSI::Back256(2), ANSI::Fore256(0)),
                Preset::Yellow => write!(f, "{}{}", ANSI::Back256(3), ANSI::Fore256(0)),
            },
        }
    }
}

fn get_word() -> String {
    let ch: Getch = Getch::new();
    let mut out: [char; 11] = Default::default();
    let mut len = 0;
    while len < 11 {
        let key = ch.getch().unwrap();
        match key {
            27 => {
                println!("{}{}Pressed ESC", ANSI::ClearScreen, ANSI::Move(Loc::Home));
                process::exit(0);
            }
            97..=122 => {
                print!("{}", (key - 32) as char);
                stdout().flush().unwrap();
                out[len] = key as char;
                len += 1;
            }
            8 | 33 => {
                if len > 0 {
                    len -= 1;
                    print!("{}*{}", ANSI::Back, ANSI::Back);
                    stdout().flush().unwrap();
                }
            }
            10 | 13 => return "jank workaround lol dont mind me".to_owned(),
            _ => continue,
        }
    }
    let mut ret: String = String::new();
    for item in out {
        ret.push(item);
    }
    ret
}

fn main() {
    enable_ansi_support::enable_ansi_support().unwrap();

    print!(
        "{}{}{}
{}      :::       :::  ::::::::::  :::::::::   :::::::::   :::         ::::::::::{} (c)(r)(tm) {}
{}     :+:       :+:  :+:         :+:    :+:  :+:    :+:  :+:         :+:
{}    +:+       +:+  +:+         +:+    +:+  +:+    +:+  +:+         +:+
{}   +#+  +:+  +#+  +#++:++#    +#++:++#:   +#+    +:+  +#+         +#++:++#
{}  +#+ +#+#+ +#+  +#+         +#+    +#+  +#+    +#+  +#+         +#+
{} #+#+# #+#+#    #+#         #+#    #+#  #+#    #+#  #+#         #+#
{}###   ###      ##########  ###    ###  #########   ##########  ##########{}
──────────────────────────────────────────────────────────────────────────────────────────────
",
        ANSI::Move(Loc::Home),
        ANSI::ClearScreen,
        ANSI::Bold,
        ANSI::Fore256(196),
        ANSI::Clear,
        ANSI::Bold,
        ANSI::Fore256(202),
        ANSI::Fore256(208),
        ANSI::Fore256(208),
        ANSI::Fore256(214),
        ANSI::Fore256(214),
        ANSI::Fore256(220),
        ANSI::Clear
    );

    let p_accepted_words = include_str!("../data/alphabetized/all/11.txt");
    let p_possible_ans = include_str!("../data/alphabetized/filtered/11.txt");

    let accepted_words: Vec<String> = load_into_vec(p_accepted_words);
    let possible_ans: Vec<String> = load_into_vec(p_possible_ans);

    loop {
        let answer: String = possible_ans
            .choose(&mut rand::thread_rng())
            .unwrap()
            .to_owned();
        let c_freq: [u8; 27] = {
            let mut out: [u8; 27] = [0; 27];
            for item in answer.chars() {
                out[item as usize - 'a' as usize] += 1;
            }
            out
        };

        print!(
            "{}\n    \
    ***********       Press Enter to give up
    ***********
    ***********        Q W E R T Y U I O P
    ***********         A S D F G H J K L
    ***********          Z X C V B N M <- ignore this for now thanks
    ***********                            (ill add it in later)
",
            ANSI::Move(Loc::Start)
        );
        let mut itr = 0;
        while itr < 6 {
            print!("{}", ANSI::Move(Loc::Word(itr)));
            stdout().flush().unwrap();
            let attempt: String = get_word();
            // oh yay we found it

            if attempt == *"jank workaround lol dont mind me" {
                break;
            }

            if accepted_words.binary_search(&attempt).is_err() {
                print!("{}***********", ANSI::Move(Loc::Word(itr)));
                continue;
            }

            print!("{}", ANSI::Move(Loc::Word(itr)));
            // mark the letters!
            let mut freq: [u8; 27] = c_freq;
            for (i, letter) in attempt.chars().enumerate() {
                if letter as u8 == answer.as_bytes()[i] {
                    print!(
                        "{}{}",
                        ANSI::Preset(Preset::Green),
                        (letter as u8 - 32) as char
                    );
                    freq[letter as usize - 'a' as usize] -= 1;
                    continue;
                }
                print!("{}{}", ANSI::Clear, (letter as u8 - 32) as char);
            }

            print!("{}", ANSI::Move(Loc::Word(itr)));
            for (i, letter) in attempt.chars().enumerate() {
                if freq[letter as usize - 'a' as usize] > 0 && letter as u8 != answer.as_bytes()[i]
                {
                    print!(
                        "{}{}",
                        ANSI::Preset(Preset::Yellow),
                        (letter as u8 - 32) as char
                    );
                    freq[letter as usize - 'a' as usize] -= 1;
                    continue;
                }
                print!("\x1b[1C");
            }
            itr += 1;
            print!("{}", ANSI::Clear);
            if attempt == answer {
                break;
            }
        }

        print!(
            "{}{}The word was: {}\npress Enter to continue\n{}",
            ANSI::Fore256(3),
            ANSI::Move(Loc::Bottom),
            answer,
            ANSI::Clear
        );

        let ch: Getch = Getch::new();
        while ch.getch().unwrap() != 13 {}
        print!("\r\x1b[2A\x1b[0J")
    }
}
