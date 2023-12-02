use crate::*;
use simple_endian::u32le;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FUrl {
    // Size: 0x68
    pub protocol: FString,
    pub host: FString,
    pub port: u32le,
    pub valid: u32le,
    pub map: FString,
    pub redirect_url: FString,
    pub op: TArray<FString>,
    pub portal: FString
}

impl FUrl {
    pub fn protocol(&self) -> &FString { &self.protocol }
    pub fn host(&self) -> &FString { &self.host }
    pub fn port(&self) -> u32 { self.port.to_native() }
    pub fn valid(&self) -> u32 { self.valid.to_native() }
    pub fn map(&self) -> &FString { &self.map }
    pub fn redirect_url(&self) -> &FString { &self.redirect_url }
    pub fn op(&self) -> &TArray<FString> { &self.op }
    pub fn portal(&self) -> &FString { &self.portal }
    pub fn to_string(&self) -> String {
        format!("{:?}://{:?}:{:?}/{:?}?{:?}", self.protocol.len(), self.host.len(), self.port, self.map.len(), self.portal.len())
    }
}