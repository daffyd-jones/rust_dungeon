//npc
use crate::enums::{NPCs};
use rand::{Rng};
use std::collections::HashMap;
// use serde_json::Value;
use serde::{Deserialize, Serialize};
use rand::prelude::SliceRandom;


pub fn new_comm_npc(sname: String, x: usize, y: usize, comms: Vec<String>) -> CommNPC {
    let mut rng = rand::thread_rng();
    let step = rng.gen_range(0..19);
    CommNPC {
        base: BaseNPC {
            ntype: NPCs::CommNPC,
            sname: sname,
            steps: step,
            x: x,
            y: y,
        },
        comms: comms,
    }
}

pub fn new_conv_npc(sname: String, x: usize, y: usize, conv: Convo) -> ConvNPC {
    let mut rng = rand::thread_rng();
    let step = rng.gen_range(0..19);
    ConvNPC {
        base: BaseNPC {
            ntype: NPCs::ConvNPC,
            sname: sname,
            steps: step,
            x: x,
            y: y,
        },
        conv: conv,
    }
}

pub fn new_quest_npc(sname: String, x: usize, y: usize, quest: NQuest) -> QuestNPC {
    let mut rng = rand::thread_rng();
    let step = rng.gen_range(0..19);
    QuestNPC {
        base: BaseNPC {
            ntype: NPCs::QuestNPC,
            sname: sname,
            steps: step,
            x: x,
            y: y,
        },
        quest: quest,
    }
}

//--
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Convo {
    id: String,
    #[serde(flatten)]
    stages: HashMap<String, Stage>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Stage {
    text: String,
    opts: Vec<ConOpt>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConOpt {
    text: String,
    next: String,
}
//--

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NQuest {
    text: String,
}



//--
pub trait NPC {
    // fn as_comm_npc(&self) -> Option<&CommNPC>;
    fn as_any(&self) -> &dyn std::any::Any;
    fn get_ntype(&mut self) -> NPCs;
    fn get_sname(&mut self) -> String;
    fn get_pos(&mut self) -> (usize, usize);
    fn set_steps(&mut self, steps: u8);
    fn inc_steps(&mut self);
    fn get_steps(&mut self) -> u8;
    fn mmove(&mut self, dir: &str);
}

impl dyn NPC {
    pub fn as_comm_npc(&self) -> Option<&CommNPC> {
        self.as_any().downcast_ref::<CommNPC>()
    }

    pub fn as_conv_npc(&self) -> Option<&ConvNPC> {
        self.as_any().downcast_ref::<ConvNPC>()
    }

    pub fn as_quest_npc(&self) -> Option<&QuestNPC> {
        self.as_any().downcast_ref::<QuestNPC>()
    }

    // fn as_any(&self) -> &dyn std::any::Any;
}


//--
#[derive(Clone, Debug, PartialEq)]
pub struct BaseNPC {
    ntype: NPCs,
    sname: String,
    steps: u8,
    x: usize,
    y: usize,
}

impl NPC for BaseNPC {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_ntype(&mut self) -> NPCs {
        self.ntype.clone()
    }

    fn get_sname(&mut self) -> String {
        self.sname.clone()
    }

    fn get_pos(&mut self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn set_steps(&mut self, steps: u8) {
        self.steps = steps;
    }

    fn inc_steps(&mut self) {
        self.steps += 1;
    }

    fn get_steps(&mut self) -> u8 {
        self.steps.clone()
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.y -= 1,
            "DN" => self.y += 1,
            "LF" => self.x -= 1,
            "RT" => self.x += 1,
            _ => println!("")
        }
    }
}

impl BaseNPC {
    pub fn new() -> Self {
        Self {ntype: NPCs::Null, sname: "".to_string(), steps: 0, x: 0, y: 0}
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommNPC {
    base: BaseNPC,
    comms: Vec<String>,
}

impl NPC for CommNPC {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_ntype(&mut self) -> NPCs {
        self.base.ntype.clone()
    }

    fn get_sname(&mut self) -> String {
        self.base.sname.clone()
    }

    fn get_pos(&mut self) -> (usize, usize) {
        (self.base.x, self.base.y)
    }

    fn set_steps(&mut self, steps: u8) {
        self.base.steps = steps;
    }

    fn get_steps(&mut self) -> u8 {
        self.base.steps.clone()
    }

    fn inc_steps(&mut self) {
        self.base.steps += 1;
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.base.y -= 1,
            "DN" => self.base.y += 1,
            "LF" => self.base.x -= 1,
            "RT" => self.base.x += 1,
            _ => println!("")
        }
    }
}

impl CommNPC {
    pub fn get_comm(&mut self) -> String {
        let mut rng = rand::thread_rng();
        if let Some(comm) = self.comms.choose(&mut rng) {
            comm.to_string()
        } else {"".to_string()}
    }
}


#[derive(Clone, Debug, PartialEq)]
pub struct ConvNPC {
    base: BaseNPC,
    conv: Convo,
}

impl NPC for ConvNPC {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_ntype(&mut self) -> NPCs {
        self.base.ntype.clone()
    }

    fn get_sname(&mut self) -> String {
        self.base.sname.clone()
    }

    fn get_pos(&mut self) -> (usize, usize) {
        (self.base.x, self.base.y)
    }

    fn set_steps(&mut self, steps: u8) {
        self.base.steps = steps;
    }

    fn get_steps(&mut self) -> u8 {
        self.base.steps.clone()
    }

    fn inc_steps(&mut self) {
        self.base.steps += 1;
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.base.y -= 1,
            "DN" => self.base.y += 1,
            "LF" => self.base.x -= 1,
            "RT" => self.base.x += 1,
            _ => println!("")
        }
    }
}

impl ConvNPC {
    pub fn get_conv(&mut self) -> Convo {
        self.conv.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuestNPC {
    base: BaseNPC,
    quest: NQuest,
}

impl NPC for QuestNPC {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn get_ntype(&mut self) -> NPCs {
        self.base.ntype.clone()
    }

    fn get_sname(&mut self) -> String {
        self.base.sname.clone()
    }

    fn get_pos(&mut self) -> (usize, usize) {
        (self.base.x, self.base.y)
    }

    fn set_steps(&mut self, steps: u8) {
        self.base.steps = steps;
    }

    fn get_steps(&mut self) -> u8 {
        self.base.steps.clone()
    }

    fn inc_steps(&mut self) {
        self.base.steps += 1;
    }

    fn mmove(&mut self, dir: &str) {
        match dir {
            "UP" => self.base.y -= 1,
            "DN" => self.base.y += 1,
            "LF" => self.base.x -= 1,
            "RT" => self.base.x += 1,
            _ => println!("")
        }
    }
}

impl QuestNPC {
    pub fn get_quest(&mut self) -> NQuest {
        self.quest.clone()
    }
}

