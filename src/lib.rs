pub mod regex;

use regex::Regex;
use regex::Symbol;
use regex::RegexElement;
use regex::Quantifier;

impl Symbol for &str{}
impl Symbol for char{}

type StrRegex<'a> = Regex<&'a str>;
type ChrRegex = Regex<char>;

#[test]
fn test(){
    let regex = ChrRegex::new()
        .then(RegexElement::Item('@', Quantifier::ZeroOrOne))
        .then(RegexElement::Item('>', Quantifier::OneOrMany))
        .then(RegexElement::Item('i', Quantifier::ZeroOrMany))
        .then(RegexElement::Item('.', Quantifier::Exactly(1)));

    let candidate1 = &"@>i.".chars().collect::<Vec<char>>();
    let candidate2 = &"@@iii".chars().collect::<Vec<char>>();
    let candidate3 = &"iii.".chars().collect::<Vec<char>>();
    let candidate4 = &">>>.".chars().collect::<Vec<char>>();
    let candidate5 = &">.".chars().collect::<Vec<char>>();
    
    assert!(regex.r#match(candidate1) == true);
    assert!(regex.r#match(candidate2) == false);
    assert!(regex.r#match(candidate3) == false);
    assert!(regex.r#match(candidate4) == true);
    assert!(regex.r#match(candidate5) == true);

        
}

