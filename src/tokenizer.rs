// tokens:
//
//  Left: <
//  Right: >
//  ClosingLeft: </
//  ClosingRight: />
//  PILeft: <?
//  PIRight: ?>
//  MDLeft: <!
//  Value(key, value): key="value"
//  Text(text): text
//  Comment: <!-- -->
    
#[derive(Clone, Debug)]
pub enum Token {
    Left,
    Right,
    ClosingLeft,
    ClosingRight,
    PILeft,
    PIRight,
    MDLeft,
    Value(String, String),
    Text(String),
    Comment,
}

pub fn invalid_token(last: &char,
                     s: &char) -> Result<Vec<Token>, String> {
    Err(format!("Invalid Token: {}{}", last, s))
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let s = strip(s);
    let mut acc = String::new();

    let mut tokens = Vec::new();
    let mut last = Vec::new();
    let mut last_space = false;

    let mut reading_value = false;
    let mut value_name = String::new();

    let mut reading_comment = false;

    for c in s.chars() {
        // for reading key/value pairs
        if reading_value {
            if c == '"' || c == '\'' {
                tokens.push(Token::Value(value_name.clone(),
                                         acc.clone()));
                value_name.clear();
                acc.clear();
                reading_value = false;
                last_space = true;
                continue;
            }
            else {
                acc.push(c);
                continue;
            }
        }

        if reading_comment {
            if !last.is_empty() {
                if last.len() == 2 {
                    if last[0] == '-' && last[1] == '-' && c == '>' {
                        reading_comment = false;
                    }
                }

                if last[0] == '-' && c == '-' {
                    last.push(c);
                }
                else {
                    last.clear();
                }
            }

            if c == '-' {
                last.push(c);
            }

            continue;
        }

        // match next token
        match c {
            '<' => {
                last_space = false;
                if !acc.is_empty() {
                    tokens.push(Token::Text(acc.clone()));
                    acc.clear();
                }
                if last.is_empty() {
                    last.push(c);
                }
                else {
                    return invalid_token(&last[0], &c);
                }
            }
            '>' => {
                last_space = false;
                if !acc.is_empty() {
                    tokens.push(Token::Text(acc.clone()));
                    acc.clear();
                }
                if last.is_empty() {
                    tokens.push(Token::Right);
                }
                else if last[0] == '/' {
                    tokens.push(Token::ClosingRight);
                    last.clear();
                }
                else if last[0] == '?' {
                    tokens.push(Token::PIRight);
                    last.clear();
                }
                else {
                    return invalid_token(&last[0], &c);
                }
            }
            '/' => {
                last_space = false;
                if !acc.is_empty() {
                    tokens.push(Token::Text(acc.clone()));
                    acc.clear();
                }
                if last.is_empty() {
                    last.push(c);
                }
                else if last[0] == '<' {
                    tokens.push(Token::ClosingLeft);
                    last.clear();
                }
                else {
                    return invalid_token(&last[0], &c);
                }
            }
            '"' | '\'' => {
                last_space = false;
                reading_value = true;

                if !acc.is_empty() {
                    tokens.push(Token::Text(acc.clone()));
                    acc.clear();
                }

                if !last.is_empty() {
                    return invalid_token(&last[0], &c);
                }
            }
            '=' => {
                last_space = false;
                if !acc.is_empty() {
                    value_name = acc.clone();
                    acc.clear();
                }
                if !last.is_empty() {
                    return invalid_token(&last[0], &c);
                }
            }
            ' ' => {
                last_space = true;
            }
            '?' => {
                if !last.is_empty() {
                    if last[0] == '<' {
                        last_space = false;
                        last.clear();
                        tokens.push(Token::PILeft);
                        acc.clear();
                    }
                    else {
                        return invalid_token(&last[0], &c);
                    }
                }
                else {
                    last.push(c);
                    acc.clear();
                }
            }
            '!' => {
                if last.is_empty() {
                    return invalid_token(&last[0], &c);
                }
                else if last[0] == '<' {
                    last_space = false;
                    last.push(c);
                    acc.clear();
                }
            }
            '-' => {
                if !last.is_empty() {
                    if last.len() == 3 {
                        if last[0] == '<'
                           && last[1] == '!' 
                           && last[2] == '-' {
                               last.clear();
                               reading_comment = true;
                           }
                    }
                    else if last.len() == 2 {
                        if last[0] == '<'
                           && last[1] == '!' {
                               last.push(c);
                           }
                    }
                    else {
                        return invalid_token(&last[0], &c);
                    }
                }
            }
            _ => {
                if last.len() == 2 {
                    if last[0] == '<' && last[1] == '!' {
                        tokens.push(Token::MDLeft);
                    }
                }

                if !last.is_empty() {
                    if last[0] == '<' {
                        tokens.push(Token::Left);
                        last.clear();
                    }
                }
                if c.is_alphanumeric() {
                    if last_space && !acc.is_empty() {
                        last_space = false;
                        tokens.push(Token::Text(acc.clone()));
                        acc.clear();

                        acc.push(c);
                    }
                    else {
                        last_space = false;
                        acc.push(c);
                    }
                }
                else {
                    if !acc.is_empty() {
                        tokens.push(Token::Text(acc.clone()));
                        acc.clear();
                    }
                }
            }
        }
    }
    Ok(tokens)
}

pub fn string_from_file(path: &str) -> Option<String> {
    use std::fs::File;
    use std::io::Read;
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("Error while reading file at {}: {}",
                     path,
                     e);
            return None;
        }
    };
    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Ok(_) => {}
        Err(e) => {
            println!(
                "Error while reading string {} to file: {}",
                string,
                e);
            return None;
        }
    }
    Some(string)
}

pub fn strip(s: &str) -> String {
    let mut stripped = String::new();
    let mut last_space = false;
    for c in s.chars() {
        if !"\n\t".to_string().contains(c) {
            if c == ' ' {
                if !last_space {
                last_space = true;
                }
                else { continue; }
            }
            else { last_space = false; }
        }
        else { continue; }
        stripped.push(c);
    }
    // println!("stripped: {:?}", stripped);
    stripped
}
