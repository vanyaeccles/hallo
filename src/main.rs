// Start the game loop

use std::io;

use hallo::{build_story, StoryEvent};


fn main() {

    let mut user_input = String::new();
    let stdin = io::stdin();

    let mut story = build_story();
    let mut gameover = false;

    let mut event;

    while !gameover {

        event = story.next_event(&mut user_input);

        match event {
            StoryEvent::ContinueEvent(message) => {
                println!("{}", message);

                let _input = stdin.read_line(&mut user_input);
            },
            StoryEvent::EndEvent(message) => {
                println!("{}", message);
                gameover = true
            }
        }
    }
    
}
