use std::hash::Hash;


pub trait Symbol : PartialEq+Eq+Hash{}

#[derive(Debug)]
pub enum Quantifier{
    Exactly(usize),
    OneOrMany,
    ZeroOrMany,
    ZeroOrOne
}
#[derive(Debug)]
pub enum RegexElement<T:Symbol>{
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

    fn match_item(value:&T, quantifier:&Quantifier, candidate: &[T]) -> (bool, usize){
        let mut occurences = 0;
        for c in candidate{
            if value == c { occurences+=1 } else{ break; }
        }

        return (match quantifier{
            Quantifier::Exactly(qt) => *qt == occurences,
            Quantifier::OneOrMany => occurences >= 1,
            Quantifier::ZeroOrMany => occurences >= 0,
            Quantifier::ZeroOrOne => occurences == 0 || occurences == 1
        }, occurences)


    }

    pub fn r#match(&self, candidate: &[T]) -> bool{
        let mut pattern_iterator = self.pattern.iter();
        let mut candidate_iterator = candidate.iter();

        let mut valid = false;
        let mut symbol = candidate_iterator.next();
        let mut element = pattern_iterator.next();
        let mut count = 0;

        loop{
            match symbol{
                Some(_) =>{
                    if let Some(e) = element{
                        match e {
                            RegexElement::Item(value, quantifier) => {
                                let mut passed = 0;
                                (valid, passed) = Regex::match_item(value, quantifier, candidate);
                                count += passed;

                                if valid && count == 0{
                                    element = pattern_iterator.next();
                                }else if valid && count > 0 {
                                    symbol =  candidate_iterator.nth(count);
                                }
                            },
                            RegexElement::Set(_, _, _) => { valid = false },
                            RegexElement::Group(_, _) => { valid = false }
                        }

                        if !valid { break; }

                        
                    }
                },
                None => break
            }
        }
        valid
    }

    
}
