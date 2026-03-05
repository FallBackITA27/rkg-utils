use chrono::NaiveDateTime;

use crate::{
    Ghost,
    ctgp_footer::CTGPFooter,
    header::{
        Header,
        combo::{Character, Combo, Vehicle},
        controller::Controller,
        date::Date,
        ghost_type::GhostType,
        in_game_time::InGameTime,
        location::{Location, constants::*},
        mii::{
            birthday::Birthday,
            build::Build,
            eyebrows::{EyebrowType, Eyebrows},
            eyes::{EyeColor, EyeType, Eyes},
            facial_hair::{BeardType, FacialHair, MustacheType},
            favorite_color::FavoriteColor,
            glasses::{Glasses, GlassesColor, GlassesType},
            hair::{Hair, HairColor, HairType},
            head::{FaceFeatures, Head, HeadShape, SkinTone},
            lips::{Lips, LipsColor, LipsType},
            mii_type::MiiType,
            mole::Mole,
            nose::{Nose, NoseType},
        },
        slot_id::SlotId,
    },
    input_data::{InputData, yaz1_compress, yaz1_decompress},
};
use std::io::Read;

#[test]
fn test_rkg_header() {
    let header =
        Header::new_from_path("./test_ghosts/JC_LC_Compressed.rkg").expect("Couldn't read header");

    // General ghost info
    assert_eq!(header.finish_time().minutes(), 1);
    assert_eq!(header.finish_time().seconds(), 3);
    assert_eq!(header.finish_time().milliseconds(), 904);
    assert_eq!(header.finish_time().to_string(), "01:03.904");
    assert_eq!(header.slot_id(), SlotId::LuigiCircuit);
    assert_eq!(header.combo().vehicle(), Vehicle::WarioBike);
    assert_eq!(header.combo().character(), Character::KingBoo);
    assert_eq!(header.date_set(), &Date::new(2025, 11, 12).unwrap());
    assert_eq!(header.controller(), Controller::Classic);
    assert!(header.is_compressed());
    assert_eq!(header.ghost_type(), GhostType::ExpertStaff);
    assert!(header.is_automatic_drift());
    assert_eq!(header.decompressed_input_data_length(), 1856);
    assert_eq!(header.lap_count(), 3);
    assert_eq!(header.lap_split_times()[0].to_string(), "00:25.540");
    assert_eq!(header.lap_split_times()[1].to_string(), "00:19.127");
    assert_eq!(header.lap_split_times()[2].to_string(), "00:19.237");
    assert_eq!(header.location().country(), Country::NotSet);
    assert_eq!(header.location().subregion(), Subregion::NotSet);

    // Mii Data
    assert!(!header.mii().is_girl());
    assert_eq!(header.mii().birthday().month(), Some(1));
    assert_eq!(header.mii().birthday().day(), Some(1));
    assert_eq!(header.mii().favorite_color(), FavoriteColor::ForestGreen);
    assert_eq!(header.mii().name(), "JC");
    assert_eq!(header.mii().build().height(), 127);
    assert_eq!(header.mii().build().weight(), 127);

    assert_eq!(header.mii().mii_id(), 0x893EF2FB);
    assert_eq!(header.mii().system_id(), 0x689EC992);

    assert_eq!(header.mii().head().shape(), HeadShape::Large);
    assert_eq!(header.mii().head().skin_tone(), SkinTone::Natural);
    assert_eq!(header.mii().head().face_features(), FaceFeatures::None);

    assert!(!header.mii().is_mingle_enabled());
    assert!(!header.mii().downloaded());

    assert_eq!(header.mii().hair().hair_type(), HairType::NormalLong);
    assert_eq!(header.mii().hair().hair_color(), HairColor::PhilippineBrown);
    assert!(!header.mii().hair().is_flipped());

    assert_eq!(header.mii().eyebrows().eyebrow_type(), EyebrowType::None);
    assert_eq!(header.mii().eyebrows().rotation(), 5);
    assert_eq!(
        header.mii().eyebrows().eyebrow_color(),
        HairColor::Chocolate
    );
    assert_eq!(header.mii().eyebrows().size(), 4);
    assert_eq!(header.mii().eyebrows().y(), 10);
    assert_eq!(header.mii().eyebrows().x(), 2);

    assert_eq!(header.mii().eyes().eye_type(), EyeType::DotAngry);
    assert_eq!(header.mii().eyes().rotation(), 4);
    assert_eq!(header.mii().eyes().y(), 9);
    assert_eq!(header.mii().eyes().eye_color(), EyeColor::Black);
    assert_eq!(header.mii().eyes().size(), 6);
    assert_eq!(header.mii().eyes().x(), 1);

    assert_eq!(header.mii().nose().nose_type(), NoseType::Dot);
    assert_eq!(header.mii().nose().size(), 0);
    assert_eq!(header.mii().nose().y(), 8);

    assert_eq!(header.mii().lips().lips_type(), LipsType::WaveAngry);
    assert_eq!(header.mii().lips().lips_color(), LipsColor::Orange);
    assert_eq!(header.mii().lips().size(), 7);
    assert_eq!(header.mii().lips().y(), 6);

    assert_eq!(header.mii().glasses().glasses_type(), GlassesType::None);
    assert_eq!(header.mii().glasses().glasses_color(), GlassesColor::Black);
    assert_eq!(header.mii().glasses().size(), 4);
    assert_eq!(header.mii().glasses().y(), 10);

    assert_eq!(
        header.mii().facial_hair().mustache_type(),
        MustacheType::None
    );
    assert_eq!(header.mii().facial_hair().beard_type(), BeardType::None);
    assert_eq!(header.mii().facial_hair().color(), HairColor::Black);
    assert_eq!(header.mii().facial_hair().mustache_size(), 4);
    assert_eq!(header.mii().facial_hair().mustache_y(), 10);

    assert!(!header.mii().mole().has_mole());
    assert_eq!(header.mii().mole().size(), 4);
    assert_eq!(header.mii().mole().y(), 20);
    assert_eq!(header.mii().mole().x(), 2);

    assert_eq!(header.mii().creator_name(), "JC");

    assert_eq!(header.mii_crc16(), 0x06F4);
    assert!(header.verify_mii_crc16());
}

