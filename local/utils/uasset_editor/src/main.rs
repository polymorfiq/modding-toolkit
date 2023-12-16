use std::fs::File;
use uasset::AssetHeader;

use unreal_asset::{
    Asset,
    engine_version::EngineVersion,
};

fn main() {
    println!("Hello, world!");

    let file = File::open("../../tmp/Human/BriarwoodBard/BriarwoodBard_ClothingMaterial_ChrTBC.uasset").expect("Cannot open UAsset File");
    let bulk_file = File::open("../../tmp/Human/BriarwoodBard/BriarwoodBard_ClothingMaterial_ChrTBC.uexp").expect("Cannot open UExp file");
    let asset = Asset::new(file, Some(bulk_file), EngineVersion::VER_UE4_23).unwrap();
    println!("{:#?}", asset);
}
