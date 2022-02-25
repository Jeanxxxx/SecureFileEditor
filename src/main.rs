use std::io::{Read, stdout, Write, ErrorKind};
use std::fs::{File, OpenOptions};
use std::{cmp, env, fs, io};

use crossterm::{event, terminal, execute, cursor, queue};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::terminal::ClearType;

/*//UP-TO: part 2
// Part 3 will introduce more input handling as well as 
//  allowing the user to move the cursor around on the screen.

// The linked tutorial page explains each bit of code in detail well 
//  enough, but I have added some comments in the code to try
//  and explain some things not covered yet in class

use std::io;        //bring in io library to read usr input
use std::io::Read;  //trait that provides the read() function
use std::time::Duration; //for timeout

use crossterm::{event, terminal};    //crate containing function for raw mode
                            //crate also for interacting with the terminal
use crossterm::event::{Event, KeyCode, KeyEvent};

struct CleanUp;
impl Drop for CleanUp {     //implementation block
    fn drop(&mut self) {    //called when a CleanUp object goes out of scope
        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?; //? operator only for returning Option or Result

    loop {
        if event::poll(Duration::from_millis(5000)).expect("Error") {                           // Could use this as inspiration for an auto save system...
            if let Event::Key(event) = event::read().expect("Failed to read line") {
                match event {   //event (enum) returned by event::read()
                    KeyEvent {
                      code: KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::NONE,
                    } => break, //if event is a pressed key 'q' without modifiers, exit
                 _ => {
                     //todo
                 }
             }
             println!("{:?}\r", event);
          };
        } else {
            break;
        } //if-else for terminal timeout (break means exit the loop from user's inactivity)
    }
    
    Ok(())
} */


fn main() {
        // SETUP
    //introduce Tidy_Up instance so that raw mode is disabled at end of main
    let _tidy_up = Tidy_Up;
    
    // If the user is working on a saved file, it will hold the path to the target file
    // If the user is working on an unsaved file, it will hold None
    let opened_file : Option<String> = {
        let args: Vec<String> = env::args().collect();
        if args.len() >= 2 {
            let file_path = &args[1];
            match FileIO::get_file(file_path) {
                Some(_f) => Some(String::from(file_path)),
                None => None
            }
        } else {
            None
        }
    };

    let mut on_screen : Display = Display::new();
    match &opened_file {
        Some(f) => {
            let test = FileIO::read_from_file(&f);
            match test {
                Ok(f) => on_screen.set_contents(String::from(f)),
                Err(e) => {
                    eprintln!("{}", e);
                    panic!("ERROR");
                }
            }
        },
        None => on_screen.set_contents(String::new())
    }

    println!("read:\n{}", on_screen.contents);
    
    crossterm::terminal::enable_raw_mode();

        //PROGRAM RUNNING
    loop {
        // DISPLAY TEXT (from on_screen.contents) HERE


        // Append test
        on_screen.insert_content_here(0, String::from("more text"));
        let worked : bool = FileIO::overwrite_to_file(&opened_file.unwrap(), &on_screen.contents).unwrap();
        if worked {
            println!("Write successful");
        } else {
            println!("Problem writing to the file");
        }
        break

        //render to user save question
    }
        // EXIT
}


// Deals with all the reading and writing to the file
struct FileIO;
impl FileIO {
    /* Read from the file */
    fn read_from_file(pathname: &String) -> Result<String, io::Error> {
        let mut data = String::new();
        File::open(pathname)?.read_to_string(&mut data)?;
        Ok(data)
    }

    fn read_from_file_object(mut file : &File) -> Result<String, io::Error> {
        let mut output = String::new();
        file.read_to_string(& mut output)?;
        Ok(output)
    }

    // Gets the file at the given location, returns None if it does not exist
    fn get_file(file_path : &String) -> Option<File> {
        let f = File::open(file_path);
        match f {
            Ok(file) => Some(file),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => None,
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            },
        } 
    }

    fn create_file(file_path : &String) -> File {
        match File::create(file_path) {
            Ok(fc) => fc,
            Err(e) => panic!("Problem creating the file: {:?}", e),
        }
    }

    fn append_to_file(pathname : &String, new_text : &String) -> Result<bool, io::Error> {
        let mut file = OpenOptions::new().write(true).append(true).open(pathname).unwrap();
        write!(file, "{}", new_text)?;
        Ok(true)
    }

    fn overwrite_to_file(pathname : &String, new_text : &String) -> Result<bool, io::Error> {
        FileIO::create_file(pathname); // If applied to a file that exists it whipes the file contents
        FileIO::append_to_file(pathname, new_text)
    }

    fn print_metadata(file : File) {
        let debug = true;
        let metadata = match file.metadata() {
            Err(e) => panic!("Could not get metadata from file: {}", e),
            Ok(f) => f,
        };
        if debug {
            print!("{:#?}", metadata);
        };
    }
}

/*
    Struct responsible for moving the user's (i)nsertion (p)oint while
    the program is running.
*/
struct KeyHandler {
    ip_x: usize,
    ip_y: usize,
    screen_cols: usize,
    screen_rows: usize,
}
impl KeyHandler {
    //create new KeyHandler with insertion point at origin (top-left corner)
    fn new(window_size: (usize, usize)) -> KeyHandler {
        KeyHandler {
            ip_x: 0,
            ip_y: 0,
            screen_cols: window_size.0,
            screen_rows: window_size.1,
        }
    }

    //move the insertion point based on user's keypress
    fn move_ip(&mut self, operation: KeyCode) {
        match operation {
            KeyCode::Up => {
                if self.ip_y < 1 {
                    self.ip_y = 0;
                } else {
                    self.ip_y -= 1;
                }
            },
            KeyCode::Down => {
                if self.ip_y != self.screen_rows - 1 {
                    self.ip_y += 1;
                }
            },
            KeyCode::Left => {
                if self.ip_x != 0 {
                    self.ip_x -= 1;
                }
            },
            KeyCode::Right => {
                if self.ip_x != self.screen_cols - 1 {
                    self.ip_x += 1;
                }
            },
            _ => {}
        }
    }

    //Backspace and moving forward when typing

    fn getCurrentLocationInString(&mut self) -> usize{
        let x = self.ip_y*self.screen_cols + self.ip_x; //Does not deal with screen having been scrolled?
        x
    }
}

/*
    Struct for displaying file contents to user
*/
struct Display {
    contents : String,
}
impl Display {
    fn new() -> Display {
        Display {
            contents: String::new(),
        }
    }
    
    fn set_contents(&mut self, new_contents : String) {
        self.contents = new_contents;
    }
    
    fn insert_content_here(&mut self, before_here : usize, new_contents : String) {
        let mut result = String::from("");
        for a in self.contents[..before_here].chars() {
            result.push(a);
        }
        for a in new_contents.chars() {
            result.push(a);
        }
        for a in self.contents[before_here..].chars() {
            result.push(a);
        }
        self.contents = result;
    }
}

/*
    Struct for disabling raw mode on program exit (when instance is dropped)
*/
struct Tidy_Up;
impl Drop for Tidy_Up {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode terminal");

    }
}
