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
    element: RegexElement<T>,
    next: Option<Box<Regex<T>>>
}


impl<T:Symbol> Regex<T>{
    pub fn new(e: RegexElement<T>) -> Self{ Regex { element: e, next:None } }

    pub fn then(mut self, regex:Regex<T>) -> Box<Self>{
        self.next = Some(Box::new(regex));
        self.next.unwrap()

    }

    pub fn then_elem(mut self, e: RegexElement<T>) -> Box<Self> { self.then(Regex::new(e)) }

    
}
