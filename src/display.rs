use crate::story;

pub fn startup(){
    // boot message
    println!(r"
 .----------------.  .----------------.  .----------------.  .----------------.  .----------------.
| .--------------. || .--------------. || .--------------. || .--------------. || .--------------. |
| |      __      | || |  ________    | || | ____   ____  | || |    ___       | || | _____  _____ | |
| |     /  \     | || | |_   ___ `.  | || ||_  _| |_  _| | || |  .' _ '.     | || ||_   _||_   _|| |
| |    / /\ \    | || |   | |   `. \ | || |  \ \   / /   | || |  | (_) '___  | || |  | |    | |  | |
| |   / ____ \   | || |   | |    | | | || |   \ \ / /    | || |  .`___'/ _/  | || |  | '    ' |  | |
| | _/ /    \ \_ | || |  _| |___.' / | || |    \ ' /     | || | | (___)  \_  | || |   \ `--' /   | |
| ||____|  |____|| || | |________.'  | || |     \_/      | || | `._____.\__| | || |    `.__.'    | |
| |              | || |              | || |              | || |              | || |              | |
| '--------------' || '--------------' || '--------------' || '--------------' || '--------------' |
 '----------------'  '----------------'  '----------------'  '----------------'  '----------------'
");
    // first prompt
}

fn display_node(node: &story::StoryNode){
    println!("{}", node.description);
    println!("Please Chose An Option:");

    if node.prev_node.is_some(){
        println!("0) {}", node.prev_node.as_ref().unwrap().description);
    }
    let mut n = 1;
    for option in &node.options{
        println!("{}) {}", n, option.description);
        n += 1;
    }
}
fn read_selected_choice(node: &story::StoryNode) -> Result<&story::StoryChoice, ()>{
    let mut s = String::new();
    match std::io::stdin().read_line(&mut s){
        Ok(_) => {},
        Err(_) => return Err(())
    };
    let chosen_option: usize = match s.trim().parse(){
        Ok(n) => n,
        Err(_) => return Err(())
    };
    if chosen_option == 0 && node.prev_node.is_some(){
        return Ok(node.prev_node.as_ref().unwrap());
    }
    match node.options.get(chosen_option - 1){
        Some(c) => Ok(c),
        None => Err(()),
    }
}

pub fn present_node(node: &story::StoryNode)-> &story::StoryChoice{
    display_node(node);

    let mut selected = read_selected_choice(node);
    while selected.is_err(){
        eprintln!("Sorry, couldn't read / parse your choice. (type a number w/o punctuation in the range)");
        selected = read_selected_choice(node);
    }
    selected.unwrap()
}

pub fn pick_choice(choice: &story::StoryChoice){
    if choice.message_on_chose.is_some(){
        println!("{}", choice.message_on_chose.as_ref().unwrap());
    }
}

pub fn skip_choice(choice: &story::StoryChoice){
    if choice.message_on_skip.is_some(){
        println!("{}", choice.message_on_skip.as_ref().unwrap());
    }
}

pub fn show_choice_locked(choice: &story::StoryChoice){
    if choice.message_on_lock.is_some(){
        println!("{}", choice.message_on_lock.as_ref().unwrap());
    }
}