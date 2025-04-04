//item
use crate::enums::{Equip, InterOpt, ItemEffect, ItemOpt, Items};
use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
//#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Item {
    pub itype: Items,
    pub sname: String,
    pub icon: (char, Color),
    pub desc: String,
    pub iopts: HashMap<InterOpt, String>,
    pub equip: bool,
    pub equip_type: Equip,
    pub effect: ItemEffect,
    pub x: usize,
    pub y: usize,
    pub properties: HashMap<String, u16>,
}

impl Default for Item {
    fn default() -> Self {
        let mut h = HashMap::new();
        h.insert(InterOpt::Null, "".to_string());
        let mut p = HashMap::new();
        p.insert("".to_string(), 0);
        Self {
            itype: Items::Null,
            sname: "".to_string(),
            desc: "".to_string(),
            icon: (' ', Color::White),
            iopts: h,
            equip: false,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
            x: 0,
            y: 0,
            properties: p,
        }
    }
}

impl Item {
    pub fn new(
        itype: Items,
        sname: String,
        icon: (char, Color),
        desc: String,
        iopts: HashMap<InterOpt, String>,
        equip: bool,
        equip_type: Equip,
        effect: ItemEffect,
        x: usize,
        y: usize,
        properties: HashMap<String, u16>,
    ) -> Self {
        Self {
            itype,
            sname,
            icon,
            desc,
            iopts,
            equip,
            equip_type,
            effect,
            x,
            y,
            properties,
        }
    }

