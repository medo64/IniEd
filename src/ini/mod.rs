use std::fmt;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Error;
use std::io::Write;

#[cfg(test)]
mod test;


pub struct IniFile {
    lines: Vec<IniLine>,  // all lines
}

impl IniFile {
    fn new(lines: Vec<IniLine>) -> IniFile {
        IniFile { 
            lines: lines,
        }
    }
}

impl IniFile {
    pub fn parse(file_name: &str) -> Result<IniFile, Error> {
        let file = File::open(file_name)?;
        let mut reader = io::BufReader::new(file);

        let mut lines = Vec::new();

        let mut input_line = String::new();
        while reader.read_line(&mut input_line).is_ok() {
            if input_line.len() == 0 { break; }

            if input_line.ends_with("\r\n") {
                input_line.truncate(input_line.len() - 2);
                let line = IniLine::new(&input_line, "\r\n");
                lines.push(line);
            } else if input_line.ends_with("\n") {
                input_line.truncate(input_line.len() - 1);
                let line = IniLine::new(&input_line, "\n");
                lines.push(line);
            } else if input_line.ends_with("\r") { //doesn't happen really as read_line ends on \n only at this time
                input_line.truncate(input_line.len() - 1);
                let line = IniLine::new(&input_line, "\r");
                lines.push(line);
            } else { //last line might not have EOL character
                let line = IniLine::new(&input_line, "");
                lines.push(line);
            }

            input_line.truncate(0);
        }

        Ok(IniFile::new(lines))
    }

    pub fn save(&self, file_name: &str) -> Result<(), Error> {
        let mut file = File::create(file_name)?;
        for line in &self.lines {
            file.write(line.content.to_string().as_bytes())?;
            file.write(line.line_ending.as_bytes())?;
        }
        Ok(())
    }
}

impl IniFile {
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
}

impl IniFile {
    pub fn reformat(&mut self) {
        if self.lines.len() > 0 {
            let line_ending = self.lines[0].get_line_ending();
            let mut lines = Vec::new();
            let mut had_entries = false;
            for line in self.lines.clone() {
                match line.clone().content {
                    IniContent::Section(_) => {
                        if had_entries { lines.push(IniLine::empty(&line_ending.to_owned())); }
                        lines.push(line.reformatted(&line_ending.to_owned()));
                        had_entries = false;
                    },
                    IniContent::Entry(_)   => {
                        lines.push(line.reformatted(&line_ending.to_owned()));
                        had_entries = true;
                    },
                    IniContent::Comment(_) => { lines.push(line.reformatted(&line_ending.to_owned())); },
                    IniContent::Other(_)   => { },
                }
            }

            self.lines.clear();
            self.lines.append(&mut lines);
        }
    }

    pub fn trim(&mut self) {
        if self.lines.len() > 0 {
            let mut lines = Vec::new();
            for line in self.lines.clone() {
                match line.clone().content {
                    IniContent::Section(_) => { lines.push(line.trimmed()); },
                    IniContent::Entry(_)   => { lines.push(line.trimmed()); },
                    IniContent::Other(_)   => { lines.push(line.trimmed()); },
                    IniContent::Comment(_) => { lines.push(line.trimmed()); },
                }
            }

            self.lines.clear();
            self.lines.append(&mut lines);
        }
    }

    pub fn remove_comments(&mut self) {
        if self.lines.len() > 0 {
            let mut lines = Vec::new();
            for line in self.lines.clone() {
                match line.clone().content {
                    IniContent::Section(_) => { lines.push(line); },
                    IniContent::Entry(_)   => { lines.push(line); },
                    IniContent::Other(_)   => { lines.push(line); },
                    IniContent::Comment(_) => { },
                }
            }

            self.lines.clear();
            self.lines.append(&mut lines);
        }
    }

    pub fn filter(&mut self, filter_section: Option<&str>, filter_key: Option<&str>) {
        assert!(filter_section.is_some() || filter_key.is_some());

        if self.lines.len() > 0 {
            let mut is_section_matched = false;
            let mut lines = Vec::new();
            for line in self.lines.clone() {
                match line.clone().content {
                    IniContent::Section(section) => {
                        match filter_section {
                            Some(filter_section) => {
                                is_section_matched = section.name == filter_section;
                                if is_section_matched && filter_key.is_none() { lines.push(line); }
                            },
                            None => { is_section_matched = true; },
                        }
                    },
                    IniContent::Entry(entry)   => {
                        if is_section_matched {
                            match filter_key {
                                Some(filter_key) => {
                                    if entry.key == filter_key {
                                        lines.push(line);
                                    }
                                },
                                None => { lines.push(line); },
                            }
                        }
                    },
                    IniContent::Other(_)   => { },
                    IniContent::Comment(_) => { },
                }
            }

            self.lines.clear();
            self.lines.append(&mut lines);
        }
    }


