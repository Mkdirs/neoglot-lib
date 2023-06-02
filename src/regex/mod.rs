use std::{hash::Hash, fmt::Debug};


pub trait Symbol : PartialEq+Eq+PartialOrd+Hash+Clone+Debug{}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Quantifier{
    Exactly(usize),
    OneOrMany,
    ZeroOrMany,
    ZeroOrOne
}
#[derive(Debug, Clone, PartialEq)]
pub enum  RegexElement<T:Symbol>{
    Item(T, Quantifier),
    Group(Vec<RegexElement<T>>, Quantifier),
    AnyOf(Vec<RegexElement<T>>),
    NoneOf(Vec<RegexElement<T>>, Quantifier),
    Set(T, T, Quantifier)

}

#[derive(Debug)]
pub struct Regex<T:Symbol>{
    pattern:Vec<RegexElement<T>>
}


fn match_quantifier(num:usize, quantifier:&Quantifier) -> bool{
    match quantifier {
        Quantifier::Exactly(n) => *n == num,
        Quantifier::OneOrMany => num >= 1,
        Quantifier::ZeroOrMany => true,
        Quantifier::ZeroOrOne => num == 0 || num == 1
    }
}



impl<T:Symbol> Regex<T>{
    pub fn new() -> Self{ Regex { pattern: vec![] } }

    pub fn then(mut self, e:RegexElement<T>) -> Self{
        self.pattern.push(e);
        self

    }

    
    fn match_element(candidate: Option<&[T]>, e:&RegexElement<T>) -> (bool, usize){
        return match e {
            RegexElement::Item(value, qt) => {
                let mut occurences = 0;

                if let Some(candidate) = candidate {
                    for c in candidate{
                        if value == c { 
                            occurences+=1;
                            
                            match qt {
                                Quantifier::Exactly(n) => if *n == occurences { break; },
                                Quantifier::OneOrMany => continue,
                                Quantifier::ZeroOrMany => continue,
                                Quantifier::ZeroOrOne => break
                            }
                        }
                        else{ break; }
                        

                    }
                }


                (match_quantifier(occurences, qt), occurences)
            },

            RegexElement::Set(low, high, qt) => {
                let mut occurences = 0;

                if let Some(candidate) = candidate{

                    for c in candidate{
                        if low <= c && c <= high { 
                            occurences+=1; 

                            match qt {
                                Quantifier::Exactly(n) => if *n == occurences { break; },
                                Quantifier::OneOrMany => continue,
                                Quantifier::ZeroOrMany => continue,
                                Quantifier::ZeroOrOne => break
                            }
                        } 
                        else{ break; }
                    }
                }
                

                (match_quantifier(occurences, qt), occurences)
            },

            RegexElement::AnyOf(elements) => {
                let mut valid = false;
                let mut passed = 0;

                for element in elements{
                    (valid, passed) = Self::match_element(candidate, element);

                    if valid { break; }
                }

                (valid, passed)
            },

            RegexElement::NoneOf(elements, qt) => {

                let mut occurences = 0;

                if let Some(candidate) = candidate{

                    for c in candidate{
                        let mut valid = false;
                        for element in elements{
                            let (matched, _) = Self::match_element(Some(&[c.clone()]), element);

                            valid = !matched;
                            if !valid { break; }

                        }
                        if valid {
                            occurences += 1;

                            match qt {
                                Quantifier::Exactly(n) => if *n == occurences { break; },
                                Quantifier::OneOrMany => continue,
                                Quantifier::ZeroOrMany => continue,
                                Quantifier::ZeroOrOne => break
                            }
                        }
                        else{ break; }
                    }
                }
                

                (match_quantifier(occurences, qt), occurences)
            },

            RegexElement::Group(elements, qt) => {
                let mut valid = false;
                let mut ind = 0;
                let mut occurences = 0;

                if let Some(candidate) = candidate{

                    loop{

                        for element in elements{
                            let passed:usize;
                            (valid, passed) = Self::match_element(candidate.get(ind..), element);
                            
    
                            if valid { ind += passed; }
                            else { break; }
                        }
    
                        if valid { occurences += 1; }
    
    
                        let (should_repeat, passed) = Self::match_element(candidate.get(ind..), elements.get(0).unwrap());
    
                        if !should_repeat { break; } else if passed == 0 { break; }
    
                    }
                }

                

                (match_quantifier(occurences, qt), ind)
            }
        }
    }

    pub fn r#match(&self, candidate:&[T]) -> bool{
        let mut valid = false;
        let mut ind = 0;

        for element in &self.pattern{
            let passed:usize;
            (valid, passed) = Self::match_element(candidate.get(ind..), element);

            if valid { ind += passed; }
            else { break;}
        }
        
        valid && ind >= candidate.len()
    }
    
}
