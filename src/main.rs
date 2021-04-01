//     copie - A file copy tool which copies files from and to paths specified in environment variables.
//
//         The MIT License (MIT)
//
//      Copyright (c) Jonathan H. R. Lopes (https://github.com/JonathanxD) <jhrldev@gmail.com>
//      Copyright (c) contributors
//
//      Permission is hereby granted, free of charge, to any person obtaining a copy
//      of this software and associated documentation files (the "Software"), to deal
//      in the Software without restriction, including without limitation the rights
//      to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//      copies of the Software, and to permit persons to whom the Software is
//      furnished to do so, subject to the following conditions:
//
//      The above copyright notice and this permission notice shall be included in
//      all copies or substantial portions of the Software.
//
//      THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//      IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//      FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//      AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//      LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//      OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
//      THE SOFTWARE.

//! *copie* is a command-line tool to copy files from or to paths specified in environment variables
//! and in command-line arguments.
//!
//! ## Usage
//!
//! ### Copy from a file into the file specified in command-line
//!
//! ```fish
//! COPIE_FROM=myfile.json copie target_file.json
//! ```
//!
//! This command copies the file `myfile.json` to `target_file.json`, overwriting the `target_file.json`
//! if it already exists.
//!
//! ### Copy from file specified in command-line to another file.
//!
//! ```fish
//! COPIE_TO=target_file.json copie myfile.json
//! ```
//!
//! This command copies the contents of `myfile.json` into the `target_file.json`, overwriting the target
//! if it already exists.
//!
//! ### Copy from a file specified in environment variable to another file specified in environment variable.
//!
//! ```fish
//! COPIE_TO=target_file.json COPIE_FROM=myfile.json copie
//! ```
//!
//! This command copies the contents of `myfile.json` into the `target_file.json`, overwriting the target
//! if it already exists.
//!
//! ### Using in place of an editor to capture the file contents.
//!
//! Some applications use the editor specified in `$VISUAL` or `$EDITOR` environment variable,
//! such as the awesome [edit](https://crates.io/crates/edit) crate does, however, while doing integration
//! with other applications, it may be interesting to open these files in the editor UI instead of the editor
//! provided in these variables, and some applications are not able to handle opening these files
//! natively while being able to "block" the command while the file is open.
//!
//! **Copie** provides a easy way to capture the file contents and copy to another file, however, keep
//! in mind that **Copie** exits immediately after copying, with the exit code 0 by default, meaning that
//! if the application who request the editor uses the exit code as a signal to check if editing succeeded,
//! then it may conclude the operation, which could not be a desired result. To overcome this, **copie**
//! exit code could be changed by setting the `$COPIE_EXIT_CODE` environment variable, by doing that,
//! **copie** will always exit with the specified code in case of success. For failures, **copie** always exits
//! with the `-1` code and prints the error message to the standard error output.
//!
//! Example of use of **copie** as an editor capture tool:
//!
//! ```fish
//! VISUAL="copie" EDITOR="copie" COPIE_TO=./tmp/myfile pijul record
//! ```
//!
//! ## Practical uses
//!
//! *copie* is used by [Dracon for IntelliJ](https://nest.pijul.com/Jonathan/Dracon) to act like an
//! editor and capture the file contents and load in [IntelliJ IDEA](https://www.jetbrains.com/pt-br/idea/)
//! editor to provide an immersive integration.
//!
//!
//!

extern crate clap;
use clap::{Arg, App};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let exit_code = env::var("COPIE_EXIT_CODE").map_err(|e| format!("{:?}", e));
    let exit_code_as_int = if let Ok(code) = exit_code {
        code.parse::<i32>().map_err(|e| format!("{:?}", e))
    } else {
        Err(exit_code.err().unwrap())
    };

    std::process::exit(match run_app() {
        Ok(_) => exit_code_as_int.unwrap_or(0),
        Err(err) => {
            eprintln!("error: {:?}", err);
            -1
        }
    });
}