    pub fn delete(&mut self, filter_section: Option<&str>, filter_key: Option<&str>) {
        assert!(filter_section.is_some() || filter_key.is_some());

        if self.lines.len() > 0 {
            let mut is_section_matched = false;
            let mut lines = Vec::new();
            for line in self.lines.clone() {
                match line.clone().content {
                    IniContent::Section(section) => {
                        match filter_section {
                            Some(filter_section) => {
                                is_section_matched = section.name == filter_section;
                                if !is_section_matched {
                                    lines.push(line);
                                } else if is_section_matched && filter_key.is_some() { //only write section if we're deleting the key
                                    lines.push(line);
                                }
                            },
                            None => {
                                is_section_matched = false;
                                lines.push(line);
                            },
                        }
                    },
                    IniContent::Entry(entry) => {
                        if is_section_matched {
                            match filter_key {
                                Some(filter_key) => { if entry.key != filter_key { lines.push(line); } }, //append lines that are not filtered
                                None => { },
                            }
                        } else {
                            lines.push(line);
                        }
                    },
                    IniContent::Comment(_) => { if !is_section_matched || filter_key.is_some() { lines.push(line); } },
                    IniContent::Other(_)   => { if !is_section_matched || filter_key.is_some() { lines.push(line); } },
                }
            }

            self.lines.clear();
            self.lines.append(&mut lines);
        }
    }

    pub fn edit(&mut self, section_name: &str, key: &str, value: &str, modify_existing: bool, create_new: bool) {
        let mut line_ending = "\n";
        let mut had_section_matched = false;
        let mut had_key_matched = false;

        let cloned_lines = self.lines.clone();
        if cloned_lines.len() > 0 { line_ending = &cloned_lines[0].get_line_ending(); }

        if self.lines.len() > 0 {
            let mut is_section_matched = false;
            for line in self.lines.clone() {
                match line.content {
                    IniContent::Section(section) => {
                        is_section_matched = section.name == section_name;
                        had_section_matched = had_section_matched || is_section_matched;
                    },
                    IniContent::Entry(entry)     => {
                        had_key_matched = had_key_matched  || (is_section_matched && (entry.key == key));
                    },
                    IniContent::Comment(_)       => { },
                    IniContent::Other(_)         => { },
                }
            }
        }

        if modify_existing && had_key_matched {
            let mut lines = Vec::new();
            let mut is_section_matched = false;

            for line in self.lines.clone() {
                match line.clone().content {
                    IniContent::Section(section) => {
                        is_section_matched = section.name == section_name;
                        lines.push(line);
                    },
                    IniContent::Entry(entry)     => {
                        let is_key_matched = is_section_matched && (entry.key == key);
                        if is_key_matched {
                            lines.push(IniLine { 
                                content:     IniContent::Entry(entry.with_modified_value(value)),
                                line_ending: line.get_line_ending().to_string(),
                            });
                        } else {
                            lines.push(line);
                        }
                    },
                    IniContent::Comment(_)       => { lines.push(line); },
                    IniContent::Other(_)         => { lines.push(line); },
                }
            }

            self.lines.clear();
            self.lines.append(&mut lines);
        } else if create_new {
            if !had_section_matched { //if section doesn't exist, we don't need to search for one
                self.lines.push(IniLine { 
                    content:     IniContent::Section(IniSection::create(section_name)),
                    line_ending: line_ending.to_string(),
                });
                self.lines.push(IniLine { 
                    content:     IniContent::Entry(IniEntry::create(key, value)),
                    line_ending: line_ending.to_string(),
                });
            } else { //we need to append to existing section
                let mut lines = Vec::new();
                let mut is_section_matched = false;
                let mut was_section_matched = false;
                let mut consecutive_other_count = 0; //to keep track how far back we need to go to insert item
                let mut done = false;

                for line in self.lines.clone() {
                    match line.clone().content {
                        IniContent::Section(section) => {
                            is_section_matched = section.name == section_name;
                        },
                        IniContent::Entry(_)         => { consecutive_other_count = 0; },
                        IniContent::Comment(_)       => { consecutive_other_count = 0; },
                        IniContent::Other(_)         => { consecutive_other_count += 1; },
                    }

                    if !done && !is_section_matched && was_section_matched {
                        let new_index = lines.len() - consecutive_other_count;
                        lines.insert(new_index, IniLine {
                            content:     IniContent::Entry(IniEntry::create(key, value)),
                            line_ending: line_ending.to_string(),
                        });
                        done = true;
                        consecutive_other_count = 0;
                    }

                    lines.push(line);
                    was_section_matched = is_section_matched;
                }

                if !done {
                    let new_index = lines.len() - consecutive_other_count;
                    lines.insert(new_index, IniLine {
                        content:     IniContent::Entry(IniEntry::create(key, value)),
                        line_ending: line_ending.to_string(),
                    });
                }

                self.lines.clear();
                self.lines.append(&mut lines);
            }
        }
    }
}

