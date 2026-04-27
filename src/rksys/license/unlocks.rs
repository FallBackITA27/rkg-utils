pub struct Unlocks {
    // CC Completion
    karts_in_100cc_completed: bool,
    bikes_in_50cc_completed: bool,

    // Cup Completion
    leaf_cup_completed: CupCompletion,
    banana_cup_completed: CupCompletion,
    star_cup_completed: CupCompletion,
    flower_cup_completed: CupCompletion,

    // Character Unlocks
    mii_a_unlocked: bool,
    mii_b_unlocked: bool,
    rosalina_unlocked: bool,
    funky_kong_unlocked: bool,
    king_boo_unlocked: bool,
    dry_bowser_unlocked: bool,
    birdo_unlocked: bool,
    daisy_unlocked: bool,
    bowser_jr_unlocked: bool,
    diddy_kong_unlocked: bool,
    baby_luigi_unlocked: bool,
    baby_daisy_unlocked: bool,
    toadette_unlocked: bool,
    dry_bones_unlocked: bool,

    // GP Class Completion
    mario_peach_endscreen: bool,
    all_characters_endscreen: bool,
    mirror_mode_completed: bool,

    // Vehicle Unlocks
    phantom_unlocked: bool,
    spear_unlocked: bool,
    shooting_star_unlocked: bool,
    dolphin_dasher_unlocked: bool,
    sneakster_unlocked: bool,
    zip_zip_unlocked: bool,
    jet_bubble_unlocked: bool,
    magikruiser_unlocked: bool,
    quacker_unlocked: bool,
    honeycoupe_unlocked: bool,
    jetsetter_unlocked: bool,
    piranha_prowler_unlocked: bool,
    sprinter_unlocked: bool,
    daytripper_unlocked: bool,
    super_blooper_unlocked: bool,
    blue_falcon_unlocked: bool,
    tiny_titan_unlocked: bool,
    cheep_charger_unlocked: bool,
}

struct CupCompletion {
    in_mirror: bool,
    in_150cc: bool,
    in_100cc: bool,
    in_50cc: bool,
}

impl From<u64> for Unlocks {
    fn from(value: u64) -> Self {
        Self {
            karts_in_100cc_completed: (value & (1 << 63)) > 0,
            bikes_in_50cc_completed: (value & (1 << 62)) > 0,
            leaf_cup_completed: CupCompletion {
                in_mirror: (value & (1 << 61)) > 0,
                in_150cc: (value & (1 << 60)) > 0,
                in_100cc: (value & (1 << 59)) > 0,
                in_50cc: (value & (1 << 58)) > 0,
            },
            banana_cup_completed: CupCompletion {
                in_mirror: (value & (1 << 57)) > 0,
                in_150cc: (value & (1 << 56)) > 0,
                in_100cc: (value & (1 << 55)) > 0,
                in_50cc: (value & (1 << 54)) > 0,
            },
            star_cup_completed: CupCompletion {
                in_mirror: (value & (1 << 53)) > 0,
                in_150cc: (value & (1 << 52)) > 0,
                in_100cc: (value & (1 << 51)) > 0,
                in_50cc: (value & (1 << 50)) > 0,
            },
            flower_cup_completed: CupCompletion {
                in_mirror: (value & (1 << 49)) > 0,
                in_150cc: (value & (1 << 48)) > 0,
                in_100cc: (value & (1 << 47)) > 0,
                in_50cc: (value & (1 << 46)) > 0,
            },
            mii_a_unlocked: (value & (1 << 45)) > 0,
            mii_b_unlocked: (value & (1 << 44)) > 0,
            rosalina_unlocked: (value & (1 << 43)) > 0,
            funky_kong_unlocked: (value & (1 << 42)) > 0,
            king_boo_unlocked: (value & (1 << 41)) > 0,
            dry_bowser_unlocked: (value & (1 << 40)) > 0,
            birdo_unlocked: (value & (1 << 39)) > 0,
            daisy_unlocked: (value & (1 << 38)) > 0,
            bowser_jr_unlocked: (value & (1 << 37)) > 0,
            diddy_kong_unlocked: (value & (1 << 36)) > 0,
            baby_luigi_unlocked: (value & (1 << 35)) > 0,
            baby_daisy_unlocked: (value & (1 << 34)) > 0,
            toadette_unlocked: (value & (1 << 33)) > 0,
            dry_bones_unlocked: (value & (1 << 32)) > 0,
            mario_peach_endscreen: (value & (1 << 25)) > 0,
            all_characters_endscreen: (value & (1 << 24)) > 0,
            mirror_mode_completed: (value & (1 << 20)) > 0,
            phantom_unlocked: (value & (1 << 19)) > 0,
            spear_unlocked: (value & (1 << 18)) > 0,
            shooting_star_unlocked: (value & (1 << 17)) > 0,
            dolphin_dasher_unlocked: (value & (1 << 16)) > 0,
            sneakster_unlocked: (value & (1 << 15)) > 0,
            zip_zip_unlocked: (value & (1 << 14)) > 0,
            jet_bubble_unlocked: (value & (1 << 13)) > 0,
            magikruiser_unlocked: (value & (1 << 12)) > 0,
            quacker_unlocked: (value & (1 << 11)) > 0,
            honeycoupe_unlocked: (value & (1 << 10)) > 0,
            jetsetter_unlocked: (value & (1 << 9)) > 0,
            piranha_prowler_unlocked: (value & (1 << 8)) > 0,
            sprinter_unlocked: (value & (1 << 7)) > 0,
            daytripper_unlocked: (value & (1 << 6)) > 0,
            super_blooper_unlocked: (value & (1 << 5)) > 0,
            blue_falcon_unlocked: (value & (1 << 4)) > 0,
            tiny_titan_unlocked: (value & (1 << 3)) > 0,
            cheep_charger_unlocked: (value & (1 << 2)) > 0,
        }
    }
}

