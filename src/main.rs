#![allow(clippy::needless_return)]
use std::rc::Rc;

use crossterm::{
    cursor::MoveTo,
    event,
    terminal::{Clear, ClearType},
};
use hashbrown::HashMap;

static FOLDER_ICON: char = '📁';
static FILE_ICON: char = '📃';

// https://stackoverflow.com/questions/40426307/change-terminal-cursor-position-in-rust
// https://github.com/crossterm-rs/crossterm

fn main() {
    let curent_dir = unsafe { std::fs::canonicalize("./").unwrap_unchecked() };
    let mut stderr = std::io::stderr();

    let binding = curent_dir.to_string_lossy();
    let mut split = binding.split('\\').map(Rc::from).collect::<Vec<_>>();
    // split.drain(0..3);

    let split_ptr: *mut _ = &mut split;

    // let split = curent_dir
    //     .components()
    //     .map(|c| {
    //         c.

    //         c.as_os_str().to_str().expect("no errors")
    //     }
    //     )
    //     .collect::<Vec<_>>();
    let mut cash = HashMap::with_capacity(50);

    let mut dir_depth = split.len();

    print_while_tabing(&split[0..dir_depth], &mut cash);

    let mut folder_search = String::with_capacity(50);

    let mut input;

    let mut auto_compleate: Option<String> = None;

    let mut is_running = true;
    while is_running {
        // events
        input = get_input();
        // update
        match input.code {
            event::KeyCode::BackTab => {
                if dir_depth > 1 {
                    dir_depth -= 1;
                }
                print_while_tabing(&split[0..dir_depth], &mut cash);
            }

            event::KeyCode::Tab => {
                if dir_depth < split.len() {
                    dir_depth += 1;
                }
                print_while_tabing(&split[0..dir_depth], &mut cash);
            }

            event::KeyCode::F(1) => {
                if let Some(mut ac) = auto_compleate {
                    let (cursor_x, cursor_y) = crossterm::cursor::position().unwrap();

                    std::mem::swap(&mut folder_search, &mut ac);
                    auto_compleate = None;
                    eprintln!(">>>{}", folder_search);

                    crossterm::execute!(stderr, crossterm::cursor::MoveTo(cursor_x, cursor_y),)
                        .unwrap();
                }
            }

            event::KeyCode::Backspace => {
                crossterm::execute!(stderr, Clear(ClearType::FromCursorDown)).unwrap();

                folder_search.pop();
                let (cursor_x, cursor_y) = crossterm::cursor::position().unwrap();

                eprintln!(">>>{}", folder_search);

                // if folder_search.len() > 1 {
                //     update_stuff_infolder(&split[0..dir_depth], &mut cash);

                //     let stuffs = cash.get(&split[0..dir_depth]).unwrap();

                //     let (_, size_y) = crossterm::terminal::size().unwrap();

                //     let mut n_printed = 0;

                //     let left_rows_till_end_of_terminal = size_y - cursor_y - 3; // Mimus `3` one for `eprintln!("...");` print; one for it start counting form next line; one for when entering char it goes to the next line i think

                //     'printloop: for (file_emote, stuf) in stuffs.iter() {
                //         if stuf.contains(folder_search.as_str()) {
                //             eprintln!("{} {}", file_emote, stuf);
                //             if n_printed >= left_rows_till_end_of_terminal {
                //                 eprintln!("...");
                //                 break 'printloop;
                //             }
                //             n_printed += 1;
                //             auto_compleate = if n_printed == 1 {
                //                 Some(stuf.clone())
                //             } else {
                //                 None
                //             };
                //         }
                //     }
                // }

                crossterm::execute!(stderr, crossterm::cursor::MoveTo(cursor_x, cursor_y),)
                    .unwrap();
            }

            event::KeyCode::Enter => unsafe {
                let mut path = split[0..dir_depth].join("\\");
                path.push('\\');
                path.push_str(&folder_search);

                if let Ok(file_metadata) = std::fs::metadata(path.as_str()) {
                    if file_metadata.is_dir() {
                        (*split_ptr).truncate(dir_depth);
                        (*split_ptr).push(Rc::from(folder_search.clone()));
                        folder_search.clear();

                        dir_depth = split.len();

                        print_while_tabing(&split[0..dir_depth], &mut cash);
                    }
                }
            },

            event::KeyCode::Esc => {
                print!("{}", split[3..dir_depth].join("\\"));

                is_running = false;
            }
            event::KeyCode::Char(c) => {
                folder_search.push(c);
                crossterm::execute!(stderr, Clear(ClearType::FromCursorDown)).unwrap();

                eprintln!(">>>{}", folder_search);

                if cash.get(&split[0..dir_depth]).is_none() {
                    let sbuilder: Vec<(char, String)> =
                        get_update_stuff_in_folder(&split[0..dir_depth]);
                    cash.insert(&split[0..dir_depth], sbuilder);
                }

                let (cursor_x, cursor_y) = print_while_geting_input(
                    cash.get(&split[0..dir_depth]).unwrap(),
                    &folder_search,
                    &mut auto_compleate,
                );

                crossterm::execute!(
                    stderr,
                    crossterm::cursor::MoveTo(cursor_x, cursor_y - 1), // and here we do `-1`, i forgot why, i never knew why
                )
                .unwrap();
            }
            _ => {}
        }
    }
}