fn run_app() -> Result<(), String> {
    let matches = App::new("copie")
        .version("1.0")
        .author("Jonathan H. R. Lopes <jhrldev@gmail.com>")
        .about("Copies data from or to specified file.\nPaths must be specified as environment variables: COPIE_FROM specifies the file to copy from and COPIE_TO specifies the file to copy to.\nWhen COPIE_TO is combined with an input file specified as first argument, the content of the file specified in argument is copied to the file specified in COPIE_TO environment variable.\nWhen COPIE_FROM is combined with an input file specified as first argument, the content of the file specified in COPIE_FROM is copied to the file specified as first argument.\nWhen COPIE_FROM is combined with COPIE_TO, the file specified in the first one is copied to the file specified in the second one.")
        .arg(Arg::new("FILE")
            .about("Sets the file to read from or to write to.")
            .required(false)
            .index(1))
        .get_matches();

    let file_to_read_or_replace =  matches.value_of("FILE").map(|i| i.to_string()).map(|i| PathBuf::from(i));
    let from_var = env::var("COPIE_FROM").map(|i| PathBuf::from(i));
    let to_var = env::var("COPIE_TO").map(|i| PathBuf::from(i));

    if from_var.is_ok() && to_var.is_ok() && file_to_read_or_replace.is_some() {
        return Err(format!("COPIE_FROM, COPIE_TO and a file parameter was specified, copie could not copy because it is impossible to determine which file is to copy from or to."))
    }

    let from_path = if let Ok(from) = from_var {
        if !from.exists() {
            return Err(format!("Path '{}' specified in variable COPIE_FROM does not exists!", from.to_string_lossy()))
        }

        if from.is_dir() {
            return Err(format!("Path '{}' specified in variable COPIE_FROM is a directory, COPEE does not copy directories!", from.to_string_lossy()))
        }

        Some(from)
    } else {
        None
    };

    let to_path = if let Ok(to) = to_var {

        if to.is_dir() {
            return Err(format!("Path '{}' specified in variable COPIE_TO is a directory, COPEE does not copy directories!", to.to_string_lossy()))
        }

        Some(to)
    } else {
        None
    };

    if from_path.is_some() && to_path.is_some() {
        let from = from_path.unwrap();
        let to = to_path.unwrap();

        if from.is_dir() {
            return Err(format!("Path '{}' specified in COPIE_FROM variable is a directory, COPEE does not copy directories.", from.to_string_lossy()))
        }

        if to.exists() {
            return Err(format!("Path '{}' specified in COPIE_TO already exists, COPEE does not replace files.", to.to_string_lossy()))
        }

        let result = fs::copy(from.clone(), to.clone());

        if let Err(err) = result {
            return Err(format!("Failed to copy from '{}' to '{}': {:?}", from.to_string_lossy(), to.to_string_lossy(), err))
        }

        return Ok(());
    }

    let file_to_read_or_replace_path =
        if let Some(file) = file_to_read_or_replace {

            if !file.exists() {
                return Err(format!("Path '{}' specified in the command line does not exists!", file.to_string_lossy()))
            }

            if file.is_dir() {
                return Err(format!("Path '{}' specified in the command line is a directory, COPEE does not copy directories!", file.to_string_lossy()))
            }

            Some(file)
        } else {
            None
        };

    if from_path.is_some() && file_to_read_or_replace_path.is_some() {
        let from = from_path.unwrap();
        let target = file_to_read_or_replace_path.unwrap();

        let result = fs::copy(from.clone(), target.clone());

        if let Err(err) = result {
            return Err(format!("Failed to copy from '{}' to '{}': {:?}", from.to_string_lossy(), target.to_string_lossy(), err))
        }

        return Ok(());
    }

    if to_path.is_some() && file_to_read_or_replace_path.is_some() {
        let to = to_path.unwrap();
        let origin = file_to_read_or_replace_path.unwrap();

        let result = fs::copy(origin.clone(), to.clone());

        if let Err(err) = result {
            return Err(format!("Failed to copy from '{}' to '{}': {:?}", origin.to_string_lossy(), to.to_string_lossy(), err))
        }

        return Ok(());
    }

    if file_to_read_or_replace_path.is_none() {
        return Err(format!("Missing file to read or to replace."))
    }

    if from_path.is_none() || to_path.is_none() {
        return Err(format!("Missing environment variable COPIE_FROM or COPIE_TO!"))
    }

    return Err(format!("Unknown error!"))
}