#[test]
fn print_rkg_header() {
    let header = Header::new_from_path("./test_ghosts/JC_LC_Compressed_Copy.rkg")
        .expect("Couldn't read header");

    println!("Track: {}", header.slot_id());
    println!("Time: {}", header.finish_time());
    println!("Date set: {:?}", header.date_set());
    println!("Player: {}", header.mii().name());
    println!("Country: {}", header.location().country());
    println!("Subregion: {}", header.location().subregion());
    println!("Controller: {}", header.controller());
    println!(
        "Combo: {} ({} Drift)",
        header.combo(),
        if header.is_automatic_drift() {
            "Automatic"
        } else {
            "Manual"
        }
    );
    println!("Ghost type: {}", header.ghost_type());
    println!("Input data compressed? {}", header.is_compressed());
    println!(
        "Decompressed input data length: {}",
        header.decompressed_input_data_length()
    );
    println!("Total lap count: {}\n", header.lap_count());

    for (index, lap_time) in header.lap_split_times().iter().enumerate() {
        println!("Lap {}: {}", index + 1, lap_time);
    }
}

#[test]
fn test_rkg_input_data() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/JC_LC_Compressed.rkg")
        .expect("Couldn't find `./test_ghosts/JC_LC_Compressed.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let input_data =
        InputData::new(&rkg_data[0x88..rkg_data.len() - 0xE0]).expect("Couldn't read input data");

    assert_eq!(input_data.face_input_count(), 0x18);
    assert_eq!(input_data.stick_input_count(), 0x037B);
    assert_eq!(input_data.dpad_input_count(), 0x09);
    assert_eq!(input_data.inputs().len(), 907);
    assert_eq!(input_data.face_inputs().len(), 12);
    assert_eq!(input_data.stick_inputs().len(), 891);
    assert_eq!(input_data.dpad_inputs().len(), 9);

    assert!(!input_data.contains_illegal_brake_or_drift_inputs());
}

#[test]
fn print_input_data() {
    let ghost = Ghost::new_from_file("./test_ghosts/illegal_brake_input.rkg").unwrap();

    for input in ghost.input_data().inputs().iter() {
        println!("{:#?}", input)
    }
}

#[test]
fn test_ctgp_metadata() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/JC_LC_Compressed.rkg")
        .expect("Couldn't find `./test_ghosts/JC_LC_Compressed.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let ctgp_metadata = CTGPFooter::new(&rkg_data).expect("Failed to read CTGP metadata");

    // Some asserts
    assert_eq!(
        ctgp_metadata.track_sha1(),
        [
            0x1A, 0xE1, 0xA7, 0xD8, 0x94, 0x96, 0x0B, 0x38, 0xE0, 0x9E, 0x74, 0x94, 0x37, 0x33,
            0x78, 0xD8, 0x73, 0x05, 0xA1, 0x63
        ]
    );
    assert_eq!(
        ctgp_metadata.player_id().to_be_bytes(),
        [0xFD, 0x31, 0x97, 0xB0, 0x7D, 0x9D, 0x2B, 0x84]
    );
    let shroomstrat: [u8; 3] = [3, 0, 0];
    assert_eq!(ctgp_metadata.shroomstrat(), &shroomstrat);
}

