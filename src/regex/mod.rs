
use std::{hash::Hash, fmt::Debug};


pub trait Symbol : PartialEq+Eq+Hash+Clone+Debug{}

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


    fn match_t(candidate: Option<&[T]>, e:&RegexElement<T>) -> (bool, usize){
        return match e {
            RegexElement::Item(value, qt) => {
                let mut occurences = 0;

                if let Some(candidate) = candidate{
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
            RegexElement::Group(elements, qt) => {
                let mut valid = false;
                let mut ind = 0;

                if let Some(candidate) = candidate{

                    for element in elements{
                        let mut passed = 0;
                        (valid, passed) = Self::match_t(candidate.get(ind..), element);

                        if valid { ind += passed; }
                        else { return (false, ind); }
                    }

                    //Gérer la répétition de groupe
                    
                }


                (valid, ind)
            },
            RegexElement::Set(l, h, qt) => (false, 0)
        }
    }

    pub fn r#match(&self, candidate:&[T]) -> bool{
        let mut valid = false;
        let mut ind = 0;

        for element in &self.pattern{
            let mut passed = 0;
            (valid, passed) = Self::match_t(candidate.get(ind..), element);

            if valid { ind += passed; }
            else { break;}
        }
        
        if valid{
            return ind >= candidate.len();
            
        }

        valid
    }
    
}
