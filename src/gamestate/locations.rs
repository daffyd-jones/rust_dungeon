use crate::enums::{NPCWrap, GUIMode, GameMode, Interactable, Location, CompMode};
use crate::map::{MAP_W, MAP_H};
//use crate::player::Player;
use crate::puzzle::Puzzle;
//use crate::puzzles::Puzzles;
//use crate::enemy::{Enemy};
use crate::npc::{NPC};
//use crate::lsystems::LSystems;
//use crate::gui::GUI;
//use crate::settlements::Settlements;
use crate::settlement::Settlement;
//use crate::shop::Shop;
use crate::gamestate::GameState;

// use crate::gui_man_draw::GUI;
use crate::item::Item;
use std::time::{Instant};

use std::collections::HashMap;
//use rand::Rng;
//use rand::prelude::SliceRandom;
use ratatui::crossterm::event::{read, Event, KeyCode, poll};
use crate::gamestate::loc_shop_items;
use crate::gamestate::in_range;
use crate::gamestate::box_npc;
use crate::gamestate::wrap_nbox;

impl GameState {
    pub fn map_location(&mut self) {
        if self.location != Location::Null {
            let (lpos, lmap) = match self.location.clone() {
                Location::Settlement(mut settle) => {
                    let p = settle.get_pos();
                    let m = settle.get_map();
                    (p, m)
                },
                Location::Puzzle(mut puzzle) => {
                    let p = puzzle.get_pos();
                    let m = puzzle.get_map();
                    (p, m)
                },
                _ => todo!(),
            };
            let mut map_vec = self.map.cells.clone();
            let pos = self.dist_fo;
            for (i, row) in lmap.iter().enumerate() {
                for (j, &cell) in row.iter().enumerate() {
                    let main_i = (pos.1 + i as i64 + lpos.1) as usize;
                    let main_j = (pos.0 + j as i64 + lpos.0) as usize;
                    if main_i < map_vec.len() && main_j < map_vec[0].len() {
                        map_vec[main_i][main_j] = cell;
                    }
                }
            }
            //log::info!("map_copied");
            self.map.cells = map_vec.clone()
        }
    }

    pub fn compass_check(&mut self) {
        let spos_list = self.settles.get_compass_pos();
        if spos_list.len() > self.comp_list.len() {
            self.comp_list = spos_list.clone();
        }
        if self.comp_mode == CompMode::Location {
            return ();
        }
        let dfo = self.dist_fo.clone();
        let mut distances = HashMap::new();
        let mut d_min = 0;
        for ((x, y), _) in spos_list {
            let (dx, dy) = (x - dfo.0*-1, y - dfo.1*-1);
            let hyp = ((dx.pow(2) + dy.pow(2)) as f64).sqrt() as i64;
            if d_min == 0 {
                d_min = hyp;
                distances.insert(hyp, (x.clone(), y.clone()));
            } else if hyp < d_min {
                d_min = hyp;
                distances.insert(hyp, (x.clone(), y.clone()));
            }
        }
        self.comp_head = distances[&d_min].clone();
        self.gui.set_comp_list(self.comp_list.clone());
    }

    pub fn new_loc_check(&mut self) {
        let mut cpos = self.dist_fo;
        let chyp = ((cpos.0.pow(2) + cpos.1.pow(2)) as f64).sqrt() as i64;
        if chyp + 200 > 1000 {
            let ks = chyp / 1000;
            //let cdir = get_dir(cpos.clone());
            if ks >= self.depth.into() {
                self.settles.spawn_new_settlement(cpos.clone());
                self.depth *= 2;
            }
        }
    }

    pub fn update_puzzle(&mut self, puzzle: Puzzle) -> Location {
        Location::Puzzle(puzzle)
    }

    pub fn update_location(&mut self) {
        let location = self.location.clone();
        self.location = match location {
            Location::Settlement(settle) => self.update_settlement(settle),
            Location::Puzzle(puzzle) => self.update_puzzle(puzzle),
            _ => todo!(),
        };
    }

