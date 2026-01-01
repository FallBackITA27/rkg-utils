use std::fmt::Display;

#[derive(Clone, Copy, Debug)]
pub struct CTGPVersion {
    major: u8,
    minor: u8,
    revision: u16,
    subrevision: u8,
}

impl CTGPVersion {
    pub fn new(major: u8, minor: u8, revision: u16, subrevision: u8) -> Self {
        Self {
            major,
            minor,
            revision,
            subrevision,
        }
    }

    pub fn from(bytes: &[u8]) -> Option<Vec<Self>> {
        let mut possible_versions = Vec::new();
        match bytes {
            &[0x01, 0x03, 0x01, 0x42] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1044, 1));
            }

            &[0x01, 0x03, 0x01, 0x44] | &[0x01, 0x03, 0x01, 0x46] | &[0x01, 0x03, 0x01, 0x48] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1046, 5));
            }

            &[0x01, 0x03, 0x01, 0x50] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1048, 5));
                possible_versions.push(CTGPVersion::new(1, 3, 1050, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1054, 5));
            }

            &[0x01, 0x03, 0x01, 0x78] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1058, 6));
            }

            &[0x01, 0x03, 0x01, 0x80] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1060, 6));
            }

            &[0x01, 0x03, 0x01, 0x82] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1062, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1062, 5));
            }

            &[0x01, 0x03, 0x01, 0x92] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1064, 6));
            }

            &[0x01, 0x03, 0x02, 0x00] => {
                for subrevision in 5..=8 {
                    possible_versions.push(CTGPVersion::new(1, 3, 1066, subrevision));
                }
            }

            &[0x01, 0x03, 0x02, 0x02] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1066, 9));
            }

            &[0x01, 0x03, 0x02, 0x16] => {
                for subrevision in 10..=15 {
                    possible_versions.push(CTGPVersion::new(1, 3, 1066, subrevision));
                }
            }

            &[0x01, 0x03, 0x02, 0x18] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1068, 5));
                possible_versions.push(CTGPVersion::new(1, 3, 1070, 2));
            }

            &[0x01, 0x03, 0x02, 0x26] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1070, 2));
            }

            &[0x01, 0x03, 0x02, 0x28] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1074, 6));
            }

            &[0x01, 0x03, 0x02, 0x40] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1074, 9));
            }

            &[0x01, 0x03, 0x02, 0x42] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1074, 10));
            }

            &[0x01, 0x03, 0x02, 0x44] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1074, 11));
            }

            &[0x01, 0x03, 0x02, 0x52] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1076, 6));
            }

            &[0x01, 0x03, 0x02, 0x56] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1076, 9));
            }

            &[0x01, 0x03, 0x02, 0x78] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1076, 12));
                possible_versions.push(CTGPVersion::new(1, 3, 1076, 15));
            }

            &[0x01, 0x03, 0x02, 0x84] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1080, 8));
                possible_versions.push(CTGPVersion::new(1, 3, 1080, 9));
                possible_versions.push(CTGPVersion::new(1, 3, 1082, 3));
            }

            &[0x01, 0x03, 0x02, 0x86] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1084, 3));
                possible_versions.push(CTGPVersion::new(1, 3, 1084, 5));

                possible_versions.push(CTGPVersion::new(1, 3, 1086, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1088, 3));
                possible_versions.push(CTGPVersion::new(1, 3, 1090, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1092, 2));
            }

            &[0x01, 0x03, 0x02, 0x92] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1094, 4));
            }

            &[0x01, 0x03, 0x02, 0x96] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1094, 8));
            }

            &[0x01, 0x03, 0x03, 0x00] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1094, 12));
                possible_versions.push(CTGPVersion::new(1, 3, 1096, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1098, 3));
            }

            &[0x01, 0x03, 0x03, 0x12] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1100, 5));
                possible_versions.push(CTGPVersion::new(1, 3, 1100, 8));
                possible_versions.push(CTGPVersion::new(1, 3, 1100, 10));
                possible_versions.push(CTGPVersion::new(1, 3, 1102, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1104, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1106, 3));
            }

            &[0x01, 0x03, 0x03, 0x14] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1106, 6));
                possible_versions.push(CTGPVersion::new(1, 3, 1108, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1112, 4));
            }

            &[0x01, 0x03, 0x03, 0x38] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1112, 8));
                possible_versions.push(CTGPVersion::new(1, 3, 1112, 10));
                possible_versions.push(CTGPVersion::new(1, 3, 1112, 12));
            }

            &[0x01, 0x03, 0x03, 0x42] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1114, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1116, 2));
                possible_versions.push(CTGPVersion::new(1, 3, 1118, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1120, 5));
            }

            &[0x01, 0x03, 0x03, 0x44] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1120, 7));
            }

            &[0x01, 0x03, 0x03, 0x46] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1120, 10));
                possible_versions.push(CTGPVersion::new(1, 3, 1122, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1124, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1126, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1128, 3));
            }

            &[0x01, 0x03, 0x03, 0x72] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1130, 6));
            }

            &[0x01, 0x03, 0x03, 0x74] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1130, 8));
                possible_versions.push(CTGPVersion::new(1, 3, 1132, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1134, 2));
                possible_versions.push(CTGPVersion::new(1, 3, 1136, 4));
            }

            &[0x01, 0x03, 0x03, 0x78] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1138, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1138, 7));
                possible_versions.push(CTGPVersion::new(1, 3, 1140, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1142, 4));
                possible_versions.push(CTGPVersion::new(1, 3, 1144, 4));
            }

            &[0x01, 0x03, 0x03, 0x90] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1150, 9));
            }

            &[0x01, 0x03, 0x03, 0x92] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1150, 11));
                possible_versions.push(CTGPVersion::new(1, 3, 1154, 5));
            }

            &[0x01, 0x03, 0x03, 0x94] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1156, 6));
            }

            &[0x01, 0x03, 0x03, 0x96] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1156, 9));
                possible_versions.push(CTGPVersion::new(1, 3, 1158, 3));
            }

            &[0x01, 0x03, 0x03, 0x98] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1158, 5));
            }

            &[0x01, 0x03, 0x04, 0x00] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1158, 7));
                possible_versions.push(CTGPVersion::new(1, 3, 1160, 5));
                possible_versions.push(CTGPVersion::new(1, 3, 1162, 3));
                possible_versions.push(CTGPVersion::new(1, 3, 1166, 5));
                possible_versions.push(CTGPVersion::new(1, 3, 1170, 5));
            }

            &[0x01, 0x03, 0x04, 0x02] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1172, 5));
            }

            &[0x01, 0x03, 0x04, 0x04] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1172, 7));
            }

            &[0x01, 0x03, 0x04, 0x06] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1176, 6));
                possible_versions.push(CTGPVersion::new(1, 3, 1178, 3));
            }

            &[0x01, 0x03, 0x04, 0x08] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1178, 5));
            }

            &[0x01, 0x03, 0x04, 0x10] => {
                possible_versions.push(CTGPVersion::new(1, 3, 1178, 10));
                possible_versions.push(CTGPVersion::new(1, 3, 1180, 5));
                possible_versions.push(CTGPVersion::new(1, 3, 1180, 7));
                possible_versions.push(CTGPVersion::new(1, 3, 1182, 4));
            }

            _ => {
                return None;
            }
        }
        Some(possible_versions)
    }
}

impl Display for CTGPVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}.{:02}.{:04}-{}",
            self.major, self.minor, self.revision, self.subrevision
        )
    }
}
