use crate::story::NodeID;

mod story;
mod display;


fn main() {
    let mut story = story::Story::new();
    display::startup();

    loop{
        let current_node = story.get_current_node();
        let choice = display::present_node(current_node); // Display node & have user pick

        if !story::is_choice_unlocked(&story, choice) {
            display::show_choice_locked(choice);
            continue;
        }

        display::pick_choice(choice);

        story::pick_choice(&mut story, choice);
        for skipped in &current_node.options{
            if std::ptr::eq(skipped, choice){
                continue;
            }
            story::skip_choice(&mut story, skipped);
        }

        story.current_node = choice.destination_node;
    }
}