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
    let candidate6 = &">>>>..".chars().collect::<Vec<char>>();
    
    assert_eq!(regex.r#match(candidate1), true);
    assert_eq!(regex.r#match(candidate2), false);
    assert_eq!(regex.r#match(candidate3), false);
    assert_eq!(regex.r#match(candidate4), true);
    assert_eq!(regex.r#match(candidate5), true);
    assert_eq!(regex.r#match(candidate6), false);

        
}

#[test]
fn group_test(){
    let regex = ChrRegex::new()
        .then(RegexElement::Group(vec![RegexElement::Item('a', Quantifier::OneOrMany), RegexElement::Item('b', Quantifier::Exactly(1)) ], Quantifier::Exactly(1)));

    let candidate1 = &"hello world".chars().collect::<Vec<char>>();
    let candidate2 = &"".chars().collect::<Vec<char>>();
    let candidate3 = &"bbbbb".chars().collect::<Vec<char>>();
    let candidate4 = &"ab".chars().collect::<Vec<char>>();
    let candidate5 = &"a".chars().collect::<Vec<char>>();
    let candidate6 = &"aaaaaaaaa".chars().collect::<Vec<char>>();
    let candidate7 = &"aaaaaaaaaaaaab".chars().collect::<Vec<char>>();
    let candidate8 = &"abababab".chars().collect::<Vec<char>>();
    let candidate9 = &"aaaabaaaabaaaabaaaab".chars().collect::<Vec<char>>();
    let candidate10 = &"aaaaabbbbbb".chars().collect::<Vec<char>>();
    let candidate11 = &"abbbbb".chars().collect::<Vec<char>>();
    let candidate12 = &"b".chars().collect::<Vec<char>>();

    assert_eq!(regex.r#match(candidate1), false, "'hello world' test");
    assert_eq!(regex.r#match(candidate2), false, "chaîne vide");
    assert_eq!(regex.r#match(candidate3), false, "que des 'b'");
    assert_eq!(regex.r#match(candidate4), true, "une seule occurence du pattern 'ab'");
    assert_eq!(regex.r#match(candidate5), false, "un seul 'a'");
    assert_eq!(regex.r#match(candidate6), false, "que des 'a'");
    assert_eq!(regex.r#match(candidate7), true, "chaîne de 'a' puis un 'b'");

    assert_eq!(regex.r#match(candidate8), false, "4 occurences du pattern 'ab'");
    assert_eq!(regex.r#match(candidate9), false, "4 occurences du pattern 'aaaab'");

    assert_eq!(regex.r#match(candidate10), false, "chaîne de 'a' puis une chaîne de 'b'");
    assert_eq!(regex.r#match(candidate11), false, "un 'a' puis une chaîne de 'b'");
    assert_eq!(regex.r#match(candidate12), false, "un seul 'b'");

}

#[test]
fn group_quantifier_test(){
    let regex = ChrRegex::new()
        .then(RegexElement::Group(vec![RegexElement::Item('a', Quantifier::OneOrMany), RegexElement::Item('b', Quantifier::Exactly(1)) ], Quantifier::OneOrMany));

    let candidate1 = &"ababab".chars().collect::<Vec<char>>();
    let candidate2 = &"aaabaaabaaab".chars().collect::<Vec<char>>();
    let candidate3 = &"ababab10ab4a5".chars().collect::<Vec<char>>();
    let candidate4 = &"abaaabab".chars().collect::<Vec<char>>();
    let candidate5 = &"aaabababaaab".chars().collect::<Vec<char>>();

    assert_eq!(regex.r#match(candidate1), true, "3 occurences du pattern 'ab'");
    assert_eq!(regex.r#match(candidate2), true, "3 occurences du pattern 'aaab'");
    assert_eq!(regex.r#match(candidate3), false, "interruption du pattern 'ab'");
    assert_eq!(regex.r#match(candidate4), true, "interruption du pattern 'ab' (bis)");
    assert_eq!(regex.r#match(candidate5), true, "interruption du pattern 'aaab'");
}


#[test]
fn number(){
    let regex = ChrRegex::new()
        .then(RegexElement::Group(
            vec![
                RegexElement::Item('-', Quantifier::ZeroOrOne),
                RegexElement::Set('0', '9', Quantifier::OneOrMany)
            ], Quantifier::ZeroOrOne
        ))

        .then(RegexElement::Group(
            vec![
                RegexElement::Item('.', Quantifier::Exactly(1)),
                RegexElement::Set('0', '9', Quantifier::OneOrMany)
            ], Quantifier::ZeroOrOne
        ));
    
    let candidate1 = &"".chars().collect::<Vec<char>>();
    let candidate2 = &"testx".chars().collect::<Vec<char>>();
    let candidate3 = &"256".chars().collect::<Vec<char>>();
    let candidate4 = &"-145".chars().collect::<Vec<char>>();
    let candidate5 = &"00001".chars().collect::<Vec<char>>();
    let candidate6 = &"3.14".chars().collect::<Vec<char>>();
    let candidate7 = &".0001".chars().collect::<Vec<char>>();
    let candidate8 = &"-.001".chars().collect::<Vec<char>>();

    assert_eq!(regex.r#match(candidate1), true);
    assert_eq!(regex.r#match(candidate2), false);
    assert_eq!(regex.r#match(candidate3), true);
    assert_eq!(regex.r#match(candidate4), true);
    assert_eq!(regex.r#match(candidate5), true);
    assert_eq!(regex.r#match(candidate6), true);
    assert_eq!(regex.r#match(candidate7), true);
    assert_eq!(regex.r#match(candidate8), true);
}

#[test]
fn snake_case(){
    let regex = ChrRegex::new()
        .then(RegexElement::Set('a', 'z', Quantifier::OneOrMany))
        .then(RegexElement::Group(
            vec![
                RegexElement::Item('_', Quantifier::Exactly(1)),
                RegexElement::Set('a', 'z', Quantifier::OneOrMany)
            ], Quantifier::ZeroOrMany
        ));

    let candidate1 = &"_test".chars().collect::<Vec<char>>();
    let candidate2 = &"10var".chars().collect::<Vec<char>>();
    let candidate3 = &"snake_case".chars().collect::<Vec<char>>();
    let candidate4 = &"camelCase".chars().collect::<Vec<char>>();
    let candidate5 = &"kebab-case".chars().collect::<Vec<char>>();
    let candidate6 = &"num#2".chars().collect::<Vec<char>>();

    assert_eq!(regex.r#match(candidate1), false);
    assert_eq!(regex.r#match(candidate2), false);
    assert_eq!(regex.r#match(candidate3), true);
    assert_eq!(regex.r#match(candidate4), false);
    assert_eq!(regex.r#match(candidate5), false);
    assert_eq!(regex.r#match(candidate6), false);
}

#[test]
fn mail(){
    let regex = ChrRegex::new()
        .then(RegexElement::Set('a', 'z', Quantifier::Exactly(1)))
        .then(RegexElement::Group(
            vec![
                RegexElement::AnyOf(vec![
                    RegexElement::Set('a', 'z', Quantifier::Exactly(1)),
                    RegexElement::Set('0', '9', Quantifier::Exactly(1))
                ])
            ], Quantifier::OneOrMany))

        .then(RegexElement::Group(
            vec![
                RegexElement::Item('.', Quantifier::Exactly(1)),
                RegexElement::Group(
                    vec![
                        RegexElement::AnyOf(vec![
                            RegexElement::Set('a', 'z', Quantifier::Exactly(1)),
                            RegexElement::Set('0', '9', Quantifier::Exactly(1))
                        ])
                    ], Quantifier::OneOrMany)
            ], Quantifier::ZeroOrMany))

        .then(RegexElement::Item('@', Quantifier::Exactly(1)))
        .then(RegexElement::Set('a', 'z', Quantifier::OneOrMany))
        .then(RegexElement::Item('.', Quantifier::Exactly(1)))
        .then(RegexElement::Set('a', 'z', Quantifier::OneOrMany));

    let candidate1 = &"super-mail-invalid@fake.abc".chars().collect::<Vec<char>>();
    let candidate2 = &"hello_world@group.tld".chars().collect::<Vec<char>>();
    let candidate3 = &"Remi.STR@yolo.com".chars().collect::<Vec<char>>();
    let candidate4 = &"".chars().collect::<Vec<char>>();
    let candidate5 = &"machin.truc@bidule.etc".chars().collect::<Vec<char>>();
    let candidate6 = &"persona04.test@fake.tv".chars().collect::<Vec<char>>();

    assert_eq!(regex.r#match(candidate1), false);
    assert_eq!(regex.r#match(candidate2), false);
    assert_eq!(regex.r#match(candidate3), false);
    assert_eq!(regex.r#match(candidate4), false);
    assert_eq!(regex.r#match(candidate5), true);
    assert_eq!(regex.r#match(candidate6), true);

}