#[test]
fn print_ctgp_metadata() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/skylar_pause_ghost_compressed.rkg")
        .expect("Couldn't find `./test_ghosts/skylar_pause_ghost_compressed.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let ctgp_metadata = CTGPFooter::new(&rkg_data).expect("Failed to read CTGP metadata");

    // Print info
    print!("Track SHA1: ");
    for byte in ctgp_metadata.track_sha1().iter() {
        print!("{:02X}", *byte);
    }
    println!();

    print!("Ghost SHA1: ");
    for byte in ctgp_metadata.ghost_sha1().iter() {
        print!("{:02X}", *byte);
    }
    println!();

    print!("Player ID: ");
    for byte in ctgp_metadata.player_id().to_be_bytes().iter() {
        print!("{:02X}", *byte);
    }
    println!();

    println!("Exact finish time: {}", ctgp_metadata.exact_finish_time());
    println!("CORE version: {}", ctgp_metadata.core_version());
    print!("Possible CTGP release versions: ");
    if let Some(ctgp_versions) = ctgp_metadata.possible_ctgp_versions() {
        for version in ctgp_versions {
            print!("{}, ", version)
        }
        println!();
    } else {
        println!("Unknown")
    }
    println!();

    for (index, time) in ctgp_metadata.exact_lap_times().iter().enumerate() {
        println!("Lap {}: {}", index + 1, time);
    }
    println!();

    println!("RTC Race Begin: {}", ctgp_metadata.rtc_race_begins());
    println!("RTC Race End: {}", ctgp_metadata.rtc_race_end());
    println!(
        "RTC Time Paused: {}ms",
        ctgp_metadata.rtc_time_paused().num_milliseconds()
    );
    println!("List of pause times: [");
    for time in ctgp_metadata.pause_times() {
        println!("  {},", time)
    }
    println!("]");

    println!("\nMy Stuff enabled? {}", ctgp_metadata.my_stuff_enabled());
    println!("My Stuff used? {}", ctgp_metadata.my_stuff_used());
    println!(
        "USB Gamecube enabled? {}",
        ctgp_metadata.usb_gamecube_enabled()
    );
    println!(
        "Final lap dubious intersection? {}",
        ctgp_metadata.final_lap_suspicious_intersection()
    );

    println!(
        "\nAll lap dubious intersection bools: {:?}",
        ctgp_metadata.lap_split_suspicious_intersections().unwrap()
    );

    println!("\nShroomstrat: {:?}", ctgp_metadata.shroomstrat());
    println!("Category: {:?}", ctgp_metadata.category());
    println!("Cannoned? {}", ctgp_metadata.cannoned());
    println!("Went OOB? {}", ctgp_metadata.went_oob());
    println!("Slowdown suspected? {}", ctgp_metadata.potential_slowdown());
    println!(
        "Rapidfire suspected? {}",
        ctgp_metadata.potential_rapidfire()
    );
    println!(
        "Suspicious ghost? {}",
        ctgp_metadata.potentially_cheated_ghost()
    );
    println!(
        "Has Mii data replaced? {}",
        ctgp_metadata.has_mii_data_replaced()
    );
    println!(
        "Has Mii name replaced? {}",
        ctgp_metadata.has_name_replaced()
    );
    println!("Respawns? {}", ctgp_metadata.respawns());
    println!("CTGP metadata version: {}", ctgp_metadata.footer_version());
}

/// CTGP adds a pause mask to frames where a pause is pressed. Actual race inputs should stay the same.
#[test]
fn test_ctgp_pause_vs_vanilla_input_timing() {
    let mut pause_rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/skylar_pause_ghost_compressed.rkg")
        .expect("Couldn't find `./test_ghosts/skylar_pause_ghost_compressed.rkg`")
        .read_to_end(&mut pause_rkg_data)
        .expect("Couldn't read bytes in file");

    let mut vanilla_rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/skylar_pause_ghost_vanilla.rkg")
        .expect("Couldn't find `./test_ghosts/skylar_pause_ghost_vanilla.rkg`")
        .read_to_end(&mut vanilla_rkg_data)
        .expect("Couldn't read bytes in file");

    let pause_inputs = InputData::new(&pause_rkg_data[0x88..pause_rkg_data.len() - 0xE0])
        .expect("Failed to read inputs from pause ghost");
    let vanilla_inputs = InputData::new(&vanilla_rkg_data[0x88..vanilla_rkg_data.len() - 0x04])
        .expect("Failed to read inputs from vanilla ghost");

    assert_eq!(pause_inputs.face_inputs(), vanilla_inputs.face_inputs());
    assert_eq!(pause_inputs.stick_inputs(), vanilla_inputs.stick_inputs());
    assert_eq!(pause_inputs.dpad_inputs(), pause_inputs.dpad_inputs());

    assert_eq!(pause_inputs.inputs(), vanilla_inputs.inputs());
}

#[test]
fn illegal_drift_input_test() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/illegal_drift_input.rkg")
        .expect("Couldn't find `./test_ghosts/illegal_drift_input.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let input_data =
        InputData::new(&rkg_data[0x88..rkg_data.len() - 0x04]).expect("Failed to read input data");
    assert!(input_data.contains_illegal_brake_or_drift_inputs());
}

#[test]
fn illegal_brake_input_test() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/illegal_brake_input.rkg")
        .expect("Couldn't find `./test_ghosts/illegal_brake_input.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let input_data =
        InputData::new(&rkg_data[0x88..rkg_data.len() - 0x04]).expect("Failed to read input data");
    assert!(input_data.contains_illegal_brake_or_drift_inputs());
}

#[test]
fn test_nine_laps() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/9laps_test.rkg")
        .expect("Couldn't find `./test_ghosts/9laps_test.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let header = Header::new(&rkg_data[..0x88]).expect("Couldn't read header");

    for (index, lap) in header.lap_split_times().iter().enumerate() {
        println!("Lap {}: {}", index + 1, lap.to_string());
    }

    println!("\nTotal time: {}", header.finish_time().to_string());
}

#[test]
fn test_exact_finish_time() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/00m58s6479888 David .rkg")
        .expect("Couldn't find `./test_ghosts/00m58s6479888 David .rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let ctgp_metadata = CTGPFooter::new(&rkg_data).expect("Failed to read CTGP metadata");

    assert_eq!(
        ctgp_metadata.exact_finish_time().to_string(),
        "00:58.647988872949"
    );
    assert_eq!(
        ctgp_metadata.exact_lap_times()[0].to_string(),
        "00:19.607006953895"
    );
    assert_eq!(
        ctgp_metadata.exact_lap_times()[1].to_string(),
        "00:19.623577742219"
    );
    assert_eq!(
        ctgp_metadata.exact_lap_times()[2].to_string(),
        "00:19.417404176835"
    );
}

