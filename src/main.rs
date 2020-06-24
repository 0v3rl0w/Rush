/*
 * RUst SHell - A Simple Shell written in Rust
 * Copyright (C) 2020 0v3rl0w & contributors
 * 
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::{env::args, io::Write, io::stdout, io::stdin};
use rustyline::Editor;
use nix::unistd::{fork, ForkResult};

fn parseArg(args: Vec<String>) {
    let mut i: usize = 0;
    while i < args.len() {
       match args[i].as_str() {
               "--version" => {
                   println!("Rush, version {}", env!("CARGO_PKG_VERSION"));
                   println!("Copyright (C) 2020 0v3rl0w");
                   println!("License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>\n");
                   println!("This is free software; you are free to change and redistribute it.");
                   println!("There is NO WARRANTY, to the extent permitted by law.");
               }

               _ => {}
       } 
       
       i += 1;

    }
}

fn execute(command: String) -> isize {
    let tokens: Vec<&str> = command.split_whitespace().collect();

    match tokens[0] {
        "exit" => std::process::exit(0),
        _ => {
            /*match fork() {
                Ok(ForkResult::Child) => {
                    
                }
            }*/ 1
        }
    }
}


fn prompt() -> ! {
    let mut r1 = Editor::<()>::new();
    let ps1 = "$ ";

    loop {
        print!("{}", ps1);
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input);

        let returncode: isize = execute(input);
    }
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 1 {
        parseArg(args);
    } else {
        prompt();
    }
}
