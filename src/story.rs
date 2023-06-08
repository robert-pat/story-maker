use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
pub struct NodeID {
    id: u64,
}
impl NodeID{
    //noinspection DuplicatedCode
    pub fn from_text(text: &str) -> Self{
        let mut h = DefaultHasher::new();
        h.write(text.as_bytes());
        Self{
            id: h.finish()
        }
    }
}
impl Default for NodeID{
    fn default() -> Self {
        NodeID::from_text("RootNode")
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct StoryEvent { //TODO: better name for these, eg: pickup an item, press a button, etc.
    id: u64,
}
impl StoryEvent{
    //noinspection DuplicatedCode
    pub fn from_text(text: &str) -> Self{
        let mut h = DefaultHasher::new();
        h.write(text.as_bytes());
        Self{
            id: h.finish()
        }
    }
}
impl Default for StoryEvent{
    fn default() -> Self {
        Self{ id: 0}
    }
}

pub struct StoryChoice{
    pub(crate) description: String,
    pub(crate) destination_node: NodeID,
    pub(crate) message_on_chose: Option<String>, // Display when this option is chosen
    pub(crate) message_on_lock: Option<String>,  // Display when trying this option & this is locked
    requirements: Vec<StoryEvent>,
    when_chosen: Vec<(StoryEvent, bool)>, // events to update when this option is chosen
    when_skipped: Vec<(StoryEvent, bool)>, // events to update when this option is skipped
}
impl StoryChoice{
    pub fn new()-> Self{
        Self{
            description: "empty choice".to_string(),
            destination_node: NodeID::default(),
            message_on_chose: None,
            message_on_lock: None,
            requirements: vec![],
            when_chosen: vec![],
            when_skipped: vec![],
        }
    }
    pub fn blank_from_id(id: NodeID) -> Self{
        Self{
            description: "".to_string(),
            destination_node: id,
            message_on_chose: None,
            message_on_lock: None,
            requirements: vec![],
            when_chosen: vec![],
            when_skipped: vec![],
        }
    }
}

pub struct StoryNode{
    pub(crate) id: NodeID,
    pub(crate) description: String,
    pub(crate) options: Vec<StoryChoice>,
    pub(crate) prev_node: Option<NodeID>
}
impl StoryNode{
    pub fn new() -> Self{
        Self{
            id: NodeID::default(),
            description: "root".to_string(),
            options: vec![],
            prev_node: None,
        }
    }

    pub fn from_file() -> Self{
        todo!()
    }
}

pub struct StoryContainer{
    nodes: HashMap<NodeID, StoryNode>,
}
impl StoryContainer{
    pub fn new() -> Self{
        let mut map = HashMap::new();
        map.insert(NodeID::default(), StoryNode::new());
        Self{
            nodes: map
        }
    }

    pub fn from_file() -> Self{
        todo!()
    }

    pub fn get_node(&self, id: NodeID) -> &StoryNode{
        match self.nodes.get(&id){
            Some(n) => n,
            None => panic!("Requested Node does not exist in map")
        }
    }
}

pub struct StoryTraverser {
    pub(crate) current_node: NodeID,
    pub(crate) prev_node: Option<NodeID>,
    pub(crate) events: HashMap<StoryEvent, bool> //TODO: rename here too (if changing StoryEvent)
}
impl StoryTraverser {
    pub fn new() -> Self{
        Self{
            current_node: NodeID::default(),
            prev_node: None,
            events: HashMap::new()
        }
    }
    pub fn from_file() -> Self{
        todo!() // read the story from JSON
    }
    pub fn is_choice_unlocked(&self, choice: &StoryChoice) -> bool{
        for r in &choice.requirements{
            if !*self.events.get(r).unwrap_or(&false){
                return false;
            }
        }
        true
    }
    pub fn pick_choice(&mut self, choice: &StoryChoice){
        for change in &choice.when_chosen{
            *self.events.get_mut(&change.0).unwrap() = change.1;
        }
    }
    pub fn skip_choice(&mut self, choice: &StoryChoice){
        self.current_node = choice.destination_node;
        for change in &choice.when_skipped{
            *self.events.get_mut(&change.0).unwrap() = change.1;
        }
    }
}