use crate::enums::{Cells, EnvInter, FeatureType, NPCWrap};
use crate::item::Item;
use crate::npc::{new_comm_npc, new_conv_npc, new_shop_npc, Convo, ShopConvos, ShopData};
use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::HashMap;
use std::fs;

const GRASS_PATCH: &str = r#"
',',',',',',',',
',",',',',',",',
',',',',',',',',
',',',",',',',',
',',',',',',',',
',",',',',',',',
',',',',',",',',
',',',',',',',',
"#;

const GRASS_PATCH_IN_CORNER: &str = r#"
',',',',',',',',
',",',',',',",',
',',',',',',',',
',',',",' ',' ',
',','           
',",',          
',','           
',',',          
"#;

const GRASS_PATCH_OUT_CORNER: &str = r#"
                
                
                
       , ' , ,  
      ',',",','"
     ,',',',',',
      ',',",',',
      ',',',',',
"#;

const GRASS_PATCH_HORZ_EDGE: &str = r#"
                
                
                
                
',',', ,',',',' 
',",',',',',',',
',',',',',",',',
',',',',',',',',
"#;

const GRASS_PATCH_VERT_EDGE: &str = r#"
      ',',',',',
       ,',',",',
      ',',',',',
     ,",',',',',
     ,',',',',',
      ',',',',',
      ',',",',',
     ,',',',',',
"#;

const SHRUB_PATCH: &str = r#"
',',',',',',',',
',",'&&&',',",',
','&&&&&&&',',',
',&&&&&&&&',',',
','&&&&&&&&,',',
',",'&&&&,',',',
',',',',',",',',
',',',',',',',',
"#;

// Abandoned shacks

const ABANDONED_SHACK_1: &str = r#"Null|Null|BronzeWarAxe
________________________________________
________________________________________
____▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒___',___,',',______
____▒_π____│________▒___',",',',',______
____▒_____O│______.¨≡°___,',',",',._____
____▒───_──┘________▒___',',',',',."____
____▒_______________▒___',",',',",______
____▒▒▒▒▒▒▒▒▒▒__▒▒▒▒▒____,',",',','_____
_____________"__",_____,',',',',',,_____
______.ѧ.____________,',',",',',",'_____
______"*'_____________',",',',',',______
________________________________________
"#;

const ABANDONED_SHACK_2: &str = r#"Null|Null|BronzeGreatsword
________________________________________
__,______.,"__,___________________,__.__
_"┌┬┬┬┬┬┬┬┬┬┐_____'__▒▒▒▒▒▒▒▒▒▒▒▒▒▒,____
__├",*______┤__,_____▒_________≡≡_▒_____
__├',"______┤_____,"_▒____________▒_____
__├'________┤________▒___┌─_______▒___"_
__├_________┤___'____▒___│______≡_▒_____
__├┬┬┬┬┐_┌┬┬┤_"__________│_____≡≡O▒__,__
__._____________'____▒___│_____≡≡≡▒_____
________,__.__*______▒___│____≡≡≡_▒,____
____'____._______.___▒▒▒▒▒▒▒▒▒▒▒▒▒▒",___
_"__________.______*_______"",'___""____
"#;

const ABANDONED_SHACK_3: &str = r#"Null|Null|BronzeGreatsword
________________________________________
_,______Ʌ___Ʌ_______'___,__:┌─────┐:_,__
_____ж_Ʌ_________,_________:│~~~~~│:____
__"__жж__ѧ___Ʌ________"____:│~~~~~│:__'_
_,__________Ʌ___________.__:│~~~~~│:____
_"▒▒▒▒▒▒▒▒▒,____,",',',____:└─────┘:_,__
__▒O__│___▒"____,',',",',_____±_±_______
__▒___│___▒_____,',',',',',',_,_,_,',___
_,▒─__┘_________,",',',",",',',",",',___
__▒≡_____≡▒__'__,',",',',',",',',',"____
_,▒▒▒▒▒▒▒▒▒,_____',',',',',',',',','__'_
_____'____"_____________________________
"#;

const ABANDONED_SHACK_4: &str = r#"Null|Null|BronzeGreatsword
______________,___________._____________
_ʬʬʬʬʬʬʬʬʬʬʬ___________,________________
_ʬʬ,ʬʬʬʬʬʬʬʬ____._____"▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒"_
_ʬʬʬʬʬʬʬʬʬʬʬ___________▒_____│_______▒__
_ʬʬʬʬʬʬʬ"ʬʬʬ_┤_____,___▒__________π__▒,_
_ʬʬʬʬʬʬʬʬʬʬʬ_┤_________▒O____│_______▒__
_ʬʬʬʬʬʬʬʬʬʬʬ_┤_________▒─────┼────_──▒__
_ʬʬʬ.ʬʬʬʬʬʬʬ_┤_______________________▒,_
_ʬʬʬʬʬʬʬʬʬʬʬ_┤__"______▒_____│_____≡≡▒__
_ʬʬʬʬʬʬʬʬʬʬʬ_┤________,▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒__
_______┌┬┬┬┬┬┤____.____'___________'"___
_________,________________._____________
"#;