impl IntoIterator for IniFile {
    type Item = IniLine;
    type IntoIter = IniFileIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        IniFileIntoIterator {
            lines: self.lines,
            index: 0,
        }
    }
}

pub struct IniFileIntoIterator {
    lines: Vec<IniLine>,
    index: usize,
}

impl Iterator for IniFileIntoIterator {
    type Item = IniLine;
    fn next(&mut self) -> Option<IniLine> {
        if self.index < self.lines.len() {
            let line = self.lines[self.index].clone();
            self.index += 1;
            Some(line)
        } else {
            None
        }
    }
}


#[derive(Clone)]
pub struct IniLine {
    content:     IniContent,  // line content
    line_ending: String,      // line ending
}

impl IniLine {
    fn new(raw_content: &str, line_ending: &str) -> IniLine {
        let mut prefix = Vec::new();
        let mut suffix = Vec::new();
        let mut section_name = Vec::new();
        let mut key = Vec::new();
        let mut separator = Vec::new();
        let mut value = Vec::new();
        let mut comment_prelude = Vec::new();
        let mut comment_text = Vec::new();

        enum State { Prefix, Suffix, SectionName, Key, Separator, Value, CommentText }
        let mut state = State::Prefix;

        enum Kind { Section, Entry, Comment, Other }
        let mut kind = Kind::Other;

        for c in raw_content.chars() {
            match &state {
                State::Prefix => {
                    if c.is_whitespace() {
                        prefix.push(c);
                    } else if c == '[' {
                        kind = Kind::Section;
                        state = State::SectionName;
                    } else if c == ';' || c == '#' {
                        comment_prelude.push(c);
                        kind = Kind::Comment;
                        state = State::CommentText;
                    } else if c == '=' { //this is invalid, just move to suffix
                        suffix.push(c);
                        state = State::Suffix;
                    } else {
                        key.push(c);
                        kind = Kind::Entry;
                        state = State::Key;
                    }
                },

                State::Suffix => {
                    if !c.is_whitespace() {
                        match kind {
                            Kind::Entry => {
                                value.append(&mut suffix);
                                value.push(c);
                                state = State::Value;
                            },
                            _ => { suffix.push(c); }
                        }
                    } else {
                        suffix.push(c);
                    }
                },

                State::SectionName => {
                    if c == ']' {
                        state = State::Suffix;
                    } else {
                        section_name.push(c);
                    }
                },

                State::Key => {
                    if c.is_whitespace() || c == '=' {
                        separator.push(c);
                        state = State::Separator;
                    } else {
                        key.push(c);
                    }
                },

                State::Separator => {
                    if c.is_whitespace() {
                        separator.push(c);
                    } else if c == '=' {
                        if separator.contains(&'=') { //second equals belongs to value
                            value.push(c);
                            state = State::Value;
                        } else {
                            separator.push(c);
                        }
                    } else {
                        value.push(c);
                        state = State::Value;
                    }
                },

                State::Value => {
                    if c.is_whitespace() {
                        suffix.push(c);
                        state = State::Suffix;
                    } else {
                        value.push(c);
                    }
                },

                State::CommentText => {
                    comment_text.push(c);
                },
            }
        }

        //fixups
        match state {
            State::SectionName => { //never finished SectionName
                prefix.push('[');
                prefix.append(&mut section_name);
                kind = Kind::Other;
            },
            State::Key => {
                prefix.append(&mut key);
                kind = Kind::Other;
            },
            State::Separator if !separator.contains(&'=') => {
                prefix.append(&mut key);
                prefix.append(&mut separator);
                kind = Kind::Other;
            },
            _ => {},
        }

        match kind {
            Kind::Section => {
                let section = IniSection::new(&prefix, &vec!['['], &section_name, &vec![']'], &suffix);
                IniLine {
                    content: IniContent::Section(section),
                    line_ending: line_ending.to_string(),
                }
            },
            Kind::Entry => {
                let entry = IniEntry::new(&prefix, &key, &separator, &value, &suffix);
                IniLine {
                    content: IniContent::Entry(entry),
                    line_ending: line_ending.to_string(),
                }
            },
            Kind::Comment => {
                let comment = IniComment::new(&prefix, &comment_prelude, &comment_text);
                IniLine {
                    content: IniContent::Comment(comment),
                    line_ending: line_ending.to_string(),
                }
            },
            Kind::Other => {
                prefix.append(&mut suffix); //combine prefix and suffix
                let other = IniOther::new(&prefix);
                IniLine {
                    content: IniContent::Other(other),
                    line_ending: line_ending.to_string(),
                }
            },
        }
    }

