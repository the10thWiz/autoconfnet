use std::io::*;
use std::process::*;

pub trait WriteProc {
    fn write_to_clip(self);
    fn write_to_minicom(self);
}

impl WriteProc for Vec<String> {
    fn write_to_clip(self) {
        // spawn process
        let p = std::process::Command::new("xclip").args(vec!["-selection", "c", "-i"]).stdin(Stdio::piped()).spawn().unwrap();
        let mut outstdin = p.stdin.unwrap();
        let mut writer = BufWriter::new(&mut outstdin);
        for l in self {
            writer.write(l.as_bytes()).unwrap();
            writer.write("\n".as_bytes()).unwrap();
        }
    }

    fn write_to_minicom(self) {
        // spawn process
        let p = std::process::Command::new("minicom").arg("cisco").stdin(Stdio::piped()).spawn().unwrap();
        let mut outstdin = p.stdin.unwrap();
        let mut writer = BufWriter::new(&mut outstdin);
        for l in self {
            writer.write(l.as_bytes()).unwrap();
            writer.write("\n".as_bytes()).unwrap();
        }
    }
}