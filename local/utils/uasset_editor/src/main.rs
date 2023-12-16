use std::fs::File;
// use uasset::AssetHeader;
use ue_types::FPackageFileSummary;
use glob::glob;

mod deserialize;
use deserialize::FileDeserialize;


// use unreal_asset::{
//     Asset,
//     engine_version::EngineVersion,
// };

fn main() {
    println!("Hello, world!");

    for entry in glob("../../../../ExtractedClient/**/*.uasset").unwrap() {
        let path = entry.unwrap();
        let mut asset_file = File::open(path.display().to_string()).expect("Cannot open UAsset file");
        let file_header = FPackageFileSummary::parse_inline(&mut asset_file);

        println!("{:?}", file_header);
    }

    // let mut asset_file = File::open("../../tmp/Human/BriarwoodBard/BriarwoodBard_ClothingMaterial_ChrTBC.uasset").expect("Cannot open UAsset File");
    // let asset_bulk_file = File::open("../../tmp/Human/BriarwoodBard/BriarwoodBard_ClothingMaterial_ChrTBC.uexp").expect("Cannot open UExp file");
    // let asset = Asset::new(file, Some(bulk_file), EngineVersion::VER_UE4_23).unwrap();


    // asset_file.read(&mut file_header).expect("Could not read file header");

}
