mod story;
mod display;


fn main() {
    let mut story_pos = story::StoryTraverser::new();
    let nodes = story::StoryContainer::new();
    display::startup();

    loop{
        let current_node = nodes.get_node(story_pos.current_node);
        // TODO: present node needs to allow some nodes to go back
        let choice = display::present_node(current_node); // Display node & have user pick

        if !story_pos.is_choice_unlocked(choice) {
            display::show_choice_locked(choice);
            continue;
        }
        
        display::pick_choice(choice);
        story_pos.pick_choice(choice);

        for skipped in &current_node.options{
            if std::ptr::eq(skipped, choice){
                continue;
            }
            story_pos.skip_choice(skipped);
        }

        story_pos.current_node = choice.destination_node;
    }
}
