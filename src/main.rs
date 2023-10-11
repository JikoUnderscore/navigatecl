use crossterm::{
    cursor::MoveTo,
    event,
    terminal::{Clear, ClearType},
};
use hashbrown::HashMap;
// https://stackoverflow.com/questions/40426307/change-terminal-cursor-position-in-rust
// https://github.com/crossterm-rs/crossterm

fn main() {
    let curent_dir = unsafe { std::fs::canonicalize("./").unwrap_unchecked() };
    let mut stderr = std::io::stderr();

    let binding = curent_dir.to_string_lossy();
    let mut split = binding
        .split('\\')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    split.drain(0..3);

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

    print_stuff_while_tabing(&split[0..dir_depth], &mut cash);

    let mut folder_search = String::with_capacity(50);

    let mut input;

    let mut auto_compleate: Option<String> = None;

    let mut is_running = true;
    while is_running {
        // events
        input = get_input();

        // update

        match input.code {
            // event::KeyCode::Char('c') if input.modifiers == event::KeyModifiers::CONTROL => {
            //     is_running = false;
            // }
            event::KeyCode::BackTab => {
                if dir_depth > 1 {
                    dir_depth -= 1;
                }

                // println!("{:?}", &split[0..dir_depth]);
                print_stuff_while_tabing(&split[0..dir_depth], &mut cash);
            }
            event::KeyCode::Tab => {
                if dir_depth < split.len() {
                    dir_depth += 1;
                }

                // println!("{:?}", &split[0..dir_depth]);
                print_stuff_while_tabing(&split[0..dir_depth], &mut cash);
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
                        (*split_ptr).push(folder_search.clone());
                        folder_search.clear();

                        dir_depth = split.len();

                        print_stuff_while_tabing(&split[0..dir_depth], &mut cash);
                    }
                }
            },

            event::KeyCode::Esc => {
                print!("{}", split[0..dir_depth].join("\\"));

                is_running = false;
            }
            event::KeyCode::Char(c) => {
                folder_search.push(c);
                crossterm::execute!(stderr, Clear(ClearType::FromCursorDown)).unwrap();

                eprintln!(">>>{}", folder_search);

                update_stuff_infolder(&split[0..dir_depth], &mut cash);
                let stuffs = cash.get(&split[0..dir_depth]).unwrap();

                let (cursor_x, cursor_y) = crossterm::cursor::position().unwrap();

                let (_, size_y) = crossterm::terminal::size().unwrap();

                let mut n_printed = 0;

                let left_rows_till_end_of_terminal = size_y - cursor_y - 3; // Mimus `3` one for `eprintln!("...");` print; one for it start counting form next line; one for when entering char it goes to the next line i think

                'printloop: for (file_emote, stuf) in stuffs.iter() {
                    if stuf.contains(folder_search.as_str()) {
                        eprintln!("{} {}", file_emote, stuf);
                        if n_printed >= left_rows_till_end_of_terminal {
                            eprintln!("...");
                            break 'printloop;
                        }
                        n_printed += 1;

                        auto_compleate = if n_printed == 1 {
                            Some(stuf.clone())
                        } else {
                            None
                        };
                    }
                }

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

fn get_input() -> Input {
    loop {
        let keyevent = event::read().expect("shit");
        match keyevent {
            event::Event::Key(key) => {
                if key.kind == event::KeyEventKind::Press {
                    return Input {
                        code: key.code,
                        // modifiers: key.modifiers,
                    };
                }
            }
            _ => {}
        }
    }
}

fn update_stuff_infolder<'s>(
    slice: &'s [String],
    cash: &mut HashMap<&'s [String], Vec<(char, String)>>,
) {
    if cash.get(slice).is_none() {
        let new_dir = if slice.len() == 1 {
            format!("{}\\", slice[0])
        } else {
            slice.join("\\")
        };
        let paths = std::fs::read_dir(new_dir).unwrap();

        let mut sb = Vec::with_capacity(50);

        for path in paths {
            let entry = path.unwrap();
            let p = entry.path();

            let file = p.file_name().unwrap();
            let file_type = entry.file_type().unwrap();
            let emote = if file_type.is_dir() { '📁' } else { '📃' };

            sb.push((emote, file.to_string_lossy().into_owned()));
        }

        cash.insert(slice, sb);
    }
}

fn print_stuff_while_tabing<'s>(
    slice: &'s [String],
    cash: &mut HashMap<&'s [String], Vec<(char, String)>>,
) {
    let mut stderr = std::io::stderr();

    crossterm::execute!(stderr, MoveTo(0, 0)).unwrap();

    crossterm::execute!(stderr, Clear(ClearType::All)).unwrap();

    const MAX_FILE_LEN: usize = 20;

    for folder in slice {
        eprint!("{}\\", folder);
    }
    eprintln!();

    let (_, cursor_y) = crossterm::cursor::position().unwrap();

    let (_, size_y) = crossterm::terminal::size().unwrap();

    let mut n_rows_printed = 0;

    let left_rows_till_end_of_terminal = size_y - cursor_y - 5;

    const NUMNER_OF_ITEM_POER_ROW: u8 = 7;

    if let Some(v) = cash.get(slice) {
        let mut i = 0;

        for path in v {
            let emote = path.0;
            let file_len = path.1.len();
            let file_name = path.1.as_str();

            if n_rows_printed >= left_rows_till_end_of_terminal {
                eprint!("...");
                break;
            }

            if i == NUMNER_OF_ITEM_POER_ROW {
                eprintln!();
                i = 0;
                n_rows_printed += 1;

            }

            let end = if file_len < MAX_FILE_LEN {
                file_len
            } else {
                MAX_FILE_LEN
            };

            let file_name_unicode = check_for_unicode_filename(file_name, end);

            if n_rows_printed >= left_rows_till_end_of_terminal {
                eprint!("...");
                break;
            }

            eprint!("{} {:<20}", emote, file_name_unicode);

            i += 1;
        }
    } else {
        let new_dir = if slice.len() == 1 {
            format!("{}\\", slice[0])
        } else {
            slice.join("\\")
        };
        let paths = std::fs::read_dir(new_dir).unwrap();

        let mut i = 0;

        let mut sb = Vec::with_capacity(50);

        let mut is_printing_done = false;

        for path in paths {
            let entry = path.unwrap();
            let p = entry.path();

            let file_type = entry.file_type().unwrap();
            let emote = if file_type.is_dir() { '📁' } else { '📄' };

            if !is_printing_done && n_rows_printed >= left_rows_till_end_of_terminal {
                eprint!("...");
                is_printing_done = true;
            }

            if !is_printing_done {
                if i == NUMNER_OF_ITEM_POER_ROW {
                    if !is_printing_done {
                        eprintln!();
                    }
                    i = 0;
                    n_rows_printed += 1;
                }
            }

            let file = p.file_name().unwrap();
            let file_len = file.len();

            let end = if file_len < MAX_FILE_LEN {
                file_len
            } else {
                MAX_FILE_LEN
            };
            sb.push((emote, file.to_string_lossy().to_lowercase()));

            let f = file.to_str().unwrap();

            let file_name = check_for_unicode_filename(f, end);

            if !is_printing_done {
                eprint!("{} {:<20}", emote, file_name);
            }
            i += 1;
        }
        cash.insert(slice, sb);
    }

    eprintln!();
    // crossterm::execute!(
    //     stderr,
    //     crossterm::cursor::MoveTo(cursor_x, cursor_y - 1), // and here we do `-1`, i forgot why, i never knew why
    // )
    // .unwrap();
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