    fn empty(line_ending: &str) -> IniLine {
        IniLine {
            content: IniContent::Other(IniOther::empty()),
            line_ending: line_ending.to_string(),
        }
    }
}

impl IniLine {
    fn reformatted(&self, line_ending: &str) -> IniLine {
        match &self.content {
            IniContent::Section(section) => IniLine { content: IniContent::Section(section.reformatted()), line_ending: line_ending.to_string() },
            IniContent::Entry(entry)     => IniLine { content: IniContent::Entry(entry.reformatted()),     line_ending: line_ending.to_string() },
            IniContent::Comment(comment) => IniLine { content: IniContent::Comment(comment.reformatted()), line_ending: line_ending.to_string() },
            IniContent::Other(_)         => IniLine { content: IniContent::Other(IniOther::empty()),       line_ending: line_ending.to_string() },
        }
    }

    fn trimmed(&self) -> IniLine {
        match &self.content {
            IniContent::Section(section) => IniLine { content: IniContent::Section(section.trimmed()), line_ending: self.line_ending.clone() },
            IniContent::Entry(entry)     => IniLine { content: IniContent::Entry(entry.trimmed()),     line_ending: self.line_ending.clone() },
            IniContent::Comment(comment) => IniLine { content: IniContent::Comment(comment.trimmed()), line_ending: self.line_ending.clone() },
            IniContent::Other(other)     => IniLine { content: IniContent::Other(other.trimmed()),     line_ending: self.line_ending.clone() },
        }
    }
}

impl IniLine {
    fn get_line_ending(&self) -> &str {
        self.line_ending.as_str()
    }

    pub fn get_content(&self) -> IniContent {
        self.content.clone()
    }
}


#[derive(Clone)]
pub enum IniContent {
    Section(IniSection),
    Entry(IniEntry),
    Comment(IniComment),
    Other(IniOther),
}

impl fmt::Display for IniContent {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
       match self {
           IniContent::Section(section) => write!(formatter, "{}", section),
           IniContent::Entry(entry)     => write!(formatter, "{}", entry),
           IniContent::Comment(comment) => write!(formatter, "{}", comment),
           IniContent::Other(other)     => write!(formatter, "{}", other),
       }
    }
}


#[derive(Clone)]
pub struct IniSection {
    prefix:   String,  // any starting whitespace
    prelude:  String,  // section start character ([)
    name:     String,  // section name
    postlude: String,  // section end character (])
    suffix:   String,  // any trailing stuff
}

impl IniSection {
    fn new(prefix: &Vec<char>, prelude: &Vec<char>, name: &Vec<char>, postlude: &Vec<char>, suffix: &Vec<char>) -> IniSection {
        IniSection {
            prefix:   prefix.into_iter().collect(),
            prelude:  prelude.into_iter().collect(),
            name:     name.into_iter().collect(),
            postlude: postlude.into_iter().collect(),
            suffix:   suffix.into_iter().collect(),
        }
    }

