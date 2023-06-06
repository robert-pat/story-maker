use std::collections::HashMap;
use crate::story::NodeID::RootNode;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub enum NodeID {
    RootNode
}

#[derive(Hash, PartialEq, Eq)]
pub enum StoryEvent { //TODO: better name for these, eg: pickup an item, press a button, etc.

}

#[derive(PartialEq)] // IDK if this works, checking for e
pub struct StoryChoice{
    pub(crate) description: String,
    pub(crate) destination_node: NodeID,
    pub(crate) message_on_chose: Option<String>, // Display when this option is chosen
    pub(crate) message_on_lock: Option<String>,  // Display when trying this option & this is locked
    requirements: Vec<StoryEvent>,
    when_chosen: Vec<(StoryEvent, bool)>, // events to update when this option is chosen
    when_skipped: Vec<(StoryEvent, bool)>, // events to update when this option is skipped
}

pub struct StoryNode{
    pub(crate) id: NodeID,
    pub(crate) description: String,
    pub(crate) options: Vec<StoryChoice>,
    pub(crate) prev_node: Option<NodeID>
}

pub struct Story {
    pub(crate) current_node: NodeID,
    pub(crate) prev_node: Option<NodeID>,
    pub(crate) nodes: HashMap<NodeID, StoryNode>,
    pub(crate) events: HashMap<StoryEvent, bool> //TODO: rename here too (if changing StoryEvent)
}

impl Story{
    pub fn new() -> Self{
        let mut n: HashMap<NodeID, StoryNode> = HashMap::new();
        n.insert(RootNode, StoryNode{
            id: RootNode,
            description: String::new(),
            options: Vec::new(),
            prev_node: None
        });

        Self{
            current_node: RootNode,
            prev_node: None,
            nodes: n,
            events: HashMap::new()
        }
    }

    pub fn from_file() -> Self{
        todo!() // read the story from JSON
    }

    pub fn get_node(&self, node: NodeID) -> &StoryNode{
        match self.nodes.get(&node) {
            Some(n) => n,
            None => panic!("NodeID not in collection: {:?}", node)
        }
    }
    pub fn get_current_node(&self) -> &StoryNode{
        match self.nodes.get(&self.current_node){
            Some(n) => n,
            None => panic!("Could not retrieve current node: {:?}", self.current_node)
        }
    }
}

pub fn is_choice_unlocked(story: &Story, choice: &StoryChoice) -> bool{
    for r in &choice.requirements{
        if !*story.events.get(r).unwrap_or(&false){
            return false;
        }
    }
    true
}

pub fn pick_choice(story: &mut Story, choice: &StoryChoice){
    story.current_node = choice.destination_node;
    for change in &choice.when_chosen{
        *story.events.get_mut(&change.0).unwrap() = change.1;
    }
}
pub fn skip_choice(story: &mut Story, choice: &StoryChoice){
    story.current_node = choice.destination_node;
    for change in &choice.when_skipped{
        *story.events.get_mut(&change.0).unwrap() = change.1;
    }
}