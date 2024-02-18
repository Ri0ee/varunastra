use std::fs;
use std::io::Write;

use crate::{
    oodle::Oodle,
    vfs::{Bundle, IndexBundle},
};

mod oodle;
mod vfs;

fn main() {
    let oodle = Oodle::new(std::path::Path::new("resources/oo2core_8_win64.dll"));

    let raw_data =
        fs::read("D:/SteamLibrary/steamapps/common/Path of Exile/Bundles2/_.index.bin").unwrap();
    let index_bundle = Bundle::try_new(&raw_data, &oodle).unwrap();

    let output = index_bundle
        .blocks
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let index = IndexBundle::try_from(output.as_slice()).unwrap();
    let mut w = fs::File::create("./index.dat").unwrap();
    index.bundle_infos.into_iter().for_each(|bundle_info| {
        writeln!(w, "{}", bundle_info.name).unwrap();
    });
}
