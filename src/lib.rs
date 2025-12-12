pub mod ctgp_metadata;
pub mod header;
pub mod input_data;

/* 
 * TODO:
 * Unfinished/unimplemented functionality
 * ----------------------------------------------
 * Country ID enum
 * State ID enum
 * Location ID enum
 * Read CTGP pause times
 * Handle older CTGP footer versions
 * Create Ghost struct that brings everything together
 * Add CRC validation functions
 * Be able to modify variables in ghost files
 */
#[cfg(test)]
mod tests;
