use std::iter::Peekable;
use std::str::Chars;

enum Token {
    LPar,
    RPar,
    Item(String),
    End
}

pub enum SList {
    Atom(String),
    List(Vec<SList>)
}

fn next(mut from: Peekable<Chars>) -> (Token, Peekable<Chars>) {
    let mut local = from.clone();
    while let Some(p) = local.next() {
        match p {
            ' ' => {
                from.next();
                continue;
            },
            '(' => {
                from.next();
                return (Token::LPar, from);
            },
            ')' => {
                from.next();
                return (Token::RPar, from);
            },
            _ => {
                let mut token: Vec<char> = Vec::new();
                let mut local = from.clone();
                while let Some(p) = local.next() {
                    match p {
                        ' ' | '(' | ')' => break,
                        _ => {
                            from.next();
                            token.push(p);
                        }
                    }
                }
                if token.len() > 0 {
                    let string: String = token.into_iter().collect();
                    return (Token::Item(string), from);
                } else {
                    return (Token::End, from);
                }
            }
        }
    } 
    return (Token::End, from);
} 

fn parse(mut chars: Peekable<Chars>) -> (Vec<SList>, Peekable<Chars>) {
    let mut slist: Vec<SList> = Vec::new();
    loop {
        let (token, from) = next(chars);
        match token {
            Token::LPar => {
                let (nestedlist, nestedchars) = parse(from);
                slist.push(SList::List(nestedlist));
                chars = nestedchars;
            },
            Token::RPar => {
                chars = from;
                break;
            },
            Token::Item(string) => {
                chars = from;
                slist.push(SList::Atom(string));
            },
            Token::End => {
                chars = from;
                break;
            },
        }
    }
    return (slist, chars);
}

pub fn find_list<'a>(list: &'a Vec<SList>, name: String) -> Option<&'a Vec<SList>> {
    for item in list {
        let copy_name = name.clone();
        match item {
            &SList::List(ref slist) => {
                let result = find_list(slist, copy_name);
                match result {
                    Some(_) => return result,
                    None => continue
                }
            },
            &SList::Atom(ref key) if *key == copy_name => {
                return Some(&list)
            },
            &SList::Atom(_) => continue
        }
    }
    return None;
}

pub fn sparser(string: String) -> Vec<SList> {
    let mut chars = string.chars().peekable();
    chars.next();
    let (slist, _) = parse(chars);
    return slist;
}

/* Example

fn print(list: Vec<SList>, deep: usize) {
    for item in list {
        match item {
            SList::Atom(string) => {
                println!("{}{}", std::iter::repeat(" ").take(deep).collect::<String>(), string);
            },
            SList::List(slist) => {
                print(slist, deep + 1);
            }
        }
    }
}

fn main() {
    let test = "(a test list with (a nested (a nested (and a nested)) list) and 4 elements (just for fun))";
    let slist = sparser(test.to_string());
    print(slist, 0);
}

*/