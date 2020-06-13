use std::fs::File;
use std::io::Read;

pub struct Reader {
    path: String,
}

impl Reader {
    pub fn new(path: String) -> Reader {
        Reader { path }
    }

    pub fn read(&self) -> Vec<u16> {
        let mut f = File::open(&self.path).expect("Couldn't read file");
        let mut bytes = vec![];
        f.read_to_end(&mut bytes).unwrap();

        let mut res: Vec<u16> = vec![];
        for i in (0..bytes.len()).step_by(2) {
            let msb = (bytes[i] as u16) << 8;
            let lsb = if i + 1 < bytes.len() {
                bytes[i + 1] as u16
            } else {
                0
            };

            res.push(msb | lsb);
        }

        res
    }
}
