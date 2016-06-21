novaXML
=======

novaXML is a DOM XML parser written in rust.
Note: This project is in its infancy and has a very limited amount of features as of now. Also, the documentation is still in progress.

Right now, the parser can only read XML Documents or Strings, but support for constructing DOM trees in memory and writing them to file is planned.

This project tracks rust stable.

What is planned (Sorted by priority):
* Complete well-formed-ness checking according to the XML 1.0 Specification
* Support for constructing and writing XML documents
* Benchmarking and optimization
* XPath query support
* DTD validation

Examples
--------
Parse a file into a 'Document' struct
```rust
use novaxml::Document;

let doc = Document::from_file("file.xml").unwrap();
```

The Document root can now be accessed like this:
```rust
let root_element = doc.get_root();
```

Element children or attributes can either be accessed by name or iterated over:
```rust
let child = root_element.get_child("name");
let attribute = child.get_attribute("attribute");

for child in root_element.iter_children() {
	println!("child found: {}", child.get_name());
}
```

License
--------
This project is MIT licensed. 
Please see the LICENSE file for more information.