fn print_while_geting_input(
    stuffs: &[(char, String)],
    folder_search: &str,
    auto_compleate: &mut Option<String>,
) -> (u16, u16) {
    let (cursor_x, cursor_y) = crossterm::cursor::position().unwrap();

    let (_, size_y) = crossterm::terminal::size().unwrap();

    let mut n_printed = 0;

    // Mimus `3` one for `eprintln!("...");` print; one for it start counting form next line; one for when entering char it goes to the next line i think
    let left_rows_till_end_of_terminal = size_y - cursor_y - 3;

    'printloop: for (file_emote, stuf) in stuffs.iter() {
        if stuf.contains(folder_search) {
            eprintln!("{} {}", file_emote, stuf);
            if n_printed >= left_rows_till_end_of_terminal {
                eprintln!("...");
                break 'printloop;
            }
            n_printed += 1;

            *auto_compleate = if n_printed == 1 {
                Some(stuf.clone())
            } else {
                None
            };
        }
    }
    (cursor_x, cursor_y)
}

fn get_input() -> Input {
    loop {
        let keyevent = event::read().expect("shit");
        if let event::Event::Key(key) = keyevent {
            if key.kind == event::KeyEventKind::Press {
                return Input { code: key.code };
            }
        }
    }
}

fn get_update_stuff_in_folder(
    slice: &[Rc<str>],
    // cash: &HashMap<&'_ [Rc<str>], Vec<(char, String)>>,
) -> Vec<(char, String)> {
    // if cash.get(slice).is_some() {
    //     return;
    // }
    let new_dir = if slice.len() == 1 {
        format!("{}\\", slice[0])
    } else {
        slice.join("\\")
    };
    let paths = std::fs::read_dir(new_dir).unwrap();

    let mut sbuilder = Vec::with_capacity(50);

    for path in paths {
        let entry = path.unwrap();
        let p = entry.path();

        let file = p.file_name().unwrap();
        let file_type = entry.file_type().unwrap();
        let emote = if file_type.is_dir() {
            FOLDER_ICON
        } else {
            FILE_ICON
        };

        sbuilder.push((emote, file.to_string_lossy().into_owned()));
    }

    return sbuilder;
}

const MAX_FILE_LEN: usize = 20;
const NUMNER_OF_ITEM_POER_ROW: u16 = 7;

