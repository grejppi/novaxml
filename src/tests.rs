#[cfg(test)]

#[test]
fn parse_file() {
    use super::Document;
    let xml_doc_test 
        = match Document::from_file("res/player_light.sprite") {
        Ok(xml_doc_test) => xml_doc_test,
        Err(e) => panic!("loading failed: {}", e),
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

#[test]
fn iter_children() {
    use super::Document;
    let xml_doc_test 
        = match Document::from_file("res/player_light.sprite") {
        Ok(xml_doc_test) => xml_doc_test,
        Err(e) => panic!("loading failed: {}", e),
    };

    for child in xml_doc_test.get_root().iter_children() {
        println!("child found: {}", child.get_name());
        for child in child.iter_children() {
            println!("child found: {}", child.get_name());
        }
    }
}

#[test]
fn get_child() {
    use super::Document;
    let xml_doc_test 
        = match Document::from_file("res/player_light.sprite") {
        Ok(xml_doc_test) => xml_doc_test,
        Err(e) => panic!("loading failed: {}", e),
    };

    let child = match xml_doc_test.get_root().get_child("animations") {
        Some(child) => child,
        None => panic!("child not found!"),
    };

    println!("child found: {}", child.get_name());
}
