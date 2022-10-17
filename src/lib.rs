
use std::collections::VecDeque;

pub enum StoryEvent {
    ContinueEvent(String),
    EndEvent(String)
}

pub struct Story {
    pub story_events: VecDeque<StoryEvent>,
}
impl Story {
    pub fn next_event(&mut self, input: &mut String) -> Option<StoryEvent> {

        //println!("You do {}", input);
        return self.story_events.pop_front()
    }
}



pub fn build_story() -> Story {

    let mut story_buf = VecDeque::new();
    story_buf.push_back(StoryEvent::ContinueEvent(String::from("First step...")));
    story_buf.push_back(StoryEvent::ContinueEvent(String::from("Second step...")));
    story_buf.push_back(StoryEvent::EndEvent(String::from("Your dead!")));

    let story = Story{ 
        story_events: story_buf,
    };

    return story
}