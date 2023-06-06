use std::collections::HashMap;
use crate::story::NodeID::RootNode;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub enum NodeID {
    RootNode
}

#[derive(Hash, PartialEq, Eq)]
pub enum StoryEvent { //TODO: better name for these, eg: pickup an item, press a button, etc.

}

pub struct StoryChoice{
    unlocked_description: String,
    locked_description: Option<String>,
    requirements: Vec<StoryEvent>,
    pub destination_node: NodeID,
    message_when_chosen: Option<String>,
    when_chosen: Vec<(StoryEvent, bool)>,
    when_skipped: Vec<(StoryEvent, bool)>,
}

pub struct StoryNode{
    id: NodeID,
    description: String,
    options: Vec<StoryChoice>,
    pub(crate) prev_node: Option<NodeID>
}

pub struct Story {
    pub current_node: NodeID,
    pub prev_node: Option<NodeID>,
    nodes: HashMap<NodeID, StoryNode>,
    //events: HashMap<StoryEvent, bool> //TODO: rename here too (if changing StoryEvent)
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
        }
    }

    pub fn from_file() -> Self{
        // Should just be JSON
        todo!()
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

    pub fn is_choice_locked(events_record: &HashMap<StoryEvent, bool>, choice: &StoryChoice) -> bool{
        for r in &choice.requirements{
            if !*events_record.get(r).unwrap_or(&false){
                return false;
            }
        }
        true
    }
}
