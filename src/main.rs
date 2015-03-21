/*
This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.
You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

// matches a pattern such as 'e..mp..'
// against a string such as  'example'
fn pmatch(pattern: &str, target: &str) -> bool {
    // early out
    if pattern.len() != target.len() {
        return false;
    }
    for (x, y) in pattern.chars().zip(target.chars()) {
        if x == y || x == '.' {
            continue;
        } else {
            return false;
        }
    }

    true
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} <PATTERN>", args[0]);
        return;
    }

    let dict_file = "/usr/share/dict/merv";

    let file = match File::open(dict_file) {
        Ok(file) => file,
        Err(err) => { println!("Error opening {}. {}", dict_file, err); return; }
    };

    let buf = BufReader::new(&file);

    for line in buf.lines() {
        let s = line.unwrap();

        if pmatch(&args[1], &s) {
            println!("{}", s);
        }
    }
}
