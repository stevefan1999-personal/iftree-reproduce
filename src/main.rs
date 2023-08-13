use std::collections::HashMap;

use derivative::Derivative;
use once_cell::sync::Lazy;

macro_rules! visit_base {
    ($length:literal, $($contents:item)*) => {
        pub mod base {
            $($contents)*
        }
    };
}

macro_rules! visit_folder {
    ($name:literal, $id:ident, $($contents:item)*) => {
        pub mod $id {
            $($contents)*
        }
    };
}    

macro_rules! visit_file {
    ($name:literal, $id:ident, $index:literal, $relative_path:literal, $absolute_path:literal) => {
        include_flate::flate!(pub static $id: str from $absolute_path);
    };
}

#[iftree::include_file_tree(
    "
root_folder_variable = 'OUT_DIR'
base_folder = 'merged/'
paths = '/**'

[[template]]
visit_base = 'visit_base'
visit_folder = 'visit_folder'
visit_file = 'visit_file'
"
)]
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Asset {
    relative_path:      &'static str,
    #[derivative(Debug = "ignore")]
    pub contents_bytes: &'static [u8],
}

impl Asset {}

pub static ASSET_MAP: Lazy<HashMap<&str, &Asset>> = Lazy::new(|| {
    ASSETS
        .iter()
        .map(|asset| (asset.relative_path, asset))
        .collect()
});

fn main() {
    println!("Hello, world!");
}