const ABANDONED_SHACK_5: &str = r#"Null|Null|BronzeGreatsword
________________________________________
_,.~~~~,__________,_____________________
_"~~~~~~~,__________ʭΠỻΠỻΠΠỻΠỻΠʭ____.___
_~~~~~~~~~,_________ʭ________O_ʭ________
_~~~~~~~~~._________ʭ__________ʭ________
_~~~~~~~~~~_________ʭ__________ʭ________
_"~~~~~~~~~'________ʭ__________ʭ________
__,~~~~~~~,_________ỻΠỻΠĦ__ĦΠỻΠỻ________
___.~~~~~,.______________________,______
_______'"______"________________________
________________________________________
________________________________________
"#;

const ABANDONED_SHACKS: [&str; 5] = [
    ABANDONED_SHACK_1,
    ABANDONED_SHACK_2,
    ABANDONED_SHACK_3,
    ABANDONED_SHACK_4,
    ABANDONED_SHACK_5,
];

const SMALL_RUIN_1: &str = r#"Null|Null|Null
________________________________________________________________________________
___________________________________,____________________________________________
_,_ʭʭʭʭʭʭʭʭʭʭʭʭʭʭʭʭ______.________________________'_____________________________
___ʭ≡≡___±©©±_____ʭ______________________________________,___________________"__
__.ʭ_____________"ʭ_________╔════════╗__╔════════╗__╔════════╗__╔════════╗______
___ʭ______::______ʭ_________║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║______
___ʭ______::______ʭ_________║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║______
___ʭ"_____::______ʭ_________║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║_,____
___ʭ_____::::_____ʭ,________╚════════╝__╚════════╝__╚════════╝__╚════════╝______
_______:::__:::________________________________"________________________________
_______:::_":::________"___________,_________________________________,__________
_"_____::::::::_____________╔════════╗__╔════════╗__╔════════╗__╔════════╗______
____________________________║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║______
___.:____Ħ::Ħ____:__________║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║__║~~~~~~~~║___.__
____:,____::_____:,_______._║~~~~~~~~║__║~~~~~~~~║_.║~~~~~~~~║__║~~~~~~~~║______
____::::::::::::::__________╚════════╝__╚════════╝__╚════════╝__╚════════╝______
____:_____::"____:________________________""___________________________"________
_________::::_________,____________________________,____________________________
__'_:ỻ__Π____Π__ỻ:_____________"_____________________________,__________________
________________________________________________________________________________
"#;

const palette: &str = r#"
empty: ' . , ' * |
wall: ▒ |
other ▓ ░ ~ |
pipes:
═ ║ ╣ ╠ ╩ ╦ ╗ ╝ ╚ ╔ ╬
┐ └ ┴ ┬ ├ ─ ┼ ┘ ┌ ┤ │

ʬ ỻ Π Ħ ʭ                     
ж ѧ π
 
≡ ° × ¤ ¸ ¨ · ■ ¦ ± ¡ ø Ø ©

i ̾¡  ͔¡  ͊¡  ͛¡  ̷¡  ̸¡  ̚¡  ͆¡ ¡˞ ¡ˡ  ̢¡ ¡     

"#;

const SMALL_RUIN_2: &str = r#"
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
________________________________________________________________________________
"#;

const SMALL_RUINS: [&str; 1] = [SMALL_RUIN_1];

