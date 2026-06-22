//Libraries (crates) being used:
use crossterm::event::{self, Event, KeyCode};
use rodio::{OutputStream, Sink, source::{SineWave, Source}};
use std::collections::HashMap;
use std::time::Duration;



fn main() -> Result<(), Box<dyn std::error::Error>> {
    let keyboard_map = build_keyboard_map();
    print_instructions();

    let (_stream, stream_handle) = OutputStream::try_default()?;

    println!("Press piano keys to play notes. Press Esc or Q to quit.");

    loop {
        // Checks for if a key was pressed in the last 100 milliseconds.
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Char(c) => {
                        // Turns user input into constant lowercase.
                        let key = c.to_ascii_lowercase();
                        if let Some((note_name, freq)) = keyboard_map.get(&key) {
                            println!("Playing {} ({})", note_name, key);
                            play_note(*freq, &stream_handle);
                        } else if key == 'q' {
                            break;
                        }
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }
    }

    println!("Goodbye!");
    Ok(())
}

fn build_keyboard_map() -> HashMap<char, (&'static str, f32)> {
    let mut map = HashMap::new();

    // Assigning keys to notes using a HashMap dictionary.
    map.insert('a', ("C4", 261.63));
    map.insert('w', ("C#4", 277.18));
    map.insert('s', ("D4", 293.66));
    map.insert('e', ("D#4", 311.13));
    map.insert('d', ("E4", 329.63));
    map.insert('f', ("F4", 349.23));
    map.insert('t', ("F#4", 369.99));
    map.insert('g', ("G4", 392.00));
    map.insert('y', ("G#4", 415.30));
    map.insert('h', ("A4", 440.00));
    map.insert('u', ("A#4", 466.16));
    map.insert('j', ("B4", 493.88));
    map.insert('i', ("C5", 523.25));

    map
}

fn print_instructions() {
    println!("Welcome to the Rust Piano!");
    println!("The key configuration is as follows:");
    println!("---------------------");
    println!("Natural Piano Keys:");
    println!("C  -> A");
    println!("D  -> S");
    println!("E  -> D");
    println!("F  -> F");
    println!("G  -> G");
    println!("A  -> H");
    println!("B  -> J");
    println!("---------------------");
    println!("Sharp Piano Keys:");
    println!("C# -> W");
    println!("D# -> E");
    println!("F# -> T");
    println!("G# -> Y");
    println!("A# -> U");
    println!("High C -> I");
    println!("---------------------");
}

fn play_note(frequency: f32, stream_handle: &rodio::OutputStreamHandle) {
    if let Ok(sink) = Sink::try_new(stream_handle) {
        let source = SineWave::new(frequency)
            // Limiting the duration of the note to 400 milliseconds. 
            .take_duration(Duration::from_millis(400))
            // Handles the frequency volume.
            .amplify(0.25);
        sink.append(source);
        // Allows for threading, meaning multiple notes can be played at the same time.
        sink.detach();
    }
}
