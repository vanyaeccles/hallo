use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone)]
pub enum StoryEvent {
    ContinueEvent(String),
    EndEvent(String)
}

pub struct Story {
    pub story_events: VecDeque<StoryEvent>,
    pub current_event: StoryEvent,
    pub world_state: HashMap<String, bool>,
}
impl Story {
    pub fn next_event(&mut self, input: &mut String) -> &StoryEvent {

        //parse the choice from the input
        // alter world state
        // present options

        let event_option = self.story_events.pop_front();

        self.current_event = match event_option {
            Some(event) => event,
            None => {
                StoryEvent::EndEvent(String::from("You've reached a strange dimension from which there is no escape."))
            }
        };
        return & self.current_event
    }
}



pub fn build_story() -> Story {

    let mut story_buf = VecDeque::new();
    story_buf.push_back(StoryEvent::ContinueEvent(String::from("You wake up in a strange land...")));
    story_buf.push_back(StoryEvent::ContinueEvent(String::from("First step...")));
    story_buf.push_back(StoryEvent::ContinueEvent(String::from("Second step...")));
    story_buf.push_back(StoryEvent::EndEvent(String::from("Your dead!")));

    let story = Story{ 
        story_events: story_buf,
        current_event: StoryEvent::ContinueEvent(String::from("")),
    };

    return story
}