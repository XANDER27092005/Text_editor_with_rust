//mod other;
use crossterm::event::{Event, KeyCode, KeyEvent}; 
use crossterm::{event, terminal};
//use std::io::{self,Read};
use std::time::Duration;
struct CleanUp;

impl Drop for CleanUp{
    fn drop(&mut self){
        terminal::disable_raw_mode().expect("Could not disable raw mode");
    }
}

struct Reader;

impl Reader{
    fn read_key(&self) -> crossterm::Result<KeyEvent>{
        loop {
            if event::poll(Duration::from_millis(500))?{
                if let Event::Key(event) = event::read()?{
                    return Ok(event);
                }
            }
        }

    }
}

struct Editor{
    reader: Reader
}

impl Editor{
    fn new() -> Self{
        Self { reader: Reader }
    }

    fn process_keypress(&self) -> crossterm::Result<bool>{
        match self.reader.read_key()? {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: event::KeyModifiers::CONTROL,
            } => return Ok(false),
            _ => {}
        }
        Ok(true)
    }
    fn run(&self) -> crossterm::Result<bool> {
        self.process_keypress()
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode()?; 
    let editor = Editor::new();
    while editor.run()? {}
    Ok(())
    // loop {
    //     if event::poll(Duration::from_millis(1000))?{
    //         if let Event::Key(event) = event::read()?{ 
    //             match event {
    //                 KeyEvent {
    //                     code: KeyCode::Char('c'),
    //                     modifiers: event::KeyModifiers::CONTROL,
    //                 } => break,
    //                 _ => {
    //                     //todo
    //                 }
    //             }
    //             println!("{:?}\r", event);
    //         };
    //     } else {
    //         println!("No input yet\r");
    //     }
    // }
    
}


// for b in io::stdin().bytes(){
    //     let c = b.unwrap() as char;
    //     println!("{}", c);
    //     if c == 'q' {
    //         break;
    //     }
    // }

 /*let mut buf = [0; 1];
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf[0] != b'q'{
        let character = buf[0] as char;
        if character.is_control(){
            println!("{}\r", character as u8)
        }else{
            println!("{}\r", character)
        }
        //println!("{:?}",buf[0] as char);
    }
*/
    //terminal::disable_raw_mode().expect("Could not turn off raw mode");