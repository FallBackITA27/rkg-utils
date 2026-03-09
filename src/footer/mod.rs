use crate::footer::{ctgp_footer::CTGPFooter, sp_footer::SPFooter};

pub mod ctgp_footer;
pub mod sp_footer;

/// Footer Enum that allows for differentiating between different modded footers.
pub enum FooterType {
    CTGPFooter(CTGPFooter),
    SPFooter(SPFooter),
}

impl FooterType {
    pub fn is_ctgp(&self) -> bool {
        matches!(self, Self::CTGPFooter(_))
    }
    pub fn is_sp(&self) -> bool {
        matches!(self, Self::SPFooter(_))
    }

    /// Calls raw_data method of the footer types contained within
    pub fn raw_data(&self) -> &[u8] {
        match self {
            Self::CTGPFooter(footer) => footer.raw_data(),
            Self::SPFooter(footer) => footer.raw_data(),
        }
    }
}
