use core::fmt::Display;
use std::fmt::{self, Formatter};
use std::io::stdout;
use std::io::Write;
use std::process;

use getch::Getch;
use rand::seq::SliceRandom;

use crate::parsing::load_into_vec;
mod parsing;

// im sorry
/// Maps Chars to loction on the printed keybord
const LAYOUT: [(char, (i32, i32)); 26] = [
    ('Q', (0, 0)),
    ('W', (2, 0)),
    ('E', (4, 0)),
    ('R', (6, 0)),
    ('T', (8, 0)),
    ('Y', (10, 0)),
    ('U', (12, 0)),
    ('I', (14, 0)),
    ('O', (16, 0)),
    ('P', (18, 0)),
    //
    ('A', (1, 1)),
    ('S', (3, 1)),
    ('D', (5, 1)),
    ('F', (7, 1)),
    ('G', (9, 1)),
    ('H', (11, 1)),
    ('J', (13, 1)),
    ('K', (15, 1)),
    ('L', (17, 1)),
    //
    ('Z', (2, 2)),
    ('X', (4, 2)),
    ('C', (6, 2)),
    ('V', (8, 2)),
    ('B', (10, 2)),
    ('N', (12, 2)),
    ('M', (14, 2)),
];

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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum LettorState {
    Location,
    Lettor,
    Wrong,
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
            65..=90 => {
                print!("{}", key as char);
                stdout().flush().unwrap();
                out[len] = (key + 32) as char;
                len += 1;
            }
            8 | 33 => {
                if len > 0 {
                    len -= 1;
                    print!("{}*{}", ANSI::Back, ANSI::Back);
                    stdout().flush().unwrap();
                }
            }
            10 | 13 => return "jank workaround lol dont mind me".to_string(),
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

        let mut tried_letters = [LettorState::Wrong; 26];

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
    ***********          Z X C V B N M
    ***********
",
            ANSI::Move(Loc::Start)
        );
        let mut itr = 0;
        while itr < 6 {
            refresh_keybord(tried_letters);
            print!("{}", ANSI::Move(Loc::Word(itr)));
            stdout().flush().unwrap();
            let attempt: String = get_word();
            if attempt == "jank workaround lol dont mind me".to_string() { break; }
            // oh yay we found it

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

                    let index = letter as usize - 'a' as usize;

                    tried_letters[index] = LettorState::Location;
                    freq[index] -= 1;
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

                    let index = letter as usize - 'a' as usize;

                    if tried_letters[index] != LettorState::Location {
                        tried_letters[index] = LettorState::Lettor;
                    }

                    freq[index] -= 1;
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

fn refresh_keybord(state: [LettorState; 26]) {
    for (index, i) in state.iter().enumerate() {
        let letter = (index as u8 + 'A' as u8) as char;

        let move_factor = LAYOUT.iter().find(|x| x.0 == letter).unwrap().1;
        print!(
            "{}",
            ANSI::MoveCursor(13 + move_factor.1, 24 + move_factor.0)
        );

        match i {
            LettorState::Location => print!("{}{}", ANSI::Preset(Preset::Green), letter),
            LettorState::Lettor => print!("{}{}", ANSI::Preset(Preset::Yellow), letter),
            LettorState::Wrong => print!("{}", letter),
        }

        print!("\x1b[0m");
    }

    stdout().flush().unwrap();
}
