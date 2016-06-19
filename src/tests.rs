#[cfg(test)]

#[test]
fn parse_file() {
    use super::Document;
    let xml_doc_test 
        = match Document::from_file("res/player_light.sprite") {
        Some(xml_doc_test) => xml_doc_test,
        None => panic!("loading failed"),
    };
    xml_doc_test.print();
}

#[test]
fn tokenize_string() {
    use super::tokenizer::*;

    let string = match string_from_file("res/test.xml") {
        Some(string) => string,
        None => panic!("failed to load string from file"),
    };

    match tokenize(&string) {
        Ok(tokens) => tokens,
        Err(e) => panic!("Tokenizing error: {}", e),
    };
}
