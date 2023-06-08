mod story;
mod display;


fn main() {
    match ctrlc::set_handler(|| {println!("Exiting game!"); std::process::exit(0);}) {
        Ok(_) => {},
        Err(_) => eprint!("Couldn't set exit handler!")
    };

    let mut story_pos = story::StoryTraverser::new();
    let nodes = story::StoryContainer::new();
    display::startup();

    loop{
        // Present the current node to the user & get the choice they picked
        let current_node = nodes.get_node(story_pos.current_node);
        let choice = display::present_node(current_node);

        // make sure the choice is unlocked; they have all requirements
        if !story_pos.is_choice_unlocked(choice) {
            display::show_choice_locked(choice);
            continue;
        }

        // updating the display & event records to pick the choice
        display::pick_choice(choice);
        story_pos.pick_choice(choice);

        // update the events record & player for each event skipped
        for skipped in &current_node.options{
            if std::ptr::eq(skipped, choice){
                continue;
            }
            story_pos.skip_choice(skipped);
            display::skip_choice(skipped);
        }

        // Some nodes have a "go back" option, check it exists & wasn't chosen, then skip it
        let prev = &current_node.prev_node.as_ref();
        if prev.is_some() && !std::ptr::eq(prev.unwrap(), choice){
            story_pos.skip_choice(prev.unwrap());
            display::skip_choice(prev.unwrap());
        }

        // update the new current node
        story_pos.current_node = choice.destination_node;
    }
}
