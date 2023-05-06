mod regex;

use regex::Regex;
use regex::Symbol;
use regex::RegexElement;
use regex::Quantifier;

impl Symbol for &str{}

type StrRegex<'a> = Regex<&'a str>;

#[test]
fn test(){
    let a = StrRegex::new(RegexElement::Item("i", Quantifier::OneOrMany))
        .then_elem(
            RegexElement::Set("10", "20", Quantifier::Exactly(10))
        ).then(
            Regex::new(RegexElement::Group(vec![RegexElement::Item("o", Quantifier::Exactly(1))], Quantifier::ZeroOrMany))
        );


    println!("{a:?}");

        
}

