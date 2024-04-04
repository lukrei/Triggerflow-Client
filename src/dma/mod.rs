use std::{thread, time::{Duration, Instant}};

use memflow::{os::Process, types::Address, mem::MemoryView};

use crate::{enums::{TeamID, PlayerType}, comms::{EntityData, PlayerData, RadarData, ArcRwlockRadarData, BombData}};

use self::{context::DmaCtx, threaddata::CsData};

// tcp client zu hack server auf windows
use std::net::UdpSocket;
use std::io::Write;
use std::collections::HashSet;

mod context;
mod threaddata;
mod cs2dumper;

pub use context::Connector;

pub async fn run(radar_data: ArcRwlockRadarData, connector: Connector, pcileech_device: String, skip_version: bool) -> anyhow::Result<()> {
    let mut ctx = DmaCtx::setup(connector, pcileech_device, skip_version)?;
    let mut data = CsData::default();
    
    // For read timing
    let mut last_bomb_dropped = false;
    let mut last_bomb_planted = false;
    let mut last_tick_count = 0;
    let mut last_big_read = Instant::now();

    // For frequency info
    let mut start_stamp = Instant::now();
    let mut iters = 0;
    let mut freq = 0;

    data.update_pointers(&mut ctx);
    data.update_common(&mut ctx);
    data.update_players(&mut ctx);
    data.update_bomb(&mut ctx);

    //CHATGPT
    let mut unique_crosshair_ids = HashSet::new();
    //CHATGPT END
    loop {
        if ctx.process.state().is_dead() {
            break;
        }

        if last_big_read.elapsed().as_millis() > 10000 {
            data.update_pointers(&mut ctx);
            data.update_players(&mut ctx);
            last_big_read = Instant::now();
        }

        data.update_common(&mut ctx);

        // Bomb update
        if (data.bomb_dropped && !last_bomb_dropped) || (data.bomb_planted && !last_bomb_planted) {
            data.update_bomb(&mut ctx);
        }

        if (!data.bomb_dropped && last_bomb_dropped) || !data.bomb_planted {
            data.recheck_bomb_holder = true;
        }


        let bomb_defuse_timeleft: f32 = {
            if data.bomb_planted && !data.bomb_exploded && !data.bomb_defused {
                if let Some(bomb_stamp) = data.bomb_planted_stamp {
                    data.bomb_plant_timer - bomb_stamp.elapsed().as_secs_f32()
                } else {
                    0.0
                }
            } else {
                0.0
            }
        };

        let bomb_can_defuse: bool = {
            if data.bomb_planted && !data.bomb_exploded && !data.bomb_defused {
                if let (Some(bomb_stamp), Some(defuse_stamp)) = (data.bomb_planted_stamp, data.bomb_defuse_stamp) {
                    let time_left = data.bomb_plant_timer - bomb_stamp.elapsed().as_secs_f32();
                    let defuse_left = data.bomb_defuse_length - defuse_stamp.elapsed().as_secs_f32();
                    time_left - defuse_left > 0.0
                } else {
                    false
                }
            } else {
                false
            }
        };

        last_bomb_dropped = data.bomb_dropped;
        last_bomb_planted = data.bomb_planted;

        // Poll entity data
        let ingame = !data.map.is_empty() && data.map != "<empty>";
        let update_data = data.tick_count != last_tick_count;
    
        if ingame {
            if !update_data {
                continue;
            }

            let mut entity_data = Vec::new();

            // Bomb
            if data.bomb_dropped || data.bomb_planted {
                let node = ctx.process.read_addr64(
                    data.bomb + cs2dumper::client::C_BaseEntity::m_pGameSceneNode as u64
                ).unwrap();
                let pos = ctx.process.read(node + cs2dumper::client::CGameSceneNode::m_vecAbsOrigin).unwrap();
    
                entity_data.push(EntityData::Bomb(BombData::new(pos, data.bomb_planted)));
            }

            // Local player
            let local_data = ctx.batched_player_read(
                data.local.into(), data.local_pawn.into()
            ).unwrap();

            if local_data.health > 0 {
                let has_bomb = {
                    if data.bomb_planted {
                        false
                    } else if data.recheck_bomb_holder {
                        if local_data.team == Some(TeamID::T) && !data.bomb_dropped && !data.bomb_planted {
                            let is_holder = ctx.has_c4(
                                data.local_pawn.into(), data.entity_list.into()
                            ).unwrap_or(false);

                            if is_holder {
                                data.bomb_holder = data.local.into();
                                data.recheck_bomb_holder = false;
                            }

                            is_holder
                        } else { false }
                    } else { Address::from(data.local) == data.bomb_holder }
                };

                entity_data.push(
                    EntityData::Player(
                        PlayerData::new(
                            local_data.pos, 
                            local_data.yaw,
                            PlayerType::Local,
                            has_bomb,
                            local_data.has_awp,
                            local_data.is_scoped
                        )
                    )
                );
            }
                //CHATGPT
                
                let mut temp_unique_crosshair_ids = HashSet::new();
                //unique_crosshair_ids.insert(player_data.crosshair_id);
                //CHATGPT END
            //DEBUG LUKI!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            //println!("Team:{}", local_data.health);
            //herausfinden was in data.teams drinsteht variablentype??? objekt oder so t und ct??? local_data.team.m versuchen etc...
            //println!("Team:{}", local_data.team);
            println!("Crosshair ID:{}", local_data.crosshair_id);
            if unique_crosshair_ids.len() <= 4 {
            unique_crosshair_ids.insert(local_data.crosshair_id);
            temp_unique_crosshair_ids = unique_crosshair_ids.clone();
            }
            //for (index, &value) in temp_unique_crosshair_ids.iter().enumerate() {
                //println!("Value at index {}: {}", index, value);
                //let first_value = unique_crosshair_ids.iter().cloned().nth(0).unwrap();
            //}
            //unique_crosshair_ids.clear();
            //println!("Length:{}", unique_crosshair_ids.len())
                // Get the local player base address
                if local_data.team == Some(TeamID::T) {
                    println!("Terror");
                    //continue;
                }
                if local_data.team == Some(TeamID::CT) {
                    println!("Counter-Terror");
                    //continue;
                }

                // Continue if there is no entity on crosshair
                if unique_crosshair_ids.len() >= 5 {
                    //if local_data.crosshair_id<255 && local_data.crosshair_id!=unique_crosshair_ids.iter().cloned().nth(0).unwrap() && local_data.crosshair_id!=unique_crosshair_ids.iter().cloned().nth(1).unwrap() && local_data.crosshair_id!=unique_crosshair_ids.iter().cloned().nth(2).unwrap() && local_data.crosshair_id!=unique_crosshair_ids.iter().cloned().nth(3).unwrap() && local_data.crosshair_id!=unique_crosshair_ids.iter().cloned().nth(4).unwrap(){
                        if (local_data.crosshair_id!=unique_crosshair_ids.iter().cloned().nth(0).unwrap() && local_data.crosshair_id!=unique_crosshair_ids.iter().cloned().nth(1).unwrap() && local_data.crosshair_id!=unique_crosshair_ids.iter().cloned().nth(2).unwrap() && local_data.crosshair_id!=unique_crosshair_ids.iter().cloned().nth(3).unwrap() && local_data.crosshair_id!=unique_crosshair_ids.iter().cloned().nth(4).unwrap()) && local_data.crosshair_id!=255{
                        println!("Schiessen");
                        println!("Entity List:{}", data.entity_list);
                        let udp_socket = UdpSocket::bind("0.0.0.0:1337").expect("Failed to bind UDP socket"); // Bind to a random local portlet server_address = "192.168.50.3:1337"; // Replace with the actual server address
                        let server_address = "192.168.50.3:1337"; // Replace with the actual server address
                        let schiessen = true; // Ihre boolsche Variable
                        let byte_value = if schiessen { 1 } else { 0 }; // Konvertieren Sie bool in ein Byte
                        match udp_socket.send_to(&[byte_value], server_address) {
                            Ok(_) => println!("Bool value successfully sent"),
                            Err(e) => eprintln!("Error sending: {}", e),
                        }
                    }
                }

        }
// DEBUG LUKI END
//            // Other players
//            for (controller, pawn) in &data.players {
//                let player_data = ctx.batched_player_read(*controller, *pawn).unwrap();
//
//                if player_data.health < 1 {
//                    continue;
//                }
//
//                let has_bomb = {
//                    if data.bomb_planted {
//                        false
//                    } else if data.recheck_bomb_holder {
//                        if player_data.team == Some(TeamID::T) && !data.bomb_dropped && !data.bomb_planted {
//                            let is_holder = ctx.has_c4(*pawn, data.entity_list.into()).unwrap_or(false);
//
//                            if is_holder {
//                                data.bomb_holder = *controller;
//                                data.recheck_bomb_holder = false;
//                            }
//
//                            is_holder
//                        } else { false }
//                    } else { *controller == data.bomb_holder }
//                };

//                let player_type = {
//                    if local_data.team != player_data.team {
//                        PlayerType::Enemy
//                    } else if local_data.team == player_data.team {
//                        PlayerType::Team
//                    } else {
//                        PlayerType::Unknown
//                    }
//                };

//                entity_data.push(
//                    EntityData::Player(
//                        PlayerData::new(
//                            player_data.pos, 
//                            player_data.yaw,
//                            player_type,
//                            has_bomb,
//                            player_data.has_awp,
//                            player_data.is_scoped
//                        )
//                    )
//                );
//            }

//            let mut radar = radar_data.write().await;
//            *radar = RadarData::new(
//                true,
//                data.map.clone(),
//                entity_data,
//                freq,
//                data.bomb_planted,
//                bomb_can_defuse,
//                bomb_defuse_timeleft,
//                data.bomb_exploded,
//                data.bomb_being_defused,
//                data.bomb_defuse_length
//            );
//        } else {
//            let mut radar = radar_data.write().await;
//            *radar = RadarData::empty(freq);
//        }

        last_tick_count = data.tick_count;
        iters += 1;
    
        if start_stamp.elapsed().as_secs() > 1 {
            freq = iters;
            iters = 0;
            start_stamp = Instant::now();
        }
    
        thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
