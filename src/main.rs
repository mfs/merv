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
use std::ascii::AsciiExt;

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

fn do_match(br: BufReader<&File>, pattern: &str) {
    for line in br.lines() {
        let s = line.unwrap();

        if pmatch(pattern, &s) {
            println!("{}", s);
        }
    }
}

fn do_anagram(br: BufReader<&File>, word: &str) {

    let mut word_chars: Vec<char> = word.to_ascii_lowercase().chars().collect();
    word_chars.sort();

    for line in br.lines() {
        let l = line.unwrap();
        if l.len() != word.len() {
            continue;
        }

        let mut line_chars: Vec<char> = l.to_ascii_lowercase().chars().collect();
        line_chars.sort();
        if word_chars == line_chars {
            println!("{}", l);
        }
    }
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

    if args[1].contains('.') {
        do_match(buf, &args[1]);
    } else {
        do_anagram(buf, &args[1]);
    }
}
