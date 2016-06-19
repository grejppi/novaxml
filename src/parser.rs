use super::tokenizer::Token;
use super::nodes::*;
use std::option::Option;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ParseState {
    InDocument,
    InElement,
    InStartTag,
    InEndTag,
}

pub fn parse(tokens: Vec<Token>) -> Option<Element> {
    let mut state_stack = Vec::<ParseState>::new();
    let mut elements = Vec::<Element>::new();

    let mut current_state = ParseState::InDocument;
    let mut current_element = Element::new("root");

    for token in tokens {
        match token {
            Token::Left => {
                match current_state {
                    ParseState::InDocument => {
                        // Found beginning of tag, 
                        // create a new element for it
                        // and push the old one
                        elements.push(current_element.clone());
                        current_element = Element::new_empty();

                        // push old state to stack, set new state
                        state_stack.push(current_state);
                        current_state = ParseState::InStartTag;
                    }
                    ParseState::InElement => {
                        // Found nested Element,
                        // create a new element for it
                        // and push the old one
                        elements.push(current_element.clone());
                        current_element = Element::new_empty();

                        // since we will still be in an element
                        // after this one ends, push the state
                        // and set new current
                        state_stack.push(current_state.clone());
                        current_state = ParseState::InStartTag;
                    }
                    _ => return token_not_allowed(
                        &token,
                        &current_state),
                }
            }
            Token::ClosingLeft => {
                match current_state {
                    ParseState::InElement => {
                        // Closing tag found
                        // we are not in element anymore, 
                        // so just set current state
                        current_state = ParseState::InEndTag;
                    }
                    _ => return token_not_allowed(
                            &token, 
                            &current_state),
                }
            }
            Token::Right => {
                match current_state {
                    ParseState::InStartTag => {
                        // tag ends, body starts
                        current_state = ParseState::InElement;
                    }
                    ParseState::InEndTag => {
                        // tag ends, element is completed
                        // see if there is a parent
                        // if there is no parent, we have reached
                        // the end of the file, return the current element

                        let mut parent = match elements.pop() {
                            Some(parent) => parent,
                            None => return Some(current_element),
                        };
                        // otherwise, append current element as parent
                        // and make parent the new current
                        parent.append_child(current_element.clone());
                        current_element = parent;

                        // also, go 1 state up
                        current_state = match state_stack.pop() {
                            Some(state) => state,
                            // since InDocument state should always
                            // be there, there's an error
                            None => return None,
                        }
                    }
                    _ => return token_not_allowed(
                            &token, 
                            &current_state),
                }
            }
            Token::ClosingRight => {
                match current_state {
                    ParseState::InStartTag => {
                        // Empty element found,
                        // set empty push it
                        current_element.set_empty(true);

                        // check if there's a parent
                        let mut parent = match elements.pop() {
                            Some(parent) => parent,
                            None => return Some(current_element),
                        };
                        // otherwise, append current element as parent
                        // and make parent the new current
                        parent.append_child(current_element.clone());
                        current_element = parent;

                        // state is now the top state from the stack
                        current_state = match state_stack.pop() {
                            Some(state) => state,
                            // since InDocument state should always
                            // be there, there's an error
                            None => return None,
                        }
                    }
                    _ => return token_not_allowed(
                            &token, 
                            &current_state),
                }
            }
            Token::Value(ref n, ref v) => {
                match current_state {
                    // attributes are only allowed in
                    // start tags
                    ParseState::InStartTag => {
                        current_element.append_attribute(
                            (Attribute::new(&n, &v)));
                    }
                    _ => return token_not_allowed(
                            &token, 
                            &current_state),
                }
            }
            Token::Text(ref s) => {
                match current_state {
                    ParseState::InStartTag => {
                        // tag name found
                        current_element.set_name(s);
                    }
                    ParseState::InEndTag => {
                        // tag name found
                        // TODO: check if tag names match
                    }
                    ParseState::InElement => {
                        // found text data attribute
                        // inside element
                        current_element.set_text(s);
                    }
                    _ => return token_not_allowed(
                            &token, 
                            &current_state),
                }
            }
            Token::PILeft => {
            }
            Token::PIRight => {
            }
            Token::MDLeft => {
            }
            Token::Comment => {
            }
        }
    }
    Some(current_element)
}

pub fn token_not_allowed(t: &Token,
                         p: &ParseState) -> Option<Element> {
    println!("Token not allowed: {:?}, state: {:?}", t, p);
    None
}