#[test]
fn test_recompressed_input_data() {
    let mut rkg_data: Vec<u8> = Vec::new();
    std::fs::File::open("./test_ghosts/JC_LC_Compressed.rkg")
        .expect("Couldn't find `./test_ghosts/JC_LC_Compressed.rkg`")
        .read_to_end(&mut rkg_data)
        .expect("Couldn't read bytes in file");

    let original_data = &rkg_data[0x88..rkg_data.len() - 0xE0];
    let decompressed_data = yaz1_decompress(&original_data[0x04..]).unwrap();
    let recompressed_data = yaz1_compress(&decompressed_data);

    let original_input_data = InputData::new(&original_data).unwrap();
    let recompressed_input_data = InputData::new(&recompressed_data).unwrap();

    assert_eq!(
        (&original_data.len() - 12) as u32,
        u32::from_be_bytes([
            original_data[0],
            original_data[1],
            original_data[2],
            original_data[3]
        ])
    );
    assert_eq!(
        (recompressed_data.len() - 12) as u32,
        u32::from_be_bytes([
            recompressed_data[0],
            recompressed_data[1],
            recompressed_data[2],
            recompressed_data[3]
        ])
    );
    assert_eq!(
        original_input_data.inputs(),
        recompressed_input_data.inputs()
    );
    assert!(original_input_data.inputs().len() > 100);
}

#[test]
fn test_full_ghost() {
    let ghost =
        Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").expect("Failed to read ghost");

    // General ghost info
    assert_eq!(ghost.header().finish_time().minutes(), 1);
    assert_eq!(ghost.header().finish_time().seconds(), 3);
    assert_eq!(ghost.header().finish_time().milliseconds(), 904);
    assert_eq!(ghost.header().finish_time().to_string(), "01:03.904");
    assert_eq!(ghost.header().slot_id(), SlotId::LuigiCircuit);
    assert_eq!(ghost.header().combo().vehicle(), Vehicle::WarioBike);
    assert_eq!(ghost.header().combo().character(), Character::KingBoo);
    assert_eq!(ghost.header().date_set(), &Date::new(2025, 11, 12).unwrap());
    assert_eq!(ghost.header().controller(), Controller::Classic);
    assert!(ghost.header().is_compressed());
    assert_eq!(ghost.header().ghost_type(), GhostType::ExpertStaff);
    assert!(ghost.header().is_automatic_drift());
    assert_eq!(ghost.header().decompressed_input_data_length(), 1856);
    assert_eq!(ghost.header().lap_count(), 3);
    assert_eq!(ghost.header().lap_split_times()[0].to_string(), "00:25.540");
    assert_eq!(ghost.header().lap_split_times()[1].to_string(), "00:19.127");
    assert_eq!(ghost.header().lap_split_times()[2].to_string(), "00:19.237");
    assert_eq!(ghost.header().location().country(), Country::NotSet);
    assert_eq!(ghost.header().location().subregion(), Subregion::NotSet);

    // Mii Data
    assert!(!ghost.header().mii().is_girl());
    assert_eq!(ghost.header().mii().birthday().month(), Some(1));
    assert_eq!(ghost.header().mii().birthday().day(), Some(1));
    assert_eq!(
        ghost.header().mii().favorite_color(),
        FavoriteColor::ForestGreen
    );
    assert_eq!(ghost.header().mii().name(), "JC");
    assert_eq!(ghost.header().mii().build().height(), 127);
    assert_eq!(ghost.header().mii().build().weight(), 127);

    assert_eq!(ghost.header().mii().mii_id(), 0x893EF2FB);
    assert_eq!(ghost.header().mii().system_id(), 0x689EC992);

    assert_eq!(ghost.header().mii().head().shape(), HeadShape::Large);
    assert_eq!(ghost.header().mii().head().skin_tone(), SkinTone::Natural);
    assert_eq!(
        ghost.header().mii().head().face_features(),
        FaceFeatures::None
    );

    assert!(!ghost.header().mii().is_mingle_enabled());
    assert!(!ghost.header().mii().downloaded());

    assert_eq!(
        ghost.header().mii().hair().hair_type(),
        HairType::NormalLong
    );
    assert_eq!(
        ghost.header().mii().hair().hair_color(),
        HairColor::PhilippineBrown
    );
    assert!(!ghost.header().mii().hair().is_flipped());

    assert_eq!(
        ghost.header().mii().eyebrows().eyebrow_type(),
        EyebrowType::None
    );
    assert_eq!(ghost.header().mii().eyebrows().rotation(), 5);
    assert_eq!(
        ghost.header().mii().eyebrows().eyebrow_color(),
        HairColor::Chocolate
    );
    assert_eq!(ghost.header().mii().eyebrows().size(), 4);
    assert_eq!(ghost.header().mii().eyebrows().y(), 10);
    assert_eq!(ghost.header().mii().eyebrows().x(), 2);

    assert_eq!(ghost.header().mii().eyes().eye_type(), EyeType::DotAngry);
    assert_eq!(ghost.header().mii().eyes().rotation(), 4);
    assert_eq!(ghost.header().mii().eyes().y(), 9);
    assert_eq!(ghost.header().mii().eyes().eye_color(), EyeColor::Black);
    assert_eq!(ghost.header().mii().eyes().size(), 6);
    assert_eq!(ghost.header().mii().eyes().x(), 1);

    assert_eq!(ghost.header().mii().nose().nose_type(), NoseType::Dot);
    assert_eq!(ghost.header().mii().nose().size(), 0);
    assert_eq!(ghost.header().mii().nose().y(), 8);

    assert_eq!(ghost.header().mii().lips().lips_type(), LipsType::WaveAngry);
    assert_eq!(ghost.header().mii().lips().lips_color(), LipsColor::Orange);
    assert_eq!(ghost.header().mii().lips().size(), 7);
    assert_eq!(ghost.header().mii().lips().y(), 6);

    assert_eq!(
        ghost.header().mii().glasses().glasses_type(),
        GlassesType::None
    );
    assert_eq!(
        ghost.header().mii().glasses().glasses_color(),
        GlassesColor::Black
    );
    assert_eq!(ghost.header().mii().glasses().size(), 4);
    assert_eq!(ghost.header().mii().glasses().y(), 10);

    assert_eq!(
        ghost.header().mii().facial_hair().mustache_type(),
        MustacheType::None
    );
    assert_eq!(
        ghost.header().mii().facial_hair().beard_type(),
        BeardType::None
    );
    assert_eq!(ghost.header().mii().facial_hair().color(), HairColor::Black);
    assert_eq!(ghost.header().mii().facial_hair().mustache_size(), 4);
    assert_eq!(ghost.header().mii().facial_hair().mustache_y(), 10);

    assert!(!ghost.header().mii().mole().has_mole());
    assert_eq!(ghost.header().mii().mole().size(), 4);
    assert_eq!(ghost.header().mii().mole().y(), 20);
    assert_eq!(ghost.header().mii().mole().x(), 2);

    assert_eq!(ghost.header().mii().creator_name(), "JC");

    assert_eq!(ghost.header().mii_crc16(), 0x06F4);
    assert!(ghost.header().verify_mii_crc16());

    // Input data
    assert_eq!(ghost.input_data().face_input_count(), 0x18);
    assert_eq!(ghost.input_data().stick_input_count(), 0x037B);
    assert_eq!(ghost.input_data().dpad_input_count(), 0x09);
    assert_eq!(ghost.input_data().inputs().len(), 907);
    assert_eq!(ghost.input_data().face_inputs().len(), 12);
    assert_eq!(ghost.input_data().stick_inputs().len(), 891);
    assert_eq!(ghost.input_data().dpad_inputs().len(), 9);

    assert!(!ghost.input_data().contains_illegal_brake_or_drift_inputs());
    assert!(
        !ghost
            .input_data()
            .contains_illegal_stick_inputs(ghost.header().controller())
    );

    // CTGP Metadata
    assert!(ghost.ctgp_footer().is_some());
}

