//  Copyright (C) 2023 @servo
//  https://github.com/servomekanism/hybrid
//
//  hybrid - This small program imitates the great python script
//  namemash.py (https://gist.github.com/superkojiman/11076951).
//  It creates permutations of possible usernames based on a
//  NAME SURNAME line contained in a text file, taken as input.
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
//  For more see the file 'LICENSE' for copying permission.

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

#[rustfmt::skip]
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        return Ok(());
    }

    let names_file = File::open(&args[1])?;
    let usernames_file = File::create(&args[2])?;
    let mut names = Vec::new();

    let names_reader = BufReader::new(names_file);
    let mut writer = BufWriter::new(usernames_file);

    for name in names_reader.lines() {
        let name_surname = name.unwrap();
        let part: Vec<&str> = name_surname.split(' ').collect();
        names.push((part[0].to_string(), part[1].to_string()));
    }

    let mut permutations = Vec::new();

    for (name, surname) in names {
        let username_permutations = vec![
            format!("{}{}", name.to_lowercase().chars().next().unwrap(), surname.to_lowercase()),
            format!("{}.{}", name.to_lowercase().chars().next().unwrap(), surname.to_lowercase()),
            format!("{}{}", name.to_lowercase(), surname.to_lowercase()),
            format!("{}.{}", name.to_lowercase(), surname.to_lowercase()),
            format!("{}{}", surname.to_lowercase(), name.to_lowercase().chars().next().unwrap()),
            format!("{}.{}", surname.to_lowercase(), name.to_lowercase().chars().next().unwrap()),
            format!("{}{}", surname.to_lowercase(), name.to_lowercase()),
            format!("{}.{}", surname.to_lowercase(), name.to_lowercase()),
            format!("{}{}", name.to_lowercase().chars().next().unwrap(), surname.to_lowercase().chars().next().unwrap()),
            format!("{}{}", surname.to_lowercase().chars().next().unwrap(), name.to_lowercase().chars().next().unwrap())
        ];

        permutations.push(username_permutations);
    }

    for permutation in permutations.iter().flatten() {
        writeln!(writer, "{}", permutation).unwrap();
    }

    Ok(())
}
