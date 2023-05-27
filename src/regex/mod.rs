
use std::{hash::Hash, fmt::Debug};


pub trait Symbol : PartialEq+Eq+PartialOrd+Hash+Clone+Debug{}

#[derive(Debug, Clone, PartialEq)]
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
    Set(T, T, Quantifier)

}

#[derive(Debug)]
pub struct Regex<T:Symbol>{
    pattern:Vec<RegexElement<T>>
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
                        if value == c { occurences+=1; } else{ break; }
                        
                    }
                }


                (match qt {
                    Quantifier::Exactly(n) => *n == occurences,
                    Quantifier::OneOrMany => occurences >= 1,
                    Quantifier::ZeroOrMany => occurences >= 0,
                    Quantifier::ZeroOrOne => occurences == 0 || occurences == 1
                }, occurences)
            },

            RegexElement::Set(low, high, qt) => {
                let mut occurences = 0;

                if let Some(candidate) = candidate{

                    for c in candidate{
                        if low <= c && c <= high { occurences+=1; } else{ break; }
                    }
                }
                

                (match qt {
                    Quantifier::Exactly(n) => *n == occurences,
                    Quantifier::OneOrMany => occurences >= 1,
                    Quantifier::ZeroOrMany => occurences >= 0,
                    Quantifier::ZeroOrOne => occurences == 0 || occurences == 1
                }, occurences)
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

            RegexElement::Group(elements, qt) => {
                let mut valid = false;
                let mut ind = 0;
                let mut occurences = 0;

                if let Some(candidate) = candidate{

                    loop{

                        for element in elements{
                            let mut passed = 0;
                            (valid, passed) = Self::match_element(candidate.get(ind..), element);
    
                            if valid { ind += passed; }
                            else { break; }
                        }
    
                        if valid { occurences += 1; }
    
    
                        let (should_repeat, passed) = Self::match_element(candidate.get(ind..), elements.get(0).unwrap());
    
                        if !should_repeat { break; } else if passed == 0 { break; }
    
                    }
                }

                

                (match qt{
                    Quantifier::Exactly(n) => *n == occurences,
                    Quantifier::OneOrMany => occurences >= 1,
                    Quantifier::ZeroOrMany => occurences >= 0,
                    Quantifier::ZeroOrOne => occurences == 0 || occurences == 1
                }, ind)
            }
        }
    }

    pub fn r#match(&self, candidate:&[T]) -> bool{
        let mut valid = false;
        let mut ind = 0;

        for element in &self.pattern{
            let mut passed = 0;
            (valid, passed) = Self::match_element(candidate.get(ind..), element);

            if valid { ind += passed; }
            else { break;}
        }
        
        valid && ind >= candidate.len()
    }
    
}
