
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

const INSTRUCTIONS: &'static [&'static str] = &[
    "nop", "jcn", "fim", "src", "fin", "jin", "jun", "jms", "inc", "isz", "add", "sub", "ld",
    "xch", "bbl", "ldm", "wrm", "wmp", "wrr", "wr0", "wr1", "wr2", "wr3", "sbm", "rdm", "rdr",
    "adm", "rd0", "rd1", "rd2", "rd3", "clb", "iac", "cmc", "cma", "ral", "rar", "tcc", "dac",
    "tcs", "stc", "daa", "kbp", "dcl"
];

enum Token {
    Label(String),
    Instruction(String),
    Number(u16),
    Comma
}

fn main() {
    let name = env::args().nth(1).unwrap();
    let f = File::open(name).unwrap();
    let mut reader = BufReader::new(f);

    for line in reader.lines() {
        let line =  line.unwrap();

        let line = match line.find(';') {
            Some(x) => line.split_at(x).0,
            None    => &*line
        }.trim();

        println!("[{}]", line);
    }

}
