impl super::IniComment {
    fn create(prelude: &str, text: &str) -> super::IniComment {
        super::IniComment {
            prefix:   String::new(),
            prelude:  prelude.to_string(),
            text:     text.to_string(),
        }
    }
}

impl super::IniOther {
    fn create(text: &str) -> super::IniOther {
        super::IniOther {
            text:     text.to_string(),
        }
    }
}


#[test]
fn parse_section_basic() {
    let parsed = super::IniLine::new("[Test]", "");
    match parsed.content {
        super::IniContent::Section(section) => {
            assert_eq!("[Test]", section.to_string());
            assert_eq!("", section.prefix);
            assert_eq!("[", section.prelude);
            assert_eq!("Test", section.name);
            assert_eq!("]", section.postlude);
            assert_eq!("", section.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_section_space_prefix() {
    let parsed = super::IniLine::new("  [Test]", "");
    match parsed.content {
        super::IniContent::Section(section) => {
            assert_eq!("  [Test]", section.to_string());
            assert_eq!("  ", section.prefix);
            assert_eq!("Test", section.name);
            assert_eq!("", section.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_section_space_suffix() {
    let parsed = super::IniLine::new("[Test]  ", "");
    match parsed.content {
        super::IniContent::Section(section) => {
            assert_eq!("[Test]  ", section.to_string());
            assert_eq!("", section.prefix);
            assert_eq!("Test", section.name);
            assert_eq!("  ", section.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_section_space_prefix_and_suffix() {
    let parsed = super::IniLine::new("\t[Test]\t", "");
    match parsed.content {
        super::IniContent::Section(section) => {
            assert_eq!("\t[Test]\t", section.to_string());
            assert_eq!("\t", section.prefix);
            assert_eq!("Test", section.name);
            assert_eq!("\t", section.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_section_space_inner_space() {
    let parsed = super::IniLine::new("[Test A]", "");
    match parsed.content {
        super::IniContent::Section(section) => {
            assert_eq!("[Test A]", section.to_string());
            assert_eq!("", section.prefix);
            assert_eq!("Test A", section.name);
            assert_eq!("", section.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_section_reformatted() {
    let parsed = super::IniLine::new("   [Test]    ", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Section(section) => {
            assert_eq!("[Test]", section.to_string());
            assert_eq!("", section.prefix);
            assert_eq!("Test", section.name);
            assert_eq!("", section.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_section_trimmed() {
    let parsed = super::IniLine::new("   [Test]    ", "");
    let parsed = parsed.trimmed();
    match parsed.content {
        super::IniContent::Section(section) => {
            assert_eq!("[Test]", section.to_string());
            assert_eq!("", section.prefix);
            assert_eq!("Test", section.name);
            assert_eq!("", section.suffix);
        },
        _ => panic!("failed match"),
    }
}


#[test]
fn parse_entry_basic() {
    let parsed = super::IniLine::new("Key=Value", "");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key=Value", entry.to_string());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("Value", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_empty() {
    let parsed = super::IniLine::new("Key=", "");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key=", entry.to_string());
            assert_eq!("", entry.get_value());
            assert_eq!("", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_separator_with_spaces() {
    let parsed = super::IniLine::new("Key = Value", "");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key = Value", entry.to_string());
            assert_eq!("Value", entry.get_value());
            assert_eq!("Value", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!(" = ", entry.separator);
            assert_eq!("Value", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_double_equals() {
    let parsed = super::IniLine::new("Key = =Value", "");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key = =Value", entry.to_string());
            assert_eq!("=Value", entry.get_value());
            assert_eq!("=Value", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!(" = ", entry.separator);
            assert_eq!("=Value", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_double_inner_space() {
    let parsed = super::IniLine::new("Key = Value With Spaces  ", "");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key = Value With Spaces  ", entry.to_string());
            assert_eq!("Value With Spaces", entry.get_value());
            assert_eq!("Value With Spaces", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!(" = ", entry.separator);
            assert_eq!("Value With Spaces", entry.value);
            assert_eq!("  ", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_reformatted() {
    let parsed = super::IniLine::new("    Key = Value    ", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key=Value", entry.to_string());
            assert_eq!("Value", entry.get_value());
            assert_eq!("Value", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("Value", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_trimmed() {
    let parsed = super::IniLine::new("    Key = Value    ", "");
    let parsed = parsed.trimmed();
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key = Value", entry.to_string());
            assert_eq!("Value", entry.get_value());
            assert_eq!("Value", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!(" = ", entry.separator);
            assert_eq!("Value", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_double_quoted() {
    let parsed = super::IniLine::new("    Key = \"Value\"    ", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key=\"Value\"", entry.to_string());
            assert_eq!("\"Value\"", entry.get_value());
            assert_eq!("Value", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("\"Value\"", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_single_quoted() {
    let parsed = super::IniLine::new("    Key = 'Value'    ", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key='Value'", entry.to_string());
            assert_eq!("'Value'", entry.get_value());
            assert_eq!("Value", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("'Value'", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_double_quoted_with_spaces() {
    let parsed = super::IniLine::new("    Key = \"  Value  Spaced  \"    ", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key=\"  Value  Spaced  \"", entry.to_string());
            assert_eq!("\"  Value  Spaced  \"", entry.get_value());
            assert_eq!("  Value  Spaced  ", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("\"  Value  Spaced  \"", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_single_quoted_with_spaces() {
    let parsed = super::IniLine::new("    Key = '  Value  Spaced  '    ", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key='  Value  Spaced  '", entry.to_string());
            assert_eq!("'  Value  Spaced  '", entry.get_value());
            assert_eq!("  Value  Spaced  ", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("'  Value  Spaced  '", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_double_quoted_with_escaped_chars() {
    let parsed = super::IniLine::new("    Key = \"  Value \\t \\n \\r \\\" \\' Spaced  \"    ", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key=\"  Value \\t \\n \\r \\\" \\' Spaced  \"", entry.to_string());
            assert_eq!("\"  Value \\t \\n \\r \\\" \\' Spaced  \"", entry.get_value());
            assert_eq!("  Value \t \n \r \" ' Spaced  ", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("\"  Value \\t \\n \\r \\\" \\' Spaced  \"", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_single_quoted_with_escaped_chars() {
    let parsed = super::IniLine::new("    Key = '  Value \\t \\n \\r \\\" \\' Spaced  '    ", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key='  Value \\t \\n \\r \\\" \\' Spaced  '", entry.to_string());
            assert_eq!("'  Value \\t \\n \\r \\\" \\' Spaced  '", entry.get_value());
            assert_eq!("  Value \t \n \r \" ' Spaced  ", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("'  Value \\t \\n \\r \\\" \\' Spaced  '", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_double_quoted_with_invalid_escape() {
    let parsed = super::IniLine::new("    Key = \"  Value \\q Spaced  \"    ", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key=\"  Value \\q Spaced  \"", entry.to_string());
            assert_eq!("\"  Value \\q Spaced  \"", entry.get_value());
            assert_eq!("  Value \\q Spaced  ", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("\"  Value \\q Spaced  \"", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_single_quoted_with_invalid_escape() {
    let parsed = super::IniLine::new("    Key = '  Value \\q Spaced  '    ", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key='  Value \\q Spaced  '", entry.to_string());
            assert_eq!("'  Value \\q Spaced  '", entry.get_value());
            assert_eq!("  Value \\q Spaced  ", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("'  Value \\q Spaced  '", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_single_quoted_double_inner() {
    let parsed = super::IniLine::new("Key='Something '' Else'", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key='Something '' Else'", entry.to_string());
            assert_eq!("'Something '' Else'", entry.get_value());
            assert_eq!("Something ' Else", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("'Something '' Else'", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_double_quoted_double_inner() {
    let parsed = super::IniLine::new("Key=\"Something \"\" Else\"", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key=\"Something \"\" Else\"", entry.to_string());
            assert_eq!("\"Something \"\" Else\"", entry.get_value());
            assert_eq!("Something \" Else", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("\"Something \"\" Else\"", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_single_quoted_stray_quote() {
    let parsed = super::IniLine::new("Key='Give ' Up'", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key='Give ' Up'", entry.to_string());
            assert_eq!("'Give ' Up'", entry.get_value());
            assert_eq!("'Give ' Up'", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("'Give ' Up'", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_single_quoted_no_escape() {
    let parsed = super::IniLine::new("Key='No \"\" Change'", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key='No \"\" Change'", entry.to_string());
            assert_eq!("'No \"\" Change'", entry.get_value());
            assert_eq!("No \"\" Change", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("'No \"\" Change'", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_entry_double_quoted_no_escape() {
    let parsed = super::IniLine::new("Key=\"No '' Change\"", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key=\"No '' Change\"", entry.to_string());
            assert_eq!("\"No '' Change\"", entry.get_value());
            assert_eq!("No '' Change", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("\"No '' Change\"", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}
#[test]
fn parse_entry_double_quoted_stray_quote() {
    let parsed = super::IniLine::new("Key=\"Give \" Up\"", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Entry(entry) => {
            assert_eq!("Key=\"Give \" Up\"", entry.to_string());
            assert_eq!("\"Give \" Up\"", entry.get_value());
            assert_eq!("\"Give \" Up\"", entry.get_value_unquoted());
            assert_eq!("", entry.prefix);
            assert_eq!("Key", entry.key);
            assert_eq!("=", entry.separator);
            assert_eq!("\"Give \" Up\"", entry.value);
            assert_eq!("", entry.suffix);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_comment_semicolon() {
    let parsed = super::IniLine::new(";Test", "");
    match parsed.content {
        super::IniContent::Comment(comment) => {
            assert_eq!(";Test", comment.to_string());
            assert_eq!("", comment.prefix);
            assert_eq!(";", comment.prelude);
            assert_eq!("Test", comment.text);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_comment_hash() {
    let parsed = super::IniLine::new("#Test", "");
    match parsed.content {
        super::IniContent::Comment(comment) => {
            assert_eq!("#Test", comment.to_string());
            assert_eq!("", comment.prefix);
            assert_eq!("#", comment.prelude);
            assert_eq!("Test", comment.text);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_comment_reformatted() {
    let parsed = super::IniLine::new("  #    Test    ", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Comment(comment) => {
            assert_eq!("# Test", comment.to_string());
            assert_eq!("", comment.prefix);
            assert_eq!("# ", comment.prelude);
            assert_eq!("Test", comment.text);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_comment_trimmed() {
    let parsed = super::IniLine::new("  #    Test    ", "");
    let parsed = parsed.trimmed();
    match parsed.content {
        super::IniContent::Comment(comment) => {
            assert_eq!("# Test", comment.to_string());
            assert_eq!("", comment.prefix);
            assert_eq!("# ", comment.prelude);
            assert_eq!("Test", comment.text);
        },
        _ => panic!("failed match"),
    }
}


#[test]
fn parse_other_empty() {
    let parsed = super::IniLine::new("", "");
    match parsed.content {
        super::IniContent::Other(other) => {
            assert_eq!("", other.to_string());
            assert_eq!("", other.text);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_other_spaces() {
    let parsed = super::IniLine::new("  ", "");
    match parsed.content {
        super::IniContent::Other(other) => {
            assert_eq!("  ", other.to_string());
            assert_eq!("  ", other.text);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_other_semi_section() {
    let parsed = super::IniLine::new("[Test ", "");
    match parsed.content {
        super::IniContent::Other(other) => {
            assert_eq!("[Test ", other.to_string());
            assert_eq!("[Test ", other.text);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_other_starts_with_equals() {
    let parsed = super::IniLine::new("=", "");
    match parsed.content {
        super::IniContent::Other(other) => {
            assert_eq!("=", other.to_string());
            assert_eq!("=", other.text);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_other_reformat() {
    let parsed = super::IniLine::new("   Testing   ", "");
    let parsed = parsed.reformatted("");
    match parsed.content {
        super::IniContent::Other(other) => {
            assert_eq!("", other.to_string());
            assert_eq!("", other.text);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_other_trimmed() {
    let parsed = super::IniLine::new("   Testing   ", "");
    let parsed = parsed.trimmed();
    match parsed.content {
        super::IniContent::Other(other) => {
            assert_eq!("Testing", other.to_string());
            assert_eq!("Testing", other.text);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_other_starts_with_equals_and_spaces() {
    let parsed = super::IniLine::new(" = ", "");
    match parsed.content {
        super::IniContent::Other(other) => {
            assert_eq!(" = ", other.to_string());
            assert_eq!(" = ", other.text);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_other_unfinished_entry() {
    let parsed = super::IniLine::new("Key", "");
    match parsed.content {
        super::IniContent::Other(other) => {
            assert_eq!("Key", other.to_string());
            assert_eq!("Key", other.text);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_other_unfinished_entry_with_spaces() {
    let parsed = super::IniLine::new("Key  ", "");
    match parsed.content {
        super::IniContent::Other(other) => {
            assert_eq!("Key  ", other.to_string());
            assert_eq!("Key  ", other.text);
        },
        _ => panic!("failed match"),
    }
}

#[test]
fn parse_other_looks_like_entry() {
    let parsed = super::IniLine::new("Something Else", "");
    match parsed.content {
        super::IniContent::Other(other) => {
            assert_eq!("Something Else", other.to_string());
            assert_eq!("Something Else", other.text);
        },
        _ => panic!("failed match"),
    }
}


#[test]
fn pretty_print_adds_empty_line() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),  line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry(super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),  line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry(super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.reformat();
    let file = file;
    print(&file);

    assert_eq!("[X]", file.lines[0].content.to_string());
    assert_eq!("A=1", file.lines[1].content.to_string());
    assert_eq!("",    file.lines[2].content.to_string());
    assert_eq!("[Y]", file.lines[3].content.to_string());
    assert_eq!("B=2", file.lines[4].content.to_string());
}

#[test]
fn pretty_print_removes_empty_line() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),  line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry(super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),  line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry(super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.reformat();
    let file = file;
    print(&file);

    assert_eq!("[X]", file.lines[0].content.to_string());
    assert_eq!("A=1", file.lines[1].content.to_string());
    assert_eq!("",    file.lines[2].content.to_string());
    assert_eq!("[Y]", file.lines[3].content.to_string());
    assert_eq!("B=2", file.lines[4].content.to_string());
}

#[test]
fn filter_section() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.filter(Some("X"), None);
    let file = file;
    print(&file);

    assert_eq!(2,      file.lines.len());
    assert_eq!("[X]",  file.lines[0].content.to_string());
    assert_eq!("A=1",  file.lines[1].content.to_string());
}

#[test]
fn filter_entry_found() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.filter(Some("X"), Some("A"));
    let file = file;
    print(&file);

    assert_eq!(1,      file.lines.len());
    assert_eq!("A=1",  file.lines[0].content.to_string());
}

#[test]
fn filter_entry_section_not_found() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.filter(Some("-"), Some("A"));
    let file = file;
    print(&file);

    assert_eq!(0,      file.lines.len());
}

#[test]
fn filter_entry_key_not_found() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.filter(Some("X"), Some("-"));
    let file = file;
    print(&file);

    assert_eq!(0,      file.lines.len());
}

#[test]
fn append_no_spacing() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.edit("X", "A", "10", false, true);
    file.edit("Y", "B", "20", false, true);
    file.edit("X", "C", "30", false, true);
    file.edit("Y", "D", "40", false, true);
    file.edit("Z", "E", "50", false, true);
    let file = file;
    print(&file);

    assert_eq!(10,     file.lines.len());
    assert_eq!("[X]",  file.lines[0].content.to_string());
    assert_eq!("A=1",  file.lines[1].content.to_string());
    assert_eq!("A=10", file.lines[2].content.to_string());
    assert_eq!("C=30", file.lines[3].content.to_string());
    assert_eq!("[Y]",  file.lines[4].content.to_string());
    assert_eq!("B=2",  file.lines[5].content.to_string());
    assert_eq!("B=20", file.lines[6].content.to_string());
    assert_eq!("D=40", file.lines[7].content.to_string());
    assert_eq!("[Z]",  file.lines[8].content.to_string());
    assert_eq!("E=50", file.lines[9].content.to_string());
}

#[test]
fn append_with_spacing() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Comment(super::IniComment::create("#", "-")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.edit("X", "A", "10", false, true);
    file.edit("Y", "B", "20", false, true);
    file.edit("X", "C", "30", false, true);
    file.edit("Y", "D", "40", false, true);
    file.edit("Z", "E", "50", false, true);
    let file = file;
    print(&file);

    assert_eq!(13,     file.lines.len());
    assert_eq!("[X]",  file.lines[0].content.to_string());
    assert_eq!("A=1",  file.lines[1].content.to_string());
    assert_eq!("#-",   file.lines[2].content.to_string());
    assert_eq!("A=10", file.lines[3].content.to_string());
    assert_eq!("C=30", file.lines[4].content.to_string());
    assert_eq!("",     file.lines[5].content.to_string());
    assert_eq!("[Y]",  file.lines[6].content.to_string());
    assert_eq!("B=2",  file.lines[7].content.to_string());
    assert_eq!("B=20", file.lines[8].content.to_string());
    assert_eq!("D=40", file.lines[9].content.to_string());
    assert_eq!("",     file.lines[10].content.to_string());
    assert_eq!("[Z]",  file.lines[11].content.to_string());
    assert_eq!("E=50", file.lines[12].content.to_string());
}

#[test]
fn append_with_multi_spacing() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Comment(super::IniComment::create("#", "-")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.edit("X", "A", "10", false, true);
    file.edit("Y", "B", "20", false, true);
    file.edit("X", "C", "30", false, true);
    file.edit("Y", "D", "40", false, true);
    file.edit("Z", "E", "50", false, true);
    let file = file;
    print(&file);

    assert_eq!(16,     file.lines.len());
    assert_eq!("[X]",  file.lines[0].content.to_string());
    assert_eq!("A=1",  file.lines[1].content.to_string());
    assert_eq!("#-",   file.lines[2].content.to_string());
    assert_eq!("A=10", file.lines[3].content.to_string());
    assert_eq!("C=30", file.lines[4].content.to_string());
    assert_eq!("",     file.lines[5].content.to_string());
    assert_eq!("",     file.lines[6].content.to_string());
    assert_eq!("",     file.lines[7].content.to_string());
    assert_eq!("[Y]",  file.lines[8].content.to_string());
    assert_eq!("B=2",  file.lines[9].content.to_string());
    assert_eq!("B=20", file.lines[10].content.to_string());
    assert_eq!("D=40", file.lines[11].content.to_string());
    assert_eq!("",     file.lines[12].content.to_string());
    assert_eq!("",     file.lines[13].content.to_string());
    assert_eq!("[Z]",  file.lines[14].content.to_string());
    assert_eq!("E=50", file.lines[15].content.to_string());
}

#[test]
fn change_no_spacing() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.edit("X", "A", "10", true, false);
    file.edit("Y", "B", "20", true, false);
    file.edit("X", "C", "30", true, false);
    file.edit("Y", "D", "40", true, false);
    file.edit("Z", "E", "50", true, false);
    let file = file;
    print(&file);

    assert_eq!(4,      file.lines.len());
    assert_eq!("[X]",  file.lines[0].content.to_string());
    assert_eq!("A=10", file.lines[1].content.to_string());
    assert_eq!("[Y]",  file.lines[2].content.to_string());
    assert_eq!("B=20", file.lines[3].content.to_string());
}

#[test]
fn change_with_spacing() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Comment(super::IniComment::create("#", "-")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.edit("X", "A", "10", true, false);
    file.edit("Y", "B", "20", true, false);
    file.edit("X", "C", "30", true, false);
    file.edit("Y", "D", "40", true, false);
    file.edit("Z", "E", "50", true, false);
    let file = file;
    print(&file);

    assert_eq!(7,     file.lines.len());
    assert_eq!("[X]",  file.lines[0].content.to_string());
    assert_eq!("A=10", file.lines[1].content.to_string());
    assert_eq!("#-",   file.lines[2].content.to_string());
    assert_eq!("",     file.lines[3].content.to_string());
    assert_eq!("[Y]",  file.lines[4].content.to_string());
    assert_eq!("B=20", file.lines[5].content.to_string());
    assert_eq!("",     file.lines[6].content.to_string());
}

#[test]
fn change_with_multi_spacing() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Comment(super::IniComment::create("#", "-")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.edit("X", "A", "10", true, false);
    file.edit("Y", "B", "20", true, false);
    file.edit("X", "C", "30", true, false);
    file.edit("Y", "D", "40", true, false);
    file.edit("Z", "E", "50", true, false);
    let file = file;
    print(&file);

    assert_eq!(10,     file.lines.len());
    assert_eq!("[X]",  file.lines[0].content.to_string());
    assert_eq!("A=10", file.lines[1].content.to_string());
    assert_eq!("#-",   file.lines[2].content.to_string());
    assert_eq!("",     file.lines[3].content.to_string());
    assert_eq!("",     file.lines[4].content.to_string());
    assert_eq!("",     file.lines[5].content.to_string());
    assert_eq!("[Y]",  file.lines[6].content.to_string());
    assert_eq!("B=20", file.lines[7].content.to_string());
    assert_eq!("",     file.lines[8].content.to_string());
    assert_eq!("",     file.lines[9].content.to_string());
}

#[test]
fn edit_no_spacing() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.edit("X", "A", "10", true, true);
    file.edit("Y", "B", "20", true, true);
    file.edit("X", "C", "30", true, true);
    file.edit("Y", "D", "40", true, true);
    file.edit("Z", "E", "50", true, true);
    let file = file;
    print(&file);

    assert_eq!(8,      file.lines.len());
    assert_eq!("[X]",  file.lines[0].content.to_string());
    assert_eq!("A=10", file.lines[1].content.to_string());
    assert_eq!("C=30", file.lines[2].content.to_string());
    assert_eq!("[Y]",  file.lines[3].content.to_string());
    assert_eq!("B=20", file.lines[4].content.to_string());
    assert_eq!("D=40", file.lines[5].content.to_string());
    assert_eq!("[Z]",  file.lines[6].content.to_string());
    assert_eq!("E=50", file.lines[7].content.to_string());
}

#[test]
fn edit_with_spacing() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Comment(super::IniComment::create("#", "-")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.edit("X", "A", "10", true, true);
    file.edit("Y", "B", "20", true, true);
    file.edit("X", "C", "30", true, true);
    file.edit("Y", "D", "40", true, true);
    file.edit("Z", "E", "50", true, true);
    let file = file;
    print(&file);

    assert_eq!(11,     file.lines.len());
    assert_eq!("[X]",  file.lines[0].content.to_string());
    assert_eq!("A=10", file.lines[1].content.to_string());
    assert_eq!("#-",   file.lines[2].content.to_string());
    assert_eq!("C=30", file.lines[3].content.to_string());
    assert_eq!("",     file.lines[4].content.to_string());
    assert_eq!("[Y]",  file.lines[5].content.to_string());
    assert_eq!("B=20", file.lines[6].content.to_string());
    assert_eq!("D=40", file.lines[7].content.to_string());
    assert_eq!("",     file.lines[8].content.to_string());
    assert_eq!("[Z]",  file.lines[9].content.to_string());
    assert_eq!("E=50", file.lines[10].content.to_string());
}

#[test]
fn edit_with_multi_spacing() {
    let mut lines = Vec::new();
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("X")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("A", "1")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Comment(super::IniComment::create("#", "-")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Section(super::IniSection::create("Y")),      line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Entry  (  super::IniEntry::create("B", "2")), line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    lines.push(super::IniLine { content: super::IniContent::Other  (  super::IniOther::create("")),       line_ending: "\n".to_string() });
    let mut file = super::IniFile { lines: lines };

    file.edit("X", "A", "10", true, true);
    file.edit("Y", "B", "20", true, true);
    file.edit("X", "C", "30", true, true);
    file.edit("Y", "D", "40", true, true);
    file.edit("Z", "E", "50", true, true);
    let file = file;
    print(&file);

    assert_eq!(14,     file.lines.len());
    assert_eq!("[X]",  file.lines[0].content.to_string());
    assert_eq!("A=10", file.lines[1].content.to_string());
    assert_eq!("#-",   file.lines[2].content.to_string());
    assert_eq!("C=30", file.lines[3].content.to_string());
    assert_eq!("",     file.lines[4].content.to_string());
    assert_eq!("",     file.lines[5].content.to_string());
    assert_eq!("",     file.lines[6].content.to_string());
    assert_eq!("[Y]",  file.lines[7].content.to_string());
    assert_eq!("B=20", file.lines[8].content.to_string());
    assert_eq!("D=40", file.lines[9].content.to_string());
    assert_eq!("",     file.lines[10].content.to_string());
    assert_eq!("",     file.lines[11].content.to_string());
    assert_eq!("[Z]",  file.lines[12].content.to_string());
    assert_eq!("E=50", file.lines[13].content.to_string());
}


fn print(file: &super::IniFile) {
    let mut line_number = 0;
    for line in file.lines.clone() {
        print!("{:2}", line_number);
        let content = line.get_content();
        match content {
            super::IniContent::Section(_) => { print!(" S: "); },
            super::IniContent::Entry(_)   => { print!(" E: "); },
            super::IniContent::Comment(_) => { print!(" C: "); },
            super::IniContent::Other(_)   => { print!(" O: "); },
        }
        line_number += 1;
        println!("{}", line.get_content());
    }
}
