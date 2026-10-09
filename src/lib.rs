mod enums;
mod structs;
const BGM_ENTRY_COUNT: usize = 127;

#[skyline::from_offset(0x00045fc0)]
fn get_current_encounter_id() -> u16;

#[skyline::from_offset(0x001d0c20)]
fn is_player_femc() -> PlayerGender;

#[skyline::from_offset(0x002cb220)]
fn play_bgm_from_table(a1: u16);

#[skyline::from_offset(0x001d2d20)]
fn get_current_timeof_day() -> enums::TimeOfDay;

pub fn patch_bgm_table() {
    let text_region = unsafe { skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as *const u8 };

    let bgm_table = unsafe { std::slice::from_raw_parts_mut(text_region.add(0x00707be8) as *mut structs::BGMTableStruct, BGM_ENTRY_COUNT) };
    bgm_table[structs::BGM::ColorYourNight].bgm_name = skyline::c_str("ColorYourNight.ADX\0");
    bgm_table[structs::BGM::Chance_M].bgm_name = skyline::c_str("Chance_M.ADX\0");
    bgm_table[structs::BGM::Chance_F].bgm_name = skyline::c_str("Chance_F.ADX\0");
    bgm_table[structs::BGM::Pinch_M].bgm_name = skyline::c_str("Pinch_M.ADX\0");
    bgm_table[structs::BGM::Pinch_F].bgm_name = skyline::c_str("Pinch_F.ADX\0");
    bgm_table[structs::BGM::MasterOfShadows_F].bgm_name = skyline::c_str("MasterOfShadows_F.ADX\0");
    bgm_table[structs::BGM::UnavoidableBattle_F].bgm_name = skyline::c_str("UnavoidableBattle_F.ADX\0");
    bgm_table[structs::BGM::BurnMyDreadFinal_F].bgm_name = skyline::c_str("BurnMyDreadFinal_F.ADX\0");
    bgm_table[structs::BGM::BattleHymnOfTheSoul_F].bgm_name = skyline::c_str("BattleHymnOfTheSoul_F.ADX\0");
    bgm_table[structs::BGM::HeartfulCry].bgm_name = skyline::c_str("HeartfulCry.ADX\0");
    bgm_table[structs::BGM::Darkness].bgm_name = skyline::c_str("Darkness.ADX\0");
}

pub fn get_encounter_context() -> &'static mut structs::EncounterContextHolder {
    unsafe {
        let encounter_ctx = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text).byte_add(0x7108b8) as *mut &mut structs::EncounterContextHolder;
        *encounter_ctx
    }
}

pub fn get_encounter_contex_value() -> u16 {
    get_encounter_context().encounter_contex_value
}

pub fn get_encount_section1() -> &'static mut [structs::EncountSection1;1000] {
    unsafe {
        let encounter_ctx = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text).byte_add(0x710a20) as *mut &mut [structs::EncountSection1;1000];
        *encounter_ctx
    }
}

pub fn get_player_gender() -> PlayerGender {
    unsafe{ is_player_femc() }
}

pub enum PlayerGender {
    Male,
    Female
}

pub fn map_bgm_male(bgm: u16) -> u16 {
    match bgm {
        2 => structs::BGM::MasterOfShadows as u16,
        3 => structs::BGM::UnavoidableBattle as u16,
        4 => structs::BGM::BurnMyDreadFinal as u16,
        5 => structs::BGM::BattleHymnOfTheSoul as u16,
        6 => structs::BGM::MasterOfTartarus as u16,
        7 => structs::BGM::HeartfulCry as u16,
        8 => structs::BGM::Darkness as u16,
        _ => match get_encounter_contex_value() {
                1 => structs::BGM::Chance_M as u16,
                2 => structs::BGM::Pinch_M as u16,
                _ => structs::BGM::MassDestruction as u16,
            }
        }
}

pub fn map_bgm_female(bgm: u16) -> u16 {
    match bgm {
        2 => structs::BGM::MasterOfShadows_F as u16,
        3 => structs::BGM::UnavoidableBattle_F as u16,
        4 => structs::BGM::BurnMyDreadFinal_F as u16,
        5 => structs::BGM::BattleHymnOfTheSoul_F as u16,
        6 => structs::BGM::DangerZone as u16,
        7 => structs::BGM::HeartfulCry as u16,
        8 => structs::BGM::Darkness as u16,
        _ => match get_encounter_contex_value() {
                1 => structs::BGM::Chance_F as u16,
                2 => structs::BGM::Pinch_F as u16,
                _ => structs::BGM::WipingAllOut as u16,
            }
    }
}

#[skyline::hook(offset = 0x0005ffa0)]
pub fn decide_battle_bgm() {
    call_original!();

        let encounterID = unsafe{get_current_encounter_id()};
        let bgm = get_encount_section1()[encounterID as usize].bgm;

        let target_bgm = match get_player_gender() {
            PlayerGender::Male => map_bgm_male(bgm),
            PlayerGender::Female => map_bgm_female(bgm),
        };

        unsafe{play_bgm_from_table(target_bgm)}
}

#[skyline::hook(offset = 0x002cb220)]
pub fn play_bgm_from_table_fn(a1: structs::BGM) {
    let mut targetbgm = a1;
    let mut currenttime: enums::TimeOfDay;

    unsafe{ currenttime = get_current_timeof_day(); }

    if (targetbgm == structs::BGM::WhenTheMoonsReachingOutStars || targetbgm == structs::BGM::WayOfLife) 
    && currenttime == enums::TimeOfDay::Night {
        targetbgm = structs::BGM::ColorYourNight;
    }

    call_original!(targetbgm)
}

#[skyline::main(name = "modBGMEnhance")]
pub fn main() {
    println!("Hello from skyline plugin");

    // Install panic hook to display errors on the screen, since we don't have a network logger for now
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };

        let err_msg = format!("thread has panicked at '{}', {}", msg, location);

        skyline::error::show_error(
            69,
            "Skyline plugin as panicked! Please open the details and send a screenshot to the developer, then close the game.\n",
            err_msg.as_str(),
        );
    }));

    skyline::install_hook!(decide_battle_bgm);
    skyline::install_hook!(play_bgm_from_table_fn);
    patch_bgm_table();
}
