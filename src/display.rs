use std::collections::HashMap;
use crate::story;

pub fn startup(){
    // boot message
    todo!();
    // first prompt
}

pub fn display_node(node: &story::StoryNode){
    todo!();
}

pub fn get_selection_from_node(node: &story::StoryNode) -> &story::StoryChoice{
    todo!()
}

pub fn evaluate_choice(story: &story::Story, events: &mut HashMap<story::StoryEvent, bool>,choice: &story::StoryChoice) -> bool{
    todo!()
}

pub fn choice_failed(){
    todo!()
}