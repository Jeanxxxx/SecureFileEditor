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
        if event::poll(Duration::from_millis(5000)).expect("Error") {   
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
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        let file_path = &args[1];
        let file : File = match FileIO::get_file(file_path) {
            Some(f) => f,
            None => FileIO::create_file(file_path), // This is where we could ask the user what to do
        };
        
        let test = FileIO::read_from_file(file_path).unwrap();
        println!("read: {}", test);
        let worked : bool = FileIO::append_to_file(file_path, &String::from("more text")).unwrap();
        if worked {
            println!("Write successful");
        } else {
            println!("Problem writing to the file");
        }
    } else {
        println!("Editor Error: no file name provided");
    }
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

    fn write_to_file(pathname : &String, new_text : &String) -> Result<bool, io::Error> {
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