    fn create(name: &str) -> IniSection {
        IniSection {
            prefix:   String::new(),
            prelude:  "[".to_string(),
            name:     name.to_string(),
            postlude: "]".to_string(),
            suffix:   String::new(),
        }
    }
}

impl IniSection {
    fn reformatted(&self) -> IniSection {
        IniSection {
            prefix:   String::new(),
            prelude:  self.prelude.clone(),
            name:     self.name.clone(),
            postlude: self.postlude.clone(),
            suffix:   String::new(),
        }
    }

    fn trimmed(&self) -> IniSection {
        IniSection {
            prefix:   String::new(),
            prelude:  self.prelude.clone(),
            name:     self.name.clone(),
            postlude: self.postlude.clone(),
            suffix:   String::new(),
        }
    }
}

impl fmt::Display for IniSection {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}{}{}{}{}", self.prefix, self.prelude, self.name, self.postlude, self.suffix)
    }
}


#[derive(Clone)]
pub struct IniEntry {
    prefix:    String,  // prefix
    key:       String,  // key name
    separator: String,  // separator (alongside any spacing)
    value:     String,  // value
    suffix:    String,  // suffix
}

impl IniEntry {
    fn new(prefix: &Vec<char>, key: &Vec<char>, separator: &Vec<char>, value: &Vec<char>, suffix: &Vec<char>) -> IniEntry {
        IniEntry {
            prefix:    prefix.into_iter().collect(),
            key:       key.into_iter().collect(),
            separator: separator.into_iter().collect(),
            value:     value.into_iter().collect(),
            suffix:    suffix.into_iter().collect(),
        }
    }

    fn create(key: &str, value: &str)-> IniEntry  {
        IniEntry {
            prefix:    String::new(),
            key:       key.to_string(),
            separator: "=".to_string(),
            value:     value.to_string(),
            suffix:    String::new(),
        }
    }
}

impl IniEntry {
    fn reformatted(&self) -> IniEntry {
        IniEntry {
            prefix:    String::new(),
            key:       self.key.clone(),
            separator: "=".to_string(),
            value:     self.value.clone(),
            suffix:    String::new(),
        }
    }

    fn trimmed(&self) -> IniEntry {
        IniEntry {
            prefix:    String::new(),
            key:       self.key.clone(),
            separator: self.separator.clone(),
            value:     self.value.clone(),
            suffix:    String::new(),
        }
    }

    fn with_modified_value(&self, value: &str) -> IniEntry {
        IniEntry {
            prefix:    String::new(),
            key:       self.key.clone(),
            separator: self.separator.clone(),
            value:     value.to_string(),
            suffix:    String::new(),
        }
    }
}

impl IniEntry {
    pub fn get_value(&self) -> &str {
        &self.value
    }
}

impl fmt::Display for IniEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}{}{}{}{}", self.prefix, self.key, self.separator, self.value, self.suffix)
    }
}


#[derive(Clone)]
pub struct IniComment {
    prefix:  String,  // any leading whitespace
    prelude: String,  // comment character
    text:    String,  // any comment text
}

impl IniComment {
    fn new(prefix: &Vec<char>, prelude: &Vec<char>, text: &Vec<char>) -> IniComment {
        IniComment {
            prefix:  prefix.into_iter().collect(),
            prelude: prelude.into_iter().collect(),
            text:    text.into_iter().collect(),
        }
    }
}

impl IniComment {
    fn reformatted(&self) -> IniComment {
        IniComment {
            prefix:  String::new(),
            prelude: self.prelude.clone() + " ",
            text:    self.text.clone().trim().to_string(),
        }
    }

    fn trimmed(&self) -> IniComment {
        IniComment {
            prefix:  String::new(),
            prelude: self.prelude.clone() + " ",
            text:    self.text.clone().trim().to_string(),
        }
    }
}

impl fmt::Display for IniComment {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}{}{}", self.prefix, self.prelude, self.text)
    }
}


#[derive(Clone)]
pub struct IniOther {
    text: String,  // any text
}

impl IniOther {
    fn new(text: &Vec<char>) -> IniOther {
        IniOther {
            text: text.into_iter().collect(),
        }
    }

    fn empty() -> IniOther {
        IniOther {
            text: String::new(),
        }
    }
}

impl IniOther {
    fn trimmed(&self) -> IniOther {
        IniOther {
            text: self.text.clone().trim().to_string(),
        }
    }
}

impl fmt::Display for IniOther {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.text)
    }
}