    pub fn new_edible_root(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 3);
        prop.insert(String::from("value"), 2);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::EdibleRoot,
            sname: "Edible Root".to_string(),
            icon: ('ȝ', Color::Yellow),
            desc: "Weird looking root, doesnt look very tasty.".to_string(),
            iopts,
            equip: false,
            equip_type: Equip::Null,
            effect: ItemEffect::Health,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_rock(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 0);
        prop.insert(String::from("value"), 0);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        Self {
            itype: Items::Rock,
            sname: "Rock".to_string(),
            icon: ('o', Color::Yellow),
            desc: "Its a rock.".to_string(),
            iopts,
            equip: false,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_guts(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 1);
        prop.insert(String::from("value"), 0);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::Guts,
            sname: "Guts".to_string(),
            icon: ('ʚ', Color::Yellow),
            desc: "Parts of a dead creature.".to_string(),
            iopts,
            equip: false,
            equip_type: Equip::Null,
            effect: ItemEffect::Health,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_metal_scrap(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 0);
        prop.insert(String::from("value"), 1);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));

        Self {
            itype: Items::MetalScrap,
            sname: "Metal Scrap".to_string(),
            icon: ('ϟ', Color::Yellow),
            desc: "Scrap of metal.".to_string(),
            iopts,
            equip: false,
            equip_type: Equip::Null,
            effect: ItemEffect::Null,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_apple(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 5);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 5);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));

        Self {
            itype: Items::Apple,
            sname: "Apple".to_string(),
            icon: ('ỏ', Color::Yellow),
            desc: "A slightly bruised apple that as been here for a while.".to_string(),
            iopts,
            equip: false,
            equip_type: Equip::Null,
            effect: ItemEffect::Health,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_health_potion(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 30);
        //prop.insert(String::from("effect"), 30);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));
        Self {
            itype: Items::HealthPotion,
            sname: "Health Potion".to_string(),
            icon: ('ṓ', Color::Green),
            desc: "Mixture of curdled liquids. Returns vitality to the body.".to_string(),
            iopts,
            equip: false,
            equip_type: Equip::Null,
            effect: ItemEffect::Health,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_salve(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("health"), 15);
        //prop.insert(String::from("effect"), 15);
        prop.insert(String::from("value"), 30);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Use), String::from("Use"));

        Self {
            itype: Items::Salve,
            sname: "Salve".to_string(),
            icon: ('ṓ', Color::Blue),
            desc: "Thick paste for smearing on wounds. It heals better than it smells.".to_string(),
            iopts,
            equip: false,
            equip_type: Equip::Null,
            effect: ItemEffect::Health,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_dowel(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 5);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 10);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::Dowel,
            sname: "Dowel".to_string(),
            icon: ('˩', Color::Red),
            desc: "Most of a broomstick. Its sharp at one end.".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_claymore(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 15);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 60);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzeClaymore,
            sname: "Bronze Claymore".to_string(),
            icon: ('Ṫ', Color::Red),
            desc: "A bronze double edged sword".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_iron_claymore(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 20);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 85);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::IronClaymore,
            sname: "Iron Claymore".to_string(),
            icon: ('Ṫ', Color::Gray),
            desc: "An iron double edged sword".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_steel_claymore(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 25);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 110);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::SteelClaymore,
            sname: "Steel Claymore".to_string(),
            icon: ('Ṫ', Color::White),
            desc: "A steel double edged sword".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_longsword(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 12);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzeLongsword,
            sname: "Bronze Longsword".to_string(),
            icon: ('†', Color::Yellow),
            desc: "A bronze longsword".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_iron_longsword(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 17);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 75);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::IronLongsword,
            sname: "Iron Longsword".to_string(),
            icon: ('†', Color::Gray),
            desc: "An iron longsword".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_greatsword(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 17);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzeGreatsword,
            sname: "Bronze Greatsword".to_string(),
            icon: ('ϯ', Color::Yellow),
            desc: "A bronze greatword".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_shortsword(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 7);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 40);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzeShortsword,
            sname: "Bronze Shortsword".to_string(),
            icon: ('Ϯ', Color::Yellow),
            desc: "A bronze shortsword".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_basic_staff(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 7);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 40);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BasicStaff,
            sname: "Basic Staff".to_string(),
            icon: ('ɭ', Color::Red),
            desc: "A basic wooden staff".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_wood_staff(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 15);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::WoodStaff,
            sname: "Wood Staff".to_string(),
            icon: ('ſ', Color::Red),
            desc: "A solid wood staff sith a knot at the top.".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bludgeon_staff(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 20);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 80);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BludgeonStaff,
            sname: "Bludgeon Staff".to_string(),
            icon: ('ƪ', Color::Red),
            desc: "A staff that has a knob at the end for hitting.".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_gem_staff(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 25);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 120);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::GemStaff,
            sname: "Gem Staff".to_string(),
            icon: ('ẛ', Color::Red),
            desc: "A staff with a gem at the top.".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_heavy_axe(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 15);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 60);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzeHeavyAxe,
            sname: "Bronze Heavy Axe".to_string(),
            icon: ('ͳ', Color::Yellow),
            desc: "A bronze heavy axe".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_light_axe(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 15);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 50);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzeLightAxe,
            sname: "Bronze Light Axe".to_string(),
            icon: ('Ͳ', Color::Yellow),
            desc: "A bronze light axe".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_pick_axe(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 15);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 55);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzePickAxe,
            sname: "Bronze PickAxe".to_string(),
            icon: ('ፐ', Color::Yellow),
            desc: "A bronze pick axe".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_pick_hammer(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 17);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 55);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzePickHammer,
            sname: "Bronze Pick Hammer".to_string(),
            icon: ('Ƭ', Color::Yellow),
            desc: "A bronze pick hammer".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_shadow_axe(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 30);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 200);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::ShadowAxe,
            sname: "Shadow Axe".to_string(),
            icon: ('ፕ', Color::Yellow),
            desc: "Shadow Axe".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    pub fn new_bronze_war_axe(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("damage"), 50);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 300);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::BronzeWarAxe,
            sname: "Bronze War Axe".to_string(),
            icon: ('ቸ', Color::Yellow),
            desc: "A bronze war axe".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Weapon,
            effect: ItemEffect::Damage,
            x,
            y,
            properties: prop,
        }
    }

    //BronzeLongsword BronzeLightAxe Salve Salve Dowel WoodenBoard BronzePickHammer BronzeShortsword Apple|Apple

    pub fn new_wooden_board(x: usize, y: usize) -> Self {
        let mut prop = HashMap::new();
        prop.insert(String::from("defence"), 5);
        //prop.insert(String::from("effect"), 5);
        prop.insert(String::from("value"), 30);
        let mut iopts = HashMap::new();
        iopts.insert(InterOpt::Item(ItemOpt::PickUp), String::from("Pick Up"));
        iopts.insert(InterOpt::Item(ItemOpt::Drp), String::from("Drop"));
        iopts.insert(InterOpt::Item(ItemOpt::Equip), String::from("Equip"));

        Self {
            itype: Items::WoodenBoard,
            sname: "Wooden Board".to_string(),
            icon: ('ѳ', Color::Red),
            desc: "A wooden board with a strap attached to it.".to_string(),
            iopts,
            equip: true,
            equip_type: Equip::Shield,
            effect: ItemEffect::Defence,
            x,
            y,
            properties: prop,
        }
    }

    pub fn get_itype(&mut self) -> Items {
        self.itype.clone()
    }

    pub fn get_sname(&mut self) -> String {
        self.sname.clone()
    }

    pub fn get_pos(&mut self) -> (usize, usize) {
        (self.x.clone(), self.y.clone())
    }

    pub fn set_pos(&mut self, pos: (usize, usize)) {
        self.x = pos.0;
        self.y = pos.1;
    }

    pub fn get_properties(&mut self) -> HashMap<String, u16> {
        self.properties.clone()
    }

    pub fn is_equip(&mut self) -> bool {
        self.equip.clone()
    }

    pub fn get_equip_type(&mut self) -> Equip {
        self.equip_type.clone()
    }

    pub fn get_effect(&mut self) -> ItemEffect {
        self.effect.clone()
    }

    pub fn get_desc(&mut self) -> String {
        self.desc.clone()
    }

    pub fn get_iopts(&mut self) -> HashMap<InterOpt, String> {
        self.iopts.clone()
    }
}
