use std::fs::File;
use std::collections::HashMap;
use std::path::Path;
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Font{
    pub letters:HashMap<char, [[bool; 6]; 5]>,
}
impl Font{
    pub fn from_file<P: AsRef<Path>>(path: P)->Font{
        let file = File::open(path).unwrap();
        serde_json::from_reader(file).unwrap()
    }
}