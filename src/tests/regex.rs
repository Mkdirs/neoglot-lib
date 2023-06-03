#[cfg(test)]
mod test{
    use crate::regex::{ Symbol, Quantifier, RegexElement, Regex };

    impl Symbol for char{}
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

        let (result1, _) = regex.r#match(candidate1);
        let (result2, _) = regex.r#match(candidate2);
        let (result3, _) = regex.r#match(candidate3);
        let (result4, _) = regex.r#match(candidate4);
        let (result5, _) = regex.r#match(candidate5);
        let (result6, _) = regex.r#match(candidate6);
        
        assert_eq!(result1, true);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
        assert_eq!(result4, true);
        assert_eq!(result5, true);
        assert_eq!(result6, false);
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

        let (result1, _) = regex.r#match(candidate1);
        let (result2, _) = regex.r#match(candidate2);
        let (result3, _) = regex.r#match(candidate3);
        let (result4, _) = regex.r#match(candidate4);
        let (result5, _) = regex.r#match(candidate5);
        let (result6, _) = regex.r#match(candidate6);
        let (result7, _) = regex.r#match(candidate7);
        let (result8, _) = regex.r#match(candidate8);
        let (result9, _) = regex.r#match(candidate9);
        let (result10, _) = regex.r#match(candidate10);
        let (result11, _) = regex.r#match(candidate11);
        let (result12, _) = regex.r#match(candidate12);

        assert_eq!(result1, false, "'hello world' test");
        assert_eq!(result2, false, "chaîne vide");
        assert_eq!(result3, false, "que des 'b'");
        assert_eq!(result4, true, "une seule occurence du pattern 'ab'");
        assert_eq!(result5, false, "un seul 'a'");
        assert_eq!(result6, false, "que des 'a'");
        assert_eq!(result7, true, "chaîne de 'a' puis un 'b'");

        assert_eq!(result8, false, "4 occurences du pattern 'ab'");
        assert_eq!(result9, false, "4 occurences du pattern 'aaaab'");

        assert_eq!(result10, false, "chaîne de 'a' puis une chaîne de 'b'");
        assert_eq!(result11, false, "un 'a' puis une chaîne de 'b'");
        assert_eq!(result12, false, "un seul 'b'");

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

        let (result1, _) = regex.r#match(candidate1);
        let (result2, _) = regex.r#match(candidate2);
        let (result3, _) = regex.r#match(candidate3);
        let (result4, _) = regex.r#match(candidate4);
        let (result5, _) = regex.r#match(candidate5);

        assert_eq!(result1, true, "3 occurences du pattern 'ab'");
        assert_eq!(result2, true, "3 occurences du pattern 'aaab'");
        assert_eq!(result3, false, "interruption du pattern 'ab'");
        assert_eq!(result4, true, "interruption du pattern 'ab' (bis)");
        assert_eq!(result5, true, "interruption du pattern 'aaab'");
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

        let (result1, _) = regex.r#match(candidate1);
        let (result2, _) = regex.r#match(candidate2);
        let (result3, _) = regex.r#match(candidate3);
        let (result4, _) = regex.r#match(candidate4);
        let (result5, _) = regex.r#match(candidate5);
        let (result6, _) = regex.r#match(candidate6);
        let (result7, _) = regex.r#match(candidate7);
        let (result8, _) = regex.r#match(candidate8);

        assert_eq!(result1, true);
        assert_eq!(result2, false);
        assert_eq!(result3, true);
        assert_eq!(result4, true);
        assert_eq!(result5, true);
        assert_eq!(result6, true);
        assert_eq!(result7, true);
        assert_eq!(result8, true);
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

        let (result1, _) = regex.r#match(candidate1);
        let (result2, _) = regex.r#match(candidate2);
        let (result3, _) = regex.r#match(candidate3);
        let (result4, _) = regex.r#match(candidate4);
        let (result5, _) = regex.r#match(candidate5);
        let (result6, _) = regex.r#match(candidate6);

        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, true);
        assert_eq!(result4, false);
        assert_eq!(result5, false);
        assert_eq!(result6, false);
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

        let (result1, _) = regex.r#match(candidate1);
        let (result2, _) = regex.r#match(candidate2);
        let (result3, _) = regex.r#match(candidate3);
        let (result4, _) = regex.r#match(candidate4);
        let (result5, _) = regex.r#match(candidate5);
        let (result6, _) = regex.r#match(candidate6);

        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
        assert_eq!(result4, false);
        assert_eq!(result5, true);
        assert_eq!(result6, true);

    }

    #[test]
    fn negation(){
        let regex = ChrRegex::new()
            .then(RegexElement::NoneOf(
                vec![
                    RegexElement::Set('a', 'z', Quantifier::Exactly(1)),
                    RegexElement::Set('A', 'Z', Quantifier::Exactly(1))
                ],
                Quantifier::OneOrMany
            ));

        let candidate1 = &"hello world".chars().collect::<Vec<char>>();
        let candidate2 = &"SELECT * FROM GROUP".chars().collect::<Vec<char>>();
        let candidate3 = &"{#!:->@".chars().collect::<Vec<char>>();
        let candidate4 = &"123.547".chars().collect::<Vec<char>>();
        let candidate5 = &"-10".chars().collect::<Vec<char>>();

        let (result1, _) = regex.r#match(candidate1);
        let (result2, _) = regex.r#match(candidate2);
        let (result3, _) = regex.r#match(candidate3);
        let (result4, _) = regex.r#match(candidate4);
        let (result5, _) = regex.r#match(candidate5);

        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, true);
        assert_eq!(result4, true);
        assert_eq!(result5, true);
    }

    #[test]
    fn matched_symbols(){
        let regex = ChrRegex::new()
            .then(RegexElement::Item('-', Quantifier::ZeroOrOne))
            .then(RegexElement::Set('0', '9', Quantifier::OneOrMany));

        let candidate1 = &"".chars().collect::<Vec<char>>();
        let candidate2 = &"  ".chars().collect::<Vec<char>>();
        let candidate3 = &"125".chars().collect::<Vec<char>>();
        let candidate4 = &"-57".chars().collect::<Vec<char>>();
        let candidate5 = &"-".chars().collect::<Vec<char>>();
        let candidate6 = &"0.78".chars().collect::<Vec<char>>();
        let candidate7 = &"hello world".chars().collect::<Vec<char>>();

        assert_eq!(regex.r#match(candidate1), (false, vec![]));
        assert_eq!(regex.r#match(candidate2), (false, vec![]));
        assert_eq!(regex.r#match(candidate3), (true, vec!['1', '2', '5']));
        assert_eq!(regex.r#match(candidate4), (true, vec!['-', '5', '7']));
        assert_eq!(regex.r#match(candidate5), (false, vec![]));
        assert_eq!(regex.r#match(candidate6), (false, vec![]));
        assert_eq!(regex.r#match(candidate7), (false, vec![]));
    }

}