    pub fn location_check(&mut self) {
        if self.location == Location::Null {
            //log::info!("looking for settlement");
            if let Some(settlement) = self.settles.check_location(self.dist_fo.clone(), self.loc_rad.clone()) {
                self.location = Location::Settlement(settlement);
                //log::info!("settlement located");
            };
            if let Some(puzzle) = self.puzzles.check_location(self.dist_fo.clone(), self.loc_rad.clone()) {
                self.location = Location::Puzzle(puzzle);
            };
        } else {
            //log::info!("checking if away from settle");
            match &mut self.location {
                Location::Settlement(ref mut settle) => {
                    let lpos = settle.get_pos();
                    if !in_range(lpos, (self.dist_fo.0*-1, self.dist_fo.1*-1), self.loc_rad) {
                        settle.tog_npcs_sent();
                        self.settles.update_settlement(settle.clone());
                        self.location = Location::Null;
                        //log::info!("updating and unlocating settle");
                    }
                },
                Location::Puzzle(ref mut puzzle) => {
                    let lpos = puzzle.get_pos();
                    if !in_range(lpos, (self.dist_fo.0*-1, self.dist_fo.1*-1), self.loc_rad) {
                        //settle.tog_npcs_sent();
                        self.puzzles.update_puzzle(puzzle.clone());
                        self.location = Location::Null;
                        //log::info!("updating and unlocating settle");
                    }
                },
                _ => todo!(),
            }
        }
    }

    pub fn update_settlement(&mut self, mut settle: Settlement) -> Location {
        let lpos = settle.get_pos();
        let pos = self.dist_fo;
        let dx = (lpos.0 + pos.0) as usize;
        let dy = (lpos.1 + pos.1) as usize;
        //log::info!("up_set: {} - {}", dx, dy);
        if dx < MAP_W && dy < MAP_H {
            if !settle.get_npcs_sent() {
                log::info!("getting items & npcs for {}", settle.get_sname());
                let sitems = settle.get_items();
                for ((x, y), mut i) in sitems {
                    let ipos = i.get_pos();
                    if pos == (0, 0) {
                        // (dist_fo.0 + x as i64 + spos.0) as usize;
                        let npos = ((self.dist_fo.0 + ipos.0 as i64 + lpos.0) as usize, (self.dist_fo.1 + ipos.1 as i64 + lpos.1) as usize);
                        i.set_pos(npos);
                        log::info!("pos: {:?} | item: {:?}", npos, i);
                        self.items.insert(npos, i.clone());
                    } else {
                        let npos = ((self.dist_fo.0 + ipos.0 as i64 + lpos.0) as usize, (self.dist_fo.1 + ipos.1 as i64 + lpos.1) as usize);
                        i.set_pos(npos);
                        log::info!("pos: {:?} | item: {:?}", npos, i);
                        self.items.insert(npos, i.clone());
                    }
                }
                let tnpcs = settle.get_npcs();
                for ((x, y), n) in tnpcs {
                    log::info!("{:?}", n);
                    let mut nbox = box_npc(n);
                    let npos = nbox.get_pos();
                    if pos == (0, 0) {
                        let nwpos = ((self.dist_fo.0 + x as i64 + lpos.0) as usize, (self.dist_fo.1 + y as i64 + lpos.1) as usize);
                        nbox.set_pos(nwpos);
                        self.npcs.insert(nwpos, wrap_nbox(nbox));
                    } else {
                        let nwpos = ((self.dist_fo.0 + x as i64 + lpos.0) as usize, (self.dist_fo.1 + y as i64 + lpos.1) as usize);
                        nbox.set_pos(nwpos);
                        self.npcs.insert(nwpos, wrap_nbox(nbox));
                    }
                }
                settle.tog_npcs_sent();
            }
        }
        Location::Settlement(settle.clone())
    }

    pub fn in_loc_check(&mut self, pos: (usize, usize)) -> bool {
        let loc = self.location.clone();
        let dpos = self.dist_fo;
        match loc {
            Location::Null => false,
            Location::Settlement(mut settle) => {
                let lpos = settle.get_pos();
                let (xx, yy) = ((lpos.0 + dpos.0) as usize, (lpos.1 + dpos.1) as usize);
                if pos.0 >= xx && pos.0 <= xx+150 && pos.1 >= yy && pos.1 <= yy+50 {
                    return true;
                } else {return false;}
            },
            Location::Puzzle(mut puzzle) => {
                let lpos = puzzle.get_pos();
                let (xx, yy) = ((lpos.0 + dpos.0) as usize, (lpos.1 + dpos.1) as usize);
                if pos.0 >= xx && pos.0 <= xx+300 && pos.1 >= yy && pos.1 <= yy+200 {
                    return true;
                } else {return false;}
            },
            _ => false,
        }
    }

