use crate::tkl::bit::player_data::*;

// health: 8, bullets: 8, brothers: 4, empty: 8, berserk:1, shield:1, infBullets:1, god:1

pub fn run() {
    let mut p_data = PlayerData::default();

    p_data.set(&SPN_HLTH, 10);
    println!("    set health: {}", p_data.get(&SPN_HLTH));
    p_data.add(&SPN_HLTH, 59);
    println!("    add health: {}", p_data.get(&SPN_HLTH));
    p_data.add(&SPN_HLTH, -27);
    println!("    subtract health: {}", p_data.get(&SPN_HLTH));
    p_data.clr(&SPN_HLTH);
    println!("    clear health: {}", p_data.get(&SPN_HLTH));

    println!();

    p_data.set(&SPN_BLTS, 10);
    println!("    set bullets: {}", p_data.get(&SPN_BLTS));
    p_data.add(&SPN_BLTS, 59);
    println!("    add bullets: {}", p_data.get(&SPN_BLTS));
    p_data.add(&SPN_BLTS, -27);
    println!("    subtract bullets: {}", p_data.get(&SPN_BLTS));
    p_data.clr(&SPN_BLTS);
    println!("    clear bullets: {}", p_data.get(&SPN_BLTS));

    println!();

    p_data.set(&SPN_BRTHS, 10);
    println!("    set brothers: {}", p_data.get(&SPN_BRTHS));
    p_data.add(&SPN_BRTHS, 5);
    println!("    add brothers: {}", p_data.get(&SPN_BRTHS));
    p_data.add(&SPN_BRTHS, -8);
    println!("    subtract brothers: {}", p_data.get(&SPN_BRTHS));
    p_data.clr(&SPN_BRTHS);
    println!("    clear brothers: {}", p_data.get(&SPN_BRTHS));

    println!();

    p_data.set(&SPN_BRSK, 1);
    println!("    set berserker: {}", p_data.get(&SPN_BRSK));
    p_data.clr(&SPN_BRSK);
    println!("    clear berserker: {}", p_data.get(&SPN_BRSK));

    println!();

    p_data.set(&SPN_SHLD, 1);
    println!("    set shield: {}", p_data.get(&SPN_SHLD));
    p_data.clr(&SPN_SHLD);
    println!("    clear shield: {}", p_data.get(&SPN_SHLD));

    println!();

    p_data.set(&SPN_INFBULTS, 1);
    println!("    set infinite bullets: {}", p_data.get(&SPN_INFBULTS));
    p_data.clr(&SPN_INFBULTS);
    println!("    clear infinite bullets: {}", p_data.get(&SPN_INFBULTS));

    println!();

    p_data.set(&SPN_GOD, 1);
    println!("    set god: {}", p_data.get(&SPN_GOD));
    p_data.clr(&SPN_GOD);
    println!("    clear god: {}", p_data.get(&SPN_GOD));

    println!();
}
