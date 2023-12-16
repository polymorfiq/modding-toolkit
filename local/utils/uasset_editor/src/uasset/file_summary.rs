use simple_endian::*;

pub struct FPackageFileSummary {
    pub tag: u32,
    pub file_version_ue4: u32,
    pub file_version_licensee_ue4: u32,
}