fn parse_map(
    s_map: &str,
    mut cells: Vec<Vec<Cells>>,
) -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    // let mut cells: Vec<Vec<Cells>> = Vec::new();
    let mut rng = rand::thread_rng();
    let map_codet = s_map.lines().next().unwrap_or("");
    let map_code: Vec<&str> = map_codet.split("|").collect();
    let npc_types: Vec<&str> = map_code.clone()[0].split(" ").collect();
    let item_types: Vec<&str> = map_code.clone()[2].split(" ").collect();

    let data1 = fs::read_to_string("src/npcs/npc_names.json");
    //log::info!("{:?}", &data1);
    let names: Vec<String> = match data1 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let data2 = fs::read_to_string("src/npcs/npc_comms.json");
    //log::info!("{:?}", &data2);
    let comms: Vec<String> = match data2 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let data3 = fs::read_to_string("src/npcs/npc_convos.json");
    //log::info!("{:?}", &data3);
    let convos: Vec<Convo> = match data3 {
        Ok(content) => serde_json::from_str(&content).unwrap(),
        Err(e) => {
            log::info!("{:?}", e);
            Vec::new()
        }
    };

    let mut ncount = 0;
    let mut icount = 0;
    //let mut cells = vec![vec![Cells::Null; 150]; 50];
    let mut npcs = HashMap::new();
    let mut items = HashMap::new();
    let mut env_inters = HashMap::new();
    for (y, line) in s_map.lines().skip(1).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let cell = match ch {
                '_' => Cells::Empty,
                ',' => Cells::Grass1,
                '\'' => Cells::Grass2,
                '\"' => Cells::Grass3,
                '·' => Cells::Dirt1,
                '.' => Cells::Dirt2,
                ':' => Cells::Dirt3,
                '*' => Cells::Rock,
                '▒' => Cells::Wall,
                ' ' => Cells::Floor,
                '░' => Cells::Floor2,
                '~' => Cells::Water,
                '═' => Cells::MwH,
                '║' => Cells::MwV,
                '╣' => Cells::MwVL,
                '╠' => Cells::MwVR,
                '╩' => Cells::MwHU,
                '╦' => Cells::MwHD,
                '╝' => Cells::MwUL,
                '╚' => Cells::MwUR,
                '╗' => Cells::MwDL,
                '╔' => Cells::MwDR,
                '╬' => Cells::MwCR,
                '─' => Cells::SwH,
                '│' => Cells::SwV,
                '┤' => Cells::SwVL,
                '├' => Cells::SwVR,
                '┴' => Cells::SwHU,
                '┬' => Cells::SwHD,
                '┘' => Cells::SwUL,
                '└' => Cells::SwUR,
                '┐' => Cells::SwDL,
                '┌' => Cells::SwDR,
                '┼' => Cells::SwCR,
                '≡' => Cells::Cong,
                '°' => Cells::Deg,
                '×' => Cells::Mult,
                '¸' => Cells::Ced,
                '¨' => Cells::Diae,
                '■' => Cells::Blsq,
                '¦' => Cells::VBrk,
                '±' => Cells::PlMin,
                'ø' => Cells::SmZer,
                'Ø' => Cells::BZer,
                '©' => Cells::Cop,
                'Ħ' => Cells::DblBracedGate, //-------
                'ỻ' => Cells::BracedGate,
                'Π' => Cells::Arch,
                'ʭ' => Cells::Bricks,
                'ʬ' => Cells::Crops,
                'ѧ' => Cells::SmallCampfire,
                'Ѧ' => Cells::Campfire,
                'π' => Cells::Table,
                'ж' => Cells::Firewood,
                'Ʌ' => Cells::Tent,
                // '@' => Cells::NPCM,
                '{' => Cells::LBrce,
                '}' => Cells::RBrce,
                '(' => Cells::LParen,
                ')' => Cells::RParen,
                '¤' => Cells::GenCur,
                'o' => Cells::Item,
                'l' => Cells::Log,
                'c' => Cells::Clinic,
                'p' => Cells::GPost,
                's' => Cells::CPost,
                _ => Cells::Empty,
            };
            cells[y][x] = cell;
            if ch == '@' {
                let def_name = "Kevthony".to_string();
                match npc_types[ncount] {
                    "CommNPC" => {
                        //let com_def = vec!["Welcome to the caves!!".to_string(), "Theres a tonne of folk down here, lerger cities as you go into the cave.".to_string()];
                        let rnd_comms = {
                            let mut tvec = Vec::new();
                            for _ in 0..4 {
                                let tidx = rng.gen_range(0..comms.len());
                                tvec.push(comms[tidx].clone());
                            }
                            tvec
                        };
                        let name = names.choose(&mut rng).unwrap_or(&def_name.clone()).clone();
                        let t_comm = new_comm_npc(name.clone(), x, y, rnd_comms.clone());
                        npcs.insert((x, y), NPCWrap::CommNPC(t_comm.clone()));
                    }
                    "ConvNPC" => {
                        let name = names.choose(&mut rng).unwrap_or(&def_name.clone()).clone();
                        //let comms = vec!["Welcome to the caves!!".to_string(), "Theres a tonne of folk down here, lerger cities as you go into the cave.".to_string()];
                        let conv: Convo = convos
                            .choose(&mut rng)
                            .unwrap_or(&convos[0].clone())
                            .clone();
                        let t_comm = new_conv_npc(name.clone(), x, y, conv.clone());
                        npcs.insert((x, y), NPCWrap::ConvNPC(t_comm.clone()));
                    }
                    _ => todo!(),
                }
                ncount += 1;
            }
            if ch == 'O' {
                match item_types[icount] {
                    "HealthPotion" => {
                        let ti = Item::new_health_potion(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "Salve" => {
                        let ti = Item::new_salve(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "Dowel" => {
                        let ti = Item::new_dowel(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "SmallWoodShield" => {
                        let ti = Item::new_small_wood_shield(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "Apple" => {
                        let ti = Item::new_apple(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeClaymore" => {
                        let ti = Item::new_bronze_claymore(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeShortsword" => {
                        let ti = Item::new_bronze_shortsword(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeLongsword" => {
                        let ti = Item::new_bronze_longsword(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeLightAxe" => {
                        let ti = Item::new_bronze_light_axe(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeHeavyAxe" => {
                        let ti = Item::new_bronze_heavy_axe(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzeWarAxe" => {
                        let ti = Item::new_bronze_war_axe(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "BronzePickHammer" => {
                        let ti = Item::new_bronze_pick_hammer(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "WoodStaff" => {
                        let ti = Item::new_wood_staff(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    "LightArmour" => {
                        let ti = Item::new_light_armour(x, y);
                        items.insert((x, y), ti.clone());
                    }
                    _ => {
                        log::info!("itm {:?}", item_types[icount]);
                    }
                }
                icount += 1;
            }
        }
    }
    (cells, npcs, items, env_inters)
}

fn make_small_ruin_feature() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let cells = vec![vec![Cells::Empty; 80]; 20];
    let mut rng = rand::thread_rng();
    parse_map(
        ABANDONED_SHACKS
            .choose(&mut rng)
            .unwrap_or(&ABANDONED_SHACK_1),
        cells,
    )
}

fn make_abandoned_shack() -> (
    Vec<Vec<Cells>>,
    HashMap<(usize, usize), NPCWrap>,
    HashMap<(usize, usize), Item>,
    HashMap<(usize, usize), EnvInter>,
) {
    let cells = vec![vec![Cells::Empty; 40]; 20];
    let mut rng = rand::thread_rng();
    parse_map(SMALL_RUINS.choose(&mut rng).unwrap_or(&SMALL_RUIN_1), cells)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Feature {
    ftype: FeatureType,
    pos: (i64, i64),
    map: Vec<Vec<Cells>>,
    items: HashMap<(usize, usize), Item>,
    npcs: HashMap<(usize, usize), NPCWrap>,
    env_inters: HashMap<(usize, usize), EnvInter>,
}

pub struct Features {
    features: HashMap<(i64, i64), Feature>,
}

impl Features {
    pub fn new() -> Self {
        Self {
            features: HashMap::new(),
        }
    }

    pub fn new_small_ruin_feature(&mut self, pos: (i64, i64)) {
        let mut small_cells = vec![vec![Cells::Empty; 80]; 40];
        let (map, npcs, items, env_inters) = make_abandoned_shack();
        self.features.insert(
            pos,
            Feature {
                ftype: FeatureType::AbandonedShack,
                pos,
                map,
                items,
                npcs,
                env_inters,
            },
        );
        // self.features.insert(pos, small_cells);
    }

    pub fn new_field_feature(&mut self, pos: (i64, i64)) {
        let mut small_cells = vec![vec![Cells::Empty; 80]; 40];
        // self.features.insert(pos, small_cells);
    }

    pub fn new_abandoned_shack(&mut self, pos: (i64, i64)) {
        let mut small_cells = vec![vec![Cells::Empty; 40]; 20];
        let (map, npcs, items, env_inters) = make_abandoned_shack();
        self.features.insert(
            pos,
            Feature {
                ftype: FeatureType::AbandonedShack,
                pos,
                map,
                items,
                npcs,
                env_inters,
            },
        );
    }

    pub fn feature_check(&mut self, pos: (i64, i64)) -> bool {
        let dir = (pos.0 / pos.0.abs(), pos.1 / pos.1.abs());
        let space = {
            match dir {
                (x, y) if x >= 0 && y >= 0 => (((pos.0 + 800), (pos.1 + 800)), pos),
                (x, y) if x < 0 && y >= 0 => (((pos.0 - 800), (pos.1 + 800)), pos),
                (x, y) if x >= 0 && y < 0 => (((pos.0 + 800), (pos.1 - 800)), pos),
                (x, y) if x < 0 && y < 0 => (((pos.0 - 800), (pos.1 - 800)), pos),
                _ => todo!(),
            }
        };
        for (k, _) in self.features.clone() {
            let xrange: Vec<i64> = {
                let mut xa = space.0 .0;
                let mut xb = space.1 .0;
                if xa > xb {
                    std::mem::swap(&mut xa, &mut xb);
                }
                (xa..xb).collect()
            };
            let yrange: Vec<i64> = {
                let mut ya = space.0 .0;
                let mut yb = space.1 .0;
                if ya > yb {
                    std::mem::swap(&mut ya, &mut yb);
                }
                (ya..yb).collect()
            };

            if xrange.contains(&k.0) && yrange.contains(&k.1) {
                return false;
            };
        }
        true
    }
}
