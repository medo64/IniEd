extern crate clap;
use clap::{Arg, App};

mod ini;
use crate::ini::{IniFile, IniContent};


const CARGO_NAME: &'static str = env!("CARGO_PKG_NAME");
const CARGO_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const CARGO_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");


fn main() {
    let args = App::new(CARGO_NAME)
                .version(CARGO_VERSION)
                .author(CARGO_AUTHORS)
                .about("Editing of .ini files")
                .arg(Arg::with_name("file")
                    .help("File to edit")
                    .required(true)
                    .multiple(false)
                    .index(1))
                .arg(Arg::with_name("section")
                    .short("s")
                    .long("section")
                    .takes_value(true)
                    .help("Section to show or replace"))
                .arg(Arg::with_name("key")
                    .short("k")
                    .long("key")
                    .takes_value(true)
                    .help("Key to show or replace"))
                .arg(Arg::with_name("print")
                    .short("p")
                    .long("print")
                    .help("Show filtered value only"))
                .arg(Arg::with_name("delete")
                    .short("d")
                    .long("delete")
                    .help("Specified entry or section will be deleted"))
                .arg(Arg::with_name("append")
                    .short("a")
                    .long("append")
                    .takes_value(true)
                    .help("Value will be appended"))
                .arg(Arg::with_name("change")
                    .short("c")
                    .long("change")
                    .takes_value(true)
                    .help("Value will be changed only if it exists"))
                .arg(Arg::with_name("edit")
                    .short("e")
                    .long("edit")
                    .takes_value(true)
                    .help("Value will be changed if exists or added if it doesn't"))
                .arg(Arg::with_name("reformat")
                    .long("pretty-print")
                    .help("Format output to look nicer"))
                .arg(Arg::with_name("trim")
                    .long("trim")
                    .help("Trim leading and trailing spaces"))
                .arg(Arg::with_name("nocomments")
                    .long("no-comments")
                    .help("Remove all comments"))
                .arg(Arg::with_name("inplace")
                    .short("i")
                    .long("in-place")
                    .help("Writes content back to the same file after processing"))
                .arg(Arg::with_name("verbose")
                    .short("v")
                    .long("verbose")
                    .multiple(true)
                    .help("Sets the level of verbosity"))
                .get_matches();

    let _show_notice = args.occurrences_of("verbose") >= 1;
    let show_info = args.occurrences_of("verbose") >= 2;
    let show_debug = args.occurrences_of("verbose") >= 3;

    let exec_inplace = args.is_present("inplace");
    let exec_reformat = args.is_present("reformat");
    let exec_nocomments = args.is_present("nocomments");
    let exec_trim = args.is_present("trim");

    let find_section = args.value_of("section");
    let find_key = args.value_of("key");

    let should_print  = args.is_present("print");
    let should_delete = args.is_present("delete");
    let should_append = args.value_of("append");
    let should_change = args.value_of("change");
    let should_edit   = args.value_of("edit");

    let mut operation_count = 0;
    if should_print            { operation_count += 1; }
    if should_delete           { operation_count += 1; }
    if should_append.is_some() { operation_count += 1; }
    if should_change.is_some() { operation_count += 1; }
    if should_edit.is_some()   { operation_count += 1; }
    let operation_count = operation_count;
    if operation_count > 1 {
        eprintln!("error: only one operation (delete, append, change, edit) is allowed");
        std::process::exit(255);
    }

    if should_print && exec_inplace {
        eprintln!("error: cannot both print and replace in-place");
        std::process::exit(255);
    }

    if should_print && (find_section.is_none() || find_key.is_none()) {
        eprintln!("error: both section and key must be specified for print operation");
        std::process::exit(255);
    }

    if should_append.is_some()  && (find_section.is_none() || find_key.is_none()) {
        eprintln!("error: both section and key must be specified for append operation");
        std::process::exit(255);
    }

    if should_change.is_some()  && (find_section.is_none() || find_key.is_none()) {
        eprintln!("error: both section and key must be specified for change operation");
        std::process::exit(255);
    }

    if should_edit.is_some()  && (find_section.is_none() || find_key.is_none()) {
        eprintln!("error: both section and key must be specified for edit operation");
        std::process::exit(255);
    }


    if let Some(file_name) = args.value_of("file") {
        let file = IniFile::parse(file_name);
        match file {
            Ok(mut file) => {
                if exec_nocomments { file.remove_comments(); }
                if exec_trim { file.trim(); }

                if operation_count == 0 {
                    if find_section.is_some() || find_key.is_some() {
                        file.filter(find_section, find_key); //just filter stuff out
                    }
                } else if should_print { //just show value
                    file.filter(find_section, find_key);
                    for line in file {
                        let content = line.get_content();
                        match content {
                            IniContent::Entry(entry)   => { println!("{}", entry.get_value()); },
                            _ => { },
                        }
                    }
                    std::process::exit(0); //no need for standard printout
                } else if should_delete {
                    file.delete(find_section, find_key);
                } else if should_append.is_some() {
                    file.edit(find_section.unwrap(), find_key.unwrap(), should_append.unwrap(), false, true);
                } else if should_change.is_some() {
                    file.edit(find_section.unwrap(), find_key.unwrap(), should_change.unwrap(), true, false);
                } else if should_edit.is_some() {
                    file.edit(find_section.unwrap(), find_key.unwrap(), should_edit.unwrap(), true, true);
                }

                if exec_reformat { file.reformat(); }

                if exec_inplace {
                    match file.save(file_name) {
                        Ok(_) => { },
                        Err(err) => {
                            eprintln!("error: cannot write file '{}': {}", file_name, err);
                            std::process::exit(2);
                        },
                    }
                } else { //final output
                    let mut line_number = 0;
                    let line_number_digit_count = (file.line_count() as f64).log10().ceil() as usize;
                    for line in file {
                        if show_info {
                            line_number += 1;
                            match line_number_digit_count {
                                1 => print!("{:1}", line_number),
                                2 => print!("{:2}", line_number),
                                3 => print!("{:3}", line_number),
                                4 => print!("{:4}", line_number),
                                5 => print!("{:5}", line_number),
                                _ => print!("{:6}", line_number),
                            }
                        }
                        if show_debug {
                            let content = line.get_content();
                            match content {
                                IniContent::Section(_) => { print!(" S"); },
                                IniContent::Entry(_)   => { print!(" E"); },
                                IniContent::Comment(_) => { print!(" C"); },
                                IniContent::Other(_)   => { print!(" O"); },
                            }
                        }
                        if show_info || show_debug { print!(": "); }
                        println!("{}", line.get_content());
                    }
                    std::process::exit(0);
                }
            },
            Err(err) => {
                eprintln!("error: cannot read file '{}': {}", file_name, err);
                std::process::exit(1);
            },
        }
    }
}
