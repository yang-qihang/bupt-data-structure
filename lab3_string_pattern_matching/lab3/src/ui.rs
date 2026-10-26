use std::io::{self, Write};

pub fn read_choice() -> u8 {
    loop {
        print!("Please enter your choice: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<u8>() {
            Ok(choice) => return choice,
            Err(_) => println!("Invalid input. Please enter an integer from 0 to 255"),
        }
    }
}

/// Clear the terminal screen and move the cursor to the top-left corner.
pub fn clear_screen() {
    // On Windows the legacy console (conhost) ignores ANSI escape sequences
    // unless virtual-terminal processing is enabled, so enable it first. This
    // is a no-op on terminals that already support ANSI (Windows Terminal,
    // VS Code, macOS/Linux).
    #[cfg(windows)]
    enable_vt_processing();

    // `\x1b[2J` erases the visible screen, `\x1b[3J` erases the scrollback
    // buffer, and `\x1b[H` moves the cursor home. All three are needed so that
    // old output can't be scrolled back into view.
    print!("\x1b[2J\x1b[3J\x1b[H");
    let _ = io::stdout().flush();
}

/// Print a page title followed by a thin underline and a blank line.
///
/// The underline is a single row of `-` characters, so the title stays visually
/// distinct without the heavy `===` borders the earlier version used.
pub fn print_page_title(title: &str) {
    println!();
    println!("{}", "-".repeat(title.chars().count()));
    println!("{title}");
    println!("{}", "-".repeat(title.chars().count()));
    println!();
}

/// Print a "Press Enter to continue" prompt and wait for the user to press
/// Enter. Keeps an operation's result on screen until the user is ready to
/// return to the menu (which then clears the screen).
pub fn wait_for_enter() {
    println!();
    println!("Press Enter to continue...");

    let mut input = String::new();

    if let Err(error) = io::stdin().read_line(&mut input) {
        eprintln!("Failed to read input: {error}");
    }
}

#[cfg(windows)]
fn enable_vt_processing() {
    const STD_OUTPUT_HANDLE: i32 = -11;
    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

    unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if handle.is_null() {
            return;
        }

        let mut mode: u32 = 0;
        if GetConsoleMode(handle, &mut mode) == 0 {
            return;
        }

        // Setting this flag is what makes `\x1b[...` sequences work on the
        // classic Windows console. Ignore the result: it only fails when stdout
        // is not a real console, in which case there is no screen to clear.
        let _ = SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
    }
}

#[cfg(windows)]
#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetStdHandle(n_std_handle: i32) -> *mut std::ffi::c_void;
    fn GetConsoleMode(h_console_handle: *mut std::ffi::c_void, lp_mode: *mut u32) -> i32;
    fn SetConsoleMode(h_console_handle: *mut std::ffi::c_void, dw_mode: u32) -> i32;
}