#[test]
fn write_to_ghost() {
    let mut ghost =
        Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").expect("Failed to read ghost");

    // General ghost info
    assert_eq!(ghost.header().finish_time().minutes(), 1);
    assert_eq!(ghost.header().finish_time().seconds(), 3);
    assert_eq!(ghost.header().finish_time().milliseconds(), 904);
    assert_eq!(ghost.header().finish_time().to_string(), "01:03.904");
    assert_eq!(ghost.header().slot_id(), SlotId::LuigiCircuit);
    assert_eq!(ghost.header().combo().vehicle(), Vehicle::WarioBike);
    assert_eq!(ghost.header().combo().character(), Character::KingBoo);
    assert_eq!(ghost.header().date_set(), &Date::new(2025, 11, 12).unwrap());
    assert_eq!(ghost.header().controller(), Controller::Classic);
    assert!(ghost.header().is_compressed());
    assert_eq!(ghost.header().ghost_type(), GhostType::ExpertStaff);
    assert!(ghost.header().is_automatic_drift());
    assert_eq!(ghost.header().decompressed_input_data_length(), 1856);
    assert_eq!(ghost.header().lap_count(), 3);
    assert_eq!(ghost.header().lap_split_times()[0].to_string(), "00:25.540");
    assert_eq!(ghost.header().lap_split_times()[1].to_string(), "00:19.127");
    assert_eq!(ghost.header().lap_split_times()[2].to_string(), "00:19.237");
    assert_eq!(ghost.header().location().country(), Country::NotSet);
    assert_eq!(ghost.header().location().subregion(), Subregion::NotSet);

    // Mii Data
    assert!(!ghost.header().mii().is_girl());
    assert_eq!(ghost.header().mii().birthday().month(), Some(1));
    assert_eq!(ghost.header().mii().birthday().day(), Some(1));
    assert_eq!(
        ghost.header().mii().favorite_color(),
        FavoriteColor::ForestGreen
    );
    assert_eq!(ghost.header().mii().name(), "JC");
    assert_eq!(ghost.header().mii().build().height(), 127);
    assert_eq!(ghost.header().mii().build().weight(), 127);

    assert_eq!(ghost.header().mii().mii_id(), 0x893EF2FB);
    assert_eq!(ghost.header().mii().system_id(), 0x689EC992);

    assert_eq!(ghost.header().mii().head().shape(), HeadShape::Large);
    assert_eq!(ghost.header().mii().head().skin_tone(), SkinTone::Natural);
    assert_eq!(
        ghost.header().mii().head().face_features(),
        FaceFeatures::None
    );

    assert!(!ghost.header().mii().is_mingle_enabled());
    assert!(!ghost.header().mii().downloaded());

    assert_eq!(
        ghost.header().mii().hair().hair_type(),
        HairType::NormalLong
    );
    assert_eq!(
        ghost.header().mii().hair().hair_color(),
        HairColor::PhilippineBrown
    );
    assert!(!ghost.header().mii().hair().is_flipped());

    assert_eq!(
        ghost.header().mii().eyebrows().eyebrow_type(),
        EyebrowType::None
    );
    assert_eq!(ghost.header().mii().eyebrows().rotation(), 5);
    assert_eq!(
        ghost.header().mii().eyebrows().eyebrow_color(),
        HairColor::Chocolate
    );
    assert_eq!(ghost.header().mii().eyebrows().size(), 4);
    assert_eq!(ghost.header().mii().eyebrows().y(), 10);
    assert_eq!(ghost.header().mii().eyebrows().x(), 2);

    assert_eq!(ghost.header().mii().eyes().eye_type(), EyeType::DotAngry);
    assert_eq!(ghost.header().mii().eyes().rotation(), 4);
    assert_eq!(ghost.header().mii().eyes().y(), 9);
    assert_eq!(ghost.header().mii().eyes().eye_color(), EyeColor::Black);
    assert_eq!(ghost.header().mii().eyes().size(), 6);
    assert_eq!(ghost.header().mii().eyes().x(), 1);

    assert_eq!(ghost.header().mii().nose().nose_type(), NoseType::Dot);
    assert_eq!(ghost.header().mii().nose().size(), 0);
    assert_eq!(ghost.header().mii().nose().y(), 8);

    assert_eq!(ghost.header().mii().lips().lips_type(), LipsType::WaveAngry);
    assert_eq!(ghost.header().mii().lips().lips_color(), LipsColor::Orange);
    assert_eq!(ghost.header().mii().lips().size(), 7);
    assert_eq!(ghost.header().mii().lips().y(), 6);

    assert_eq!(
        ghost.header().mii().glasses().glasses_type(),
        GlassesType::None
    );
    assert_eq!(
        ghost.header().mii().glasses().glasses_color(),
        GlassesColor::Black
    );
    assert_eq!(ghost.header().mii().glasses().size(), 4);
    assert_eq!(ghost.header().mii().glasses().y(), 10);

    assert_eq!(
        ghost.header().mii().facial_hair().mustache_type(),
        MustacheType::None
    );
    assert_eq!(
        ghost.header().mii().facial_hair().beard_type(),
        BeardType::None
    );
    assert_eq!(ghost.header().mii().facial_hair().color(), HairColor::Black);
    assert_eq!(ghost.header().mii().facial_hair().mustache_size(), 4);
    assert_eq!(ghost.header().mii().facial_hair().mustache_y(), 10);

    assert!(!ghost.header().mii().mole().has_mole());
    assert_eq!(ghost.header().mii().mole().size(), 4);
    assert_eq!(ghost.header().mii().mole().y(), 20);
    assert_eq!(ghost.header().mii().mole().x(), 2);

    assert_eq!(ghost.header().mii().creator_name(), "JC");

    assert_eq!(ghost.header().mii_crc16(), 0x06F4);
    assert!(ghost.header().verify_mii_crc16());

    // modify fields
    ghost
        .header_mut()
        .set_finish_time(InGameTime::new(1, 37, 999));
    ghost.header_mut().set_slot_id(SlotId::DryDryRuins);
    ghost
        .header_mut()
        .set_combo(Combo::new(Vehicle::BlueFalcon, Character::Toad).unwrap());
    ghost
        .header_mut()
        .set_date_set(Date::new(2018, 12, 25).unwrap());
    ghost.header_mut().set_controller(Controller::Gamecube);
    ghost.decompress_input_data();
    ghost.header_mut().set_ghost_type(GhostType::Friend23);
    ghost.header_mut().set_automatic_drift(false);
    ghost
        .header_mut()
        .set_lap_split_time(0, InGameTime::new(0, 6, 741));
    ghost
        .header_mut()
        .set_lap_split_time(1, InGameTime::new(0, 42, 069));
    ghost
        .header_mut()
        .set_lap_split_time(2, InGameTime::new(0, 21, 910));
    ghost.header_mut().set_location(
        Location::find(
            u8::from(Country::UnitedStates),
            u8::from(Subregion::UnitedStates(UnitedStatesSubregion::California)),
            None,
        )
        .unwrap(),
    );

    let mii = ghost.header_mut().mii_mut();

    mii.set_is_girl(true);
    mii.set_birthday(Birthday::new(4, 20).unwrap());
    mii.set_favorite_color(FavoriteColor::Red);
    mii.set_name("DUMBASS");
    mii.set_build(Build::new(100, 50).unwrap());
    mii.set_mii_type(MiiType::Foreign);
    mii.set_creation_date(
        NaiveDateTime::parse_from_str("2023-10-27 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
    );
    mii.set_system_id(0x453AE846);
    mii.set_head(Head::new(
        HeadShape::Rounded,
        SkinTone::Honey,
        FaceFeatures::Freckles,
    ));
    mii.set_mingle_enabled(true);
    mii.set_downloaded(true);
    mii.set_hair(Hair::new(HairType::NormalMedium, HairColor::Black, true));
    mii.set_eyebrows(
        Eyebrows::new(4, 3, 1, 9, HairColor::Black, EyebrowType::RoundedMedium).unwrap(),
    );
    mii.set_eyes(Eyes::new(3, 5, 0, 8, EyeColor::Brown, EyeType::Normal).unwrap());
    mii.set_nose(Nose::new(7, 1, NoseType::Normal).unwrap());
    mii.set_lips(Lips::new(5, 6, LipsType::Neutral, LipsColor::Red).unwrap());
    mii.set_glasses(Glasses::new(9, 3, GlassesType::Square, GlassesColor::Brown).unwrap());
    mii.set_facial_hair(
        FacialHair::new(
            BeardType::GoateeLong,
            MustacheType::Pencil,
            HairColor::Walnut,
            3,
            9,
        )
        .unwrap(),
    );
    mii.set_mole(Mole::new(true, 3, 19, 3).unwrap());

    mii.set_creator_name("IDIOT");

    // new ghost asserts
    ghost.update_raw_data().unwrap();
    let mut ghost = Ghost::new(&ghost.raw_data).unwrap();

    // General ghost info
    assert_eq!(ghost.header().finish_time().minutes(), 1);
    assert_eq!(ghost.header().finish_time().seconds(), 37);
    assert_eq!(ghost.header().finish_time().milliseconds(), 999);
    assert_eq!(ghost.header().finish_time().to_string(), "01:37.999");
    assert_eq!(ghost.header().slot_id(), SlotId::DryDryRuins);
    assert_eq!(ghost.header().combo().vehicle(), Vehicle::BlueFalcon);
    assert_eq!(ghost.header().combo().character(), Character::Toad);
    assert_eq!(ghost.header().date_set(), &Date::new(2018, 12, 25).unwrap());
    assert_eq!(ghost.header().controller(), Controller::Gamecube);
    assert!(!ghost.header().is_compressed());
    assert_eq!(ghost.header().ghost_type(), GhostType::Friend23);
    assert!(!ghost.header().is_automatic_drift());
    assert_eq!(ghost.header().decompressed_input_data_length(), 1856);
    assert_eq!(ghost.header().lap_count(), 3);
    assert_eq!(ghost.header().lap_split_times()[0].to_string(), "00:06.741");
    assert_eq!(ghost.header().lap_split_times()[1].to_string(), "00:42.069");
    assert_eq!(ghost.header().lap_split_times()[2].to_string(), "00:21.910");
    assert_eq!(ghost.header().location().country(), Country::UnitedStates);
    assert_eq!(
        ghost.header().location().subregion(),
        Subregion::UnitedStates(UnitedStatesSubregion::California)
    );

    // Mii Data
    assert!(ghost.header().mii().is_girl());
    assert_eq!(ghost.header().mii().birthday().month(), Some(4));
    assert_eq!(ghost.header().mii().birthday().day(), Some(20));
    assert_eq!(ghost.header().mii().favorite_color(), FavoriteColor::Red);
    assert_eq!(ghost.header().mii().name(), "DUMBASS");
    assert_eq!(ghost.header().mii().build().height(), 100);
    assert_eq!(ghost.header().mii().build().weight(), 50);

    // assert_eq!(ghost.header().mii().mii_id(), 0xB00B1355);
    assert_eq!(ghost.header().mii().mii_type(), MiiType::Foreign);
    assert_eq!(
        ghost.header().mii().creation_date(),
        NaiveDateTime::parse_from_str("2023-10-27 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap()
    );
    assert_eq!(ghost.header().mii().system_id(), 0x453AE846);

    assert_eq!(ghost.header().mii().head().shape(), HeadShape::Rounded);
    assert_eq!(ghost.header().mii().head().skin_tone(), SkinTone::Honey);
    assert_eq!(
        ghost.header().mii().head().face_features(),
        FaceFeatures::Freckles
    );

    assert!(ghost.header().mii().is_mingle_enabled());
    assert!(ghost.header().mii().downloaded());

    assert_eq!(
        ghost.header().mii().hair().hair_type(),
        HairType::NormalMedium
    );
    assert_eq!(ghost.header().mii().hair().hair_color(), HairColor::Black);
    assert!(ghost.header().mii().hair().is_flipped());

    assert_eq!(
        ghost.header().mii().eyebrows().eyebrow_type(),
        EyebrowType::RoundedMedium
    );
    assert_eq!(ghost.header().mii().eyebrows().rotation(), 4);
    assert_eq!(
        ghost.header().mii().eyebrows().eyebrow_color(),
        HairColor::Black
    );
    assert_eq!(ghost.header().mii().eyebrows().size(), 3);
    assert_eq!(ghost.header().mii().eyebrows().y(), 9);
    assert_eq!(ghost.header().mii().eyebrows().x(), 1);

    assert_eq!(ghost.header().mii().eyes().eye_type(), EyeType::Normal);
    assert_eq!(ghost.header().mii().eyes().rotation(), 3);
    assert_eq!(ghost.header().mii().eyes().y(), 8);
    assert_eq!(ghost.header().mii().eyes().eye_color(), EyeColor::Brown);
    assert_eq!(ghost.header().mii().eyes().size(), 5);
    assert_eq!(ghost.header().mii().eyes().x(), 0);

    assert_eq!(ghost.header().mii().nose().nose_type(), NoseType::Normal);
    assert_eq!(ghost.header().mii().nose().size(), 1);
    assert_eq!(ghost.header().mii().nose().y(), 7);

    assert_eq!(ghost.header().mii().lips().lips_type(), LipsType::Neutral);
    assert_eq!(ghost.header().mii().lips().lips_color(), LipsColor::Red);
    assert_eq!(ghost.header().mii().lips().size(), 6);
    assert_eq!(ghost.header().mii().lips().y(), 5);

    assert_eq!(
        ghost.header().mii().glasses().glasses_type(),
        GlassesType::Square
    );
    assert_eq!(
        ghost.header().mii().glasses().glasses_color(),
        GlassesColor::Brown
    );
    assert_eq!(ghost.header().mii().glasses().size(), 3);
    assert_eq!(ghost.header().mii().glasses().y(), 9);

    assert_eq!(
        ghost.header().mii().facial_hair().mustache_type(),
        MustacheType::Pencil
    );
    assert_eq!(
        ghost.header().mii().facial_hair().beard_type(),
        BeardType::GoateeLong
    );
    assert_eq!(
        ghost.header().mii().facial_hair().color(),
        HairColor::Walnut
    );
    assert_eq!(ghost.header().mii().facial_hair().mustache_size(), 3);
    assert_eq!(ghost.header().mii().facial_hair().mustache_y(), 9);

    assert!(ghost.header().mii().mole().has_mole());
    assert_eq!(ghost.header().mii().mole().size(), 3);
    assert_eq!(ghost.header().mii().mole().y(), 19);
    assert_eq!(ghost.header().mii().mole().x(), 3);

    assert_eq!(ghost.header().mii().creator_name(), "IDIOT");

    assert!(ghost.header().verify_mii_crc16());

    let _ = ghost
        .save_to_file("./test_ghosts/JC_transformed_ghost.rkg")
        .unwrap();
}

// This test requires a "ctgp_ghost_collection" folder not included in the rkg-utils repository, as it's 6.5k ghost files.
// Downloadable here: https://drive.google.com/file/d/1g-aY0mcBcMq9Zse0dkQEmZqHxV_UhmXM/view?usp=sharing
/*
#[test]
fn bulk_ghost_collection() {
    for entry in std::fs::read_dir("./test_ghosts/ctgp_ghost_collection").unwrap() {
        let ghost = Ghost::new_from_file(entry.as_ref().unwrap().path())
            .expect(format!("Failed on ghost {:?}", entry.as_ref().unwrap().file_name()).as_str());
        assert!(ghost.verify_base_crc32());
        assert!(ghost.verify_file_crc32());
    }
}
*/

#[test]
fn test_compare_saved_ghost() {
    let mut ghost1 =
        Ghost::new_from_file("./test_ghosts/JC_LC_Compressed.rkg").expect("Failed to read ghost");

    ghost1
        .save_to_file("./test_ghosts/JC_LC_Compressed_Resave.rkg")
        .expect("Failed to save ghost");

    let ghost2 = Ghost::new_from_file("./test_ghosts/JC_LC_Compressed_Resave.rkg")
        .expect("Failed to read ghost");

    assert_eq!(ghost1.header().raw_data(), ghost2.header().raw_data());
    assert_eq!(
        ghost1.input_data().raw_data(),
        ghost2.input_data().raw_data()
    );
    assert_eq!(ghost1.file_crc32(), ghost2.file_crc32());
    assert_eq!(
        ghost1.ctgp_footer().as_ref().unwrap().raw_data(),
        ghost2.ctgp_footer().as_ref().unwrap().raw_data()
    );
}

#[test]
fn test_sp_footer() {
    let ghost = Ghost::new_from_file("./test_ghosts/spv5.rkg").unwrap();

    let sp_footer = ghost.sp_footer().unwrap();

    println!("SP footer version: {}", sp_footer.footer_version());
    print!("Possible MKW-SP release versions: ");
    if let Some(sp_versions) = sp_footer.possible_sp_versions() {
        for version in sp_versions {
            print!("{}, ", version)
        }
        println!();
    } else {
        println!("Unknown")
    }

    print!("Track SHA1: ");
    for byte in sp_footer.track_sha1().iter() {
        print!("{:02X}", *byte);
    }
    println!();

    for (index, time) in sp_footer.exact_lap_times().iter().enumerate() {
        println!("Lap {}: {}", index + 1, time);
    }
    println!("Total: {}", sp_footer.exact_finish_time());
    println!("Has speed mod? {}", sp_footer.has_speed_mod());
    println!("Has ultra shortcut? {}", sp_footer.has_ultra_shortcut());
    println!(
        "Has horizontal wall glitch? {}",
        sp_footer.has_horizontal_wall_glitch()
    );
    println!("Has wallride? {}", sp_footer.has_wallride());

    if let Some(shroomstrat_string) = sp_footer.shroomstrat_string() {
        println!("Shroomstrat: {}", shroomstrat_string);
    }

    if let Some(is_vanilla) = sp_footer.is_vanilla_mode_enabled() {
        println!("Vanilla mode enabled? {}", is_vanilla);
    }

    if let Some(simplified_controls) = sp_footer.has_simplified_controls() {
        println!("Has simplified controls? {}", simplified_controls);
    }

    if let Some(set_in_mirror) = sp_footer.set_in_mirror() {
        println!("Set in mirror mode? {}", set_in_mirror);
    }

    assert!(ghost.verify_file_crc32());
}
