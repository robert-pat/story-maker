use std::collections::HashMap;

mod story;
mod display;


fn main() {
    let mut story = story::Story::new();
    let mut events: HashMap<story::StoryEvent, bool> = HashMap::new();
    display::startup();

    loop{
        let choice = display::get_selection_from_node(story.get_current_node());

        if display::evaluate_choice(&story, &mut events, choice){
            story.current_node = choice.destination_node;
            if story.get_node(choice.destination_node).prev_node.is_some(){

            }
        }
        else{
            display::choice_failed();
            continue;
        }
    }
}
