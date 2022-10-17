// Start the game loop

use std::io;

use hallo::{build_story, StoryEvent};


fn main() {


    let mut gameover = false;

    let mut user_input = String::new();
    let stdin = io::stdin();

    let mut story = build_story();

    println!{"You wake up in a strange land..."};

    while !gameover {

        let _input = stdin.read_line(&mut user_input);
 
        let event = story.next_event(&mut user_input);

        match event {
            Some(StoryEvent::ContinueEvent(message)) => {
                println!("{}", message);
            },
            Some(StoryEvent::EndEvent(message)) => {
                println!("{}", message);
                gameover = true
            }
            None => {
                println!("You've reached a strange dimension with no escape.");
                gameover = true
            }
        }

    }
    
}
