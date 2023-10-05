const CTRL_A: char = '\u{1}';
const CTRL_D: char = '\u{4}';
const CTRL_C: char = '\u{3}';
const BACKSPACE: char = '\u{8}';
const ENTER: char = '\r';
// https://stackoverflow.com/questions/40426307/change-terminal-cursor-position-in-rust
// https://github.com/crossterm-rs/crossterm

fn main() {
    let curent_dir = unsafe { std::fs::canonicalize("./").unwrap_unchecked() };

    let binding = curent_dir.to_string_lossy();
    let mut split = binding.split('\\').collect::<Vec<_>>();
    split.drain(0..3);
    // for w in curent_dir.iter() {
    //     if '\\' != w.to_str().unwrap().chars().nth(0).unwrap() {

    //     }
    //     println!("{:?}", w);

    // }
    let mut dir_depth = split.len();

    print_stuff_in_folder(&split[0..dir_depth]);

    let mut folder_search = String::with_capacity(50);

    let mut is_running = true;
    while is_running {
        let char_input = getch();
        // println!("[TEST] {:?}", char_input);

        match char_input {
            CTRL_C => {
                is_running = false;
            }
            CTRL_A => {
                if dir_depth > 1 {
                    dir_depth -= 1;
                }

                // println!("{:?}", &split[0..dir_depth]);
                print_stuff_in_folder(&split[0..dir_depth]);
            }
            CTRL_D => {
                if dir_depth < split.len() {
                    dir_depth += 1;
                }
                // println!("{:?}", &split[0..dir_depth]);
                print_stuff_in_folder(&split[0..dir_depth]);
            }
            BACKSPACE => {
                folder_search.pop();

                eprintln!(">{}", folder_search);
                // std::process::Command::new("cmd")
                //     .args(["/c", "cls"])
                //     .spawn()
                //     .expect("cls command failed to start")
                //     .wait()
                //     .expect("failed to wait");
            }

            ENTER => {
                // let paths = std::fs::read_dir(result).unwrap();
                let a = split[0..dir_depth].join("\\");
                print!("{}", a);

                // match std::env::set_current_dir(&a) {
                //     Ok(()) => eprintln!("Successfully changed working directory to {}", a),
                //     Err(e) => eprintln!("Failed to change working directory: {}", e),
                // }
                is_running = false;
            }
            _ => {
                folder_search.push(char_input);
                // println!("{:?}", &split[0..dir_depth]);

                // std::process::Command::new("cmd")
                //     .args(["/c", "cls"])
                //     .spawn()
                //     .expect("cls command failed to start")
                //     .wait()
                //     .expect("failed to wait");

                eprintln!(">{}", folder_search);

                let stuffs = get_stuff_infolder(&split[0..dir_depth]);
                // dbg!(&stuffs);

                // eprintln!("===========");
                for (file_emote, stuf) in stuffs.iter() {
                    if stuf.contains(folder_search.as_str()) {
                        eprintln!("{} {}", file_emote, stuf);
                    }
                }
                // eprintln!("===========");
                // std::process::Command::new("cmd")
                //     .args(["/c", "cls"])
                //     .spawn()
                //     .expect("cls command failed to start")
                //     .wait()
                //     .expect("failed to wait");
            }
        }
    }
}

fn get_stuff_infolder(slice: &[&str]) -> Vec<(char, String)> {
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

    return sb;
}

fn print_stuff_in_folder(slice: &[&str]) {
    // dbg!(slice);
    // std::process::Command::new("cmd")
    //     .args(["/c", "cls"])
    //     .spawn()
    //     .expect("cls command failed to start")
    //     .wait()
    //     .expect("failed to wait");

    let new_dir = if slice.len() == 1 {
        format!("{}\\", slice[0])
    } else {
        slice.join("\\")
    };

    let paths = std::fs::read_dir(new_dir).unwrap();

    eprintln!("${:?}", paths);
    let mut i = 0;

    for path in paths {
        let entry = path.unwrap();
        let p = entry.path();

        let file_type = entry.file_type().unwrap();
        let emote = if file_type.is_dir() { '📁' } else { '📄' };
        if i == 5 {
            eprintln!();
            i = 0;
        }

        let file = p.file_name().unwrap().to_str().unwrap();
        let file_len = file.len();

        const MAX_FILE_LEN: usize = 20;
        let end = if file_len < MAX_FILE_LEN {
            file_len
        } else {
            MAX_FILE_LEN
        };

        eprint!("{} {:<20}", emote, &file[0..end]);
        i += 1;
    }

    eprintln!();
}

extern "C" {
    // Windows only
    fn _getch() -> core::ffi::c_char;
}

fn getch() -> char {
    unsafe { _getch() as u8 as char }
}