impl From<[u8; 8]> for Unlocks {
    fn from(value: [u8; 8]) -> Self {
        Self::from(u64::from_be_bytes(value))
    }
}

impl Unlocks {
    pub const fn is_karts_in_100cc_completed(&self) -> bool {
        self.karts_in_100cc_completed
    }
    pub const fn is_bikes_in_50cc_completed(&self) -> bool {
        self.bikes_in_50cc_completed
    }
    pub const fn is_leaf_cup_completed(&self) -> bool {
        self.leaf_cup_completed.in_mirror
            && self.leaf_cup_completed.in_150cc
            && self.leaf_cup_completed.in_100cc
            && self.leaf_cup_completed.in_50cc
    }
    pub const fn is_leaf_cup_completed_in_mirror(&self) -> bool {
        self.leaf_cup_completed.in_mirror
    }
    pub const fn is_leaf_cup_completed_in_150cc(&self) -> bool {
        self.leaf_cup_completed.in_150cc
    }
    pub const fn is_leaf_cup_completed_in_100cc(&self) -> bool {
        self.leaf_cup_completed.in_100cc
    }
    pub const fn is_leaf_cup_completed_in_50cc(&self) -> bool {
        self.leaf_cup_completed.in_50cc
    }
    pub const fn is_banana_cup_completed(&self) -> bool {
        self.banana_cup_completed.in_mirror
            && self.banana_cup_completed.in_150cc
            && self.banana_cup_completed.in_100cc
            && self.banana_cup_completed.in_50cc
    }
    pub const fn is_banana_cup_completed_in_mirror(&self) -> bool {
        self.banana_cup_completed.in_mirror
    }
    pub const fn is_banana_cup_completed_in_150cc(&self) -> bool {
        self.banana_cup_completed.in_150cc
    }
    pub const fn is_banana_cup_completed_in_100cc(&self) -> bool {
        self.banana_cup_completed.in_100cc
    }
    pub const fn is_banana_cup_completed_in_50cc(&self) -> bool {
        self.banana_cup_completed.in_50cc
    }
    pub const fn is_star_cup_completed(&self) -> bool {
        self.star_cup_completed.in_mirror
            && self.star_cup_completed.in_150cc
            && self.star_cup_completed.in_100cc
            && self.star_cup_completed.in_50cc
    }
    pub const fn is_star_cup_completed_in_mirror(&self) -> bool {
        self.star_cup_completed.in_mirror
    }
    pub const fn is_star_cup_completed_in_150cc(&self) -> bool {
        self.star_cup_completed.in_150cc
    }
    pub const fn is_star_cup_completed_in_100cc(&self) -> bool {
        self.star_cup_completed.in_100cc
    }
    pub const fn is_star_cup_completed_in_50cc(&self) -> bool {
        self.star_cup_completed.in_50cc
    }
    pub const fn is_flower_cup_completed(&self) -> bool {
        self.flower_cup_completed.in_mirror
            && self.flower_cup_completed.in_150cc
            && self.flower_cup_completed.in_100cc
            && self.flower_cup_completed.in_50cc
    }
    pub const fn is_flower_cup_completed_in_mirror(&self) -> bool {
        self.flower_cup_completed.in_mirror
    }
    pub const fn is_flower_cup_completed_in_150cc(&self) -> bool {
        self.flower_cup_completed.in_150cc
    }
    pub const fn is_flower_cup_completed_in_100cc(&self) -> bool {
        self.flower_cup_completed.in_100cc
    }
    pub const fn is_flower_cup_completed_in_50cc(&self) -> bool {
        self.flower_cup_completed.in_50cc
    }
    pub const fn is_mii_a_unlocked(&self) -> bool {
        self.mii_a_unlocked
    }
    pub const fn is_mii_b_unlocked(&self) -> bool {
        self.mii_b_unlocked
    }
    pub const fn is_rosalina_unlocked(&self) -> bool {
        self.rosalina_unlocked
    }
    pub const fn is_funky_kong_unlocked(&self) -> bool {
        self.funky_kong_unlocked
    }
    pub const fn is_king_boo_unlocked(&self) -> bool {
        self.king_boo_unlocked
    }
    pub const fn is_dry_bowser_unlocked(&self) -> bool {
        self.dry_bowser_unlocked
    }
    pub const fn is_birdo_unlocked(&self) -> bool {
        self.birdo_unlocked
    }
    pub const fn is_daisy_unlocked(&self) -> bool {
        self.daisy_unlocked
    }
    pub const fn is_bowser_jr_unlocked(&self) -> bool {
        self.bowser_jr_unlocked
    }
    pub const fn is_diddy_kong_unlocked(&self) -> bool {
        self.diddy_kong_unlocked
    }
    pub const fn is_baby_luigi_unlocked(&self) -> bool {
        self.baby_luigi_unlocked
    }
    pub const fn is_baby_daisy_unlocked(&self) -> bool {
        self.baby_daisy_unlocked
    }
    pub const fn is_toadette_unlocked(&self) -> bool {
        self.toadette_unlocked
    }
    pub const fn is_dry_bones_unlocked(&self) -> bool {
        self.dry_bones_unlocked
    }
    pub const fn is_mario_peach_endscreen(&self) -> bool {
        self.mario_peach_endscreen
    }
    pub const fn is_all_characters_endscreen(&self) -> bool {
        self.all_characters_endscreen
    }
    pub const fn is_mirror_mode_completed(&self) -> bool {
        self.mirror_mode_completed
    }
    pub const fn is_phantom_unlocked(&self) -> bool {
        self.phantom_unlocked
    }
    pub const fn is_spear_unlocked(&self) -> bool {
        self.spear_unlocked
    }
    pub const fn is_shooting_star_unlocked(&self) -> bool {
        self.shooting_star_unlocked
    }
    pub const fn is_dolphin_dasher_unlocked(&self) -> bool {
        self.dolphin_dasher_unlocked
    }
    pub const fn is_sneakster_unlocked(&self) -> bool {
        self.sneakster_unlocked
    }
    pub const fn is_zip_zip_unlocked(&self) -> bool {
        self.zip_zip_unlocked
    }
    pub const fn is_jet_bubble_unlocked(&self) -> bool {
        self.jet_bubble_unlocked
    }
    pub const fn is_magikruiser_unlocked(&self) -> bool {
        self.magikruiser_unlocked
    }
    pub const fn is_quacker_unlocked(&self) -> bool {
        self.quacker_unlocked
    }
    pub const fn is_honeycoupe_unlocked(&self) -> bool {
        self.honeycoupe_unlocked
    }
    pub const fn is_jetsetter_unlocked(&self) -> bool {
        self.jetsetter_unlocked
    }
    pub const fn is_piranha_prowler_unlocked(&self) -> bool {
        self.piranha_prowler_unlocked
    }
    pub const fn is_sprinter_unlocked(&self) -> bool {
        self.sprinter_unlocked
    }
    pub const fn is_daytripper_unlocked(&self) -> bool {
        self.daytripper_unlocked
    }
    pub const fn is_super_blooper_unlocked(&self) -> bool {
        self.super_blooper_unlocked
    }
    pub const fn is_blue_falcon_unlocked(&self) -> bool {
        self.blue_falcon_unlocked
    }
    pub const fn is_tiny_titan_unlocked(&self) -> bool {
        self.tiny_titan_unlocked
    }
    pub const fn is_cheep_charger_unlocked(&self) -> bool {
        self.cheep_charger_unlocked
    }
}