    pub fn buy_item(&mut self) {
        let mut item = {
            match self.interactee.clone() {
                Interactable::ShopItem(sitem) => sitem,
                _ => todo!(),
            }
        };
        let mut shop = self.get_shop_from_item(item.clone());
        let price = item.get_properties()["value"];
        let paid = self.player.dec_money(price);
        if paid {
            self.player.add_to_inv(item.clone());
            let ipos = item.get_pos();
            let mut loc = match self.location.clone() {
                Location::Settlement(settle) => settle,
                _ => todo!(),
            };
            let lpos = loc.get_pos();
            shop.set_paid(true);
            shop.remove_item(((ipos.0 as i64 - lpos.0 - self.dist_fo.0) as usize, (ipos.1 as i64 - lpos.1 - self.dist_fo.1) as usize));
            loc.update_shop(shop);
            self.location = Location::Settlement(loc);
        } else {
            shop.set_paid(false);
            let mut loc = match self.location.clone() {
                Location::Settlement(settle) => settle,
                _ => todo!(),
            };
            loc.update_shop(shop);
            self.location = Location::Settlement(loc);
        }
    }

    pub fn shop_key(&mut self, code: KeyCode) -> (bool, bool) {
        match code {
            KeyCode::Up => {
                self.gui.move_cursor("UP");
            },
            KeyCode::Down => {
                self.gui.move_cursor("DN");
            },
            KeyCode::Left => {
                self.gui.move_cursor("LF");
            },
            KeyCode::Right => {
                self.gui.move_cursor("RT");
            },
            KeyCode::Char('p') => self.gui.set_info_mode(GUIMode::Bug),
            KeyCode::Char('o') => self.gui.set_info_mode(GUIMode::Normal),
            KeyCode::Char('z') => {
                self.gui.set_info_mode(GUIMode::Normal);
                self.game_mode = GameMode::Play;
            },
            KeyCode::Char('a') => self.gui.move_cursor("LF"),
            KeyCode::Char('s') => self.gui.move_cursor("UP"),
            KeyCode::Char('d') => self.gui.move_cursor("DN"),
            KeyCode::Char('f') => self.gui.move_cursor("RT"),
            KeyCode::Enter => {
                let buy = self.gui.get_ysno();
                if buy {
                    self.buy_item();
                    return (false, true);
                } else {
                    return (false, false);
                }

            },
            _ => {},
        }
        (true, false)
    }

    pub fn shop_item_interaction(&mut self, mut sitem: Item) -> bool {
        let mut shop = self.get_shop_from_item(sitem.clone());
        // log::info!("shop  \n{:?}", shop.clone());
        let npc = shop.get_npc();
        // log::info!("shop npc \n{:?}", npc.clone());
        let (sname, sh_convo) = match npc {
            NPCWrap::ShopNPC(mut snpc) => (snpc.get_sname(), snpc.get_sh_conv()),
            _ => todo!(),
        };
        let iprice = sitem.get_properties()["value"].to_string();
        let dialogue_temp = &sh_convo["item_desc"];
        let sh_dialogue = dialogue_temp.replace("{i}", &sitem.get_sname()).replace("{v}", &iprice);
        // let sh_dialogue = format!(form_dialogue.as_str(), sitem.get_sname(), iprice);
        // let sh_dialogue = fmt::format(format_args!(format!(dialogue_temp, sitem.sname(), iprice)));
        let mut buy_item = false;
        self.gui.reset_cursor();
        loop {
            self.gui.shop_convo_draw(sname.clone(), sh_dialogue.clone(), self.map.clone(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone(), loc_shop_items(self.dist_fo.clone(), self.location.clone()));
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        let res = self.shop_key(event.code);
                        if !res.0 {
                            buy_item = res.1;
                            break
                        }
                    }
                }
            }
        }
        let mut nshop = self.get_shop_from_item(sitem.clone());
        let resp_dialogue = {
            if buy_item {
                if nshop.get_paid() {
                    &sh_convo["item_bought"]
                } else {
                    &sh_convo["item_broke"]
                }
            } else {
                &sh_convo["item_nbought"]
            }
        };
        self.gui.reset_cursor();
        loop {
            self.gui.shop_convo_draw(sname.clone(), resp_dialogue.clone(), self.map.clone(), self.player.clone(), self.enemies.clone(), self.items.clone(), self.npcs.clone(), loc_shop_items(self.dist_fo.clone(), self.location.clone()));
            if poll(std::time::Duration::from_millis(100)).unwrap() {
                if let Event::Key(event) = read().unwrap() {
                    // log::info!("keykind {:?}", event.kind.clone());
                    let now = Instant::now();
                    if now.duration_since(self.last_event_time) > self.key_debounce_dur {
                        self.last_event_time = now;
                        match event.code {
                            KeyCode::Enter => {
                                break;
                            },
                            _ => todo!(),
                        }
                    }
                }
            }
        }
        self.game_mode = GameMode::Play;
        true
    }
}