fn print_while_tabing<'s>(
    slice: &'s [Rc<str>],
    cash: &mut HashMap<&'s [Rc<str>], Vec<(char, String)>>,
) {
    let mut stderr = std::io::stderr();

    crossterm::execute!(stderr, MoveTo(0, 0)).unwrap();

    crossterm::execute!(stderr, Clear(ClearType::All)).unwrap();

    for folder in slice {
        eprint!("{}\\", folder);
    }
    eprintln!();

    let (_, cursor_y) = crossterm::cursor::position().unwrap();

    let (size_x, size_y) = crossterm::terminal::size().unwrap();

    let mut n_rows_printed = 0;

    let left_rows_till_end_of_terminal = size_y - cursor_y - 5;

    let numer_of_items_per_row =
        std::cmp::min((size_x / MAX_FILE_LEN as u16) - 1, NUMNER_OF_ITEM_POER_ROW);

    if let Some(v) = cash.get(slice) {
        tab_printing(
            v,
            &mut n_rows_printed,
            left_rows_till_end_of_terminal,
            numer_of_items_per_row,
        );
    } else {
        let new_dir = if slice.len() == 1 {
            format!("{}\\", slice[0])
        } else {
            slice.join("\\")
        };
        let paths = std::fs::read_dir(new_dir).unwrap();

        let mut i = 0;

        let mut sbuilder = Vec::with_capacity(50);

        let mut is_printing_done = false;

        for path in paths {
            let entry = path.unwrap();
            let p = entry.path();

            let file = p.file_name().unwrap();
            let file_type = entry.file_type().unwrap();
            let emote = if file_type.is_dir() {
                FOLDER_ICON
            } else {
                FILE_ICON
            };

            sbuilder.push((emote, file.to_string_lossy().to_lowercase()));

            let file_len = file.len();

            let end = if file_len < MAX_FILE_LEN {
                file_len
            } else {
                MAX_FILE_LEN
            };

            let file_name_unicode = check_for_unicode_filename(file.to_str().unwrap(), end);

            if !is_printing_done && n_rows_printed >= left_rows_till_end_of_terminal {
                eprint!("...");
                is_printing_done = true;
            }
            if !is_printing_done && i == numer_of_items_per_row {
                if !is_printing_done {
                    eprintln!();
                }
                i = 0;
                n_rows_printed += 1;
            }
            if !is_printing_done {
                eprint!("{} {:<20}", emote, file_name_unicode);
            }
            i += 1;
        }

        cash.insert(slice, sbuilder);
    }

    eprintln!();
    // crossterm::execute!(
    //     stderr,
    //     crossterm::cursor::MoveTo(cursor_x, cursor_y - 1), // and here we do `-1`, i forgot why, i never knew why
    // )
    // .unwrap();
}

fn tab_printing(
    v: &Vec<(char, String)>,
    n_rows_printed: &mut u16,
    left_rows_till_end_of_terminal: u16,
    numer_of_items_per_row: u16,
) {
    let mut i = 0;

    for path in v {
        let emote = path.0;
        let file_name = path.1.as_str();

        let file_len = path.1.len();
        let end = if file_len < MAX_FILE_LEN {
            file_len
        } else {
            MAX_FILE_LEN
        };

        let file_name_unicode = check_for_unicode_filename(file_name, end);

        if *n_rows_printed >= left_rows_till_end_of_terminal {
            eprint!("...");
            break;
        }
        if i == numer_of_items_per_row {
            eprintln!();
            i = 0;
            *n_rows_printed += 1;
        }

        eprint!("{} {:<20}", emote, file_name_unicode);

        i += 1;
    }
}

fn check_for_unicode_filename(f: &str, end: usize) -> &str {
    let mut i = 0;
    loop {
        let item = f.get(0..end + i);

        if let Some(v) = item {
            return v;
        }
        i += 1;
    }
}

#[derive(Debug)]
struct Input {
    pub code: event::KeyCode,
    // pub modifiers: event::KeyModifiers,
}
