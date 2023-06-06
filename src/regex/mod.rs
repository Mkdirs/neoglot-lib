use std::{hash::Hash, fmt::Debug};

/// A symbol is the smallest bit of information a [regex](Regex) can work with
/// 
/// Implement this trait for any type you want to be supported with [regex](Regex)
/// 
/// # Examples
/// ```rust
/// use crate::neoglot_lib::regex::Symbol;
/// 
/// // Implement the needed traits
/// #[derive(PartialEq, Eq, PartialOrd, Hash, Clone, Debug)]
/// struct Foo{
/// }
/// 
/// // Now your type can be used to do regular expressions
/// impl Symbol for Foo {}
/// ```
pub trait Symbol : PartialEq+Eq+PartialOrd+Hash+Clone+Debug{}

impl Symbol for char{}

#[derive(Debug, Clone, PartialEq, Copy)]
/// A Quantifier is the number of occurences of a [RegexElement]
pub enum Quantifier{
    /// The [RegexElement] must have the exact amount of occurences given
    /// 
    /// This is equivalent to '{n}'
    Exactly(usize),

    /// The [RegexElement] must have at least one occurence
    /// 
    /// This is equivalent to '+'
    OneOrMany,

    /// The [RegexElement] may have several occurences or none
    /// 
    /// This is equivalent to '*'
    ZeroOrMany,

    /// The [RegexElement] may have one occurence or none
    /// 
    /// This is equivalent to '?'
    ZeroOrOne
}
#[derive(Debug, Clone, PartialEq)]
/// RegexElements are what make up a [Regex]
/// 
/// They indicate what set of [Symbols](Symbol) are expected
pub enum  RegexElement<T:Symbol>{
    /// A single [Symbol]
    Item(T, Quantifier),

    /// A group of other RegexElements
    /// 
    /// A Group is valid only if all elements inside are valid
    /// 
    /// This is equivalent to '(...)'
    Group(Vec<RegexElement<T>>, Quantifier),

    /// Convenience way of doing alternation
    /// 
    /// As suggested it is valid if any of its elements are valid
    /// 
    /// This is equivalent to 'a|b|c|...|z'
    AnyOf(Vec<RegexElement<T>>),

    /// Convenience way of doing negation
    /// 
    /// As suggested it is valid only if none of its elements are valid
    /// 
    /// So it accepts anything except the elements inside
    /// 
    /// This is equivalent to '[\^abc...]'
    NoneOf(Vec<RegexElement<T>>, Quantifier),

    /// The first parameter is the lower end of the set
    /// 
    /// The second parameter is the upper end of the set
    /// 
    /// It accepts any [Symbol] that is inside the set
    /// 
    /// This is equivalent to '[..-..]'
    Set(T, T, Quantifier)

}

#[derive(Debug)]
/// Describes a pattern of [Symbols](Symbol)
/// 
/// # Examples
/// ```rust
/// use crate::neoglot_lib::regex::{Quantifier, Regex, RegexElement};
/// 
/// let regex = Regex::<char>::new()
///         .then(RegexElement::Item('-', Quantifier::ZeroOrOne))
///         .then(RegexElement::Set('0', '9', Quantifier::OneOrMany));
/// 
/// // We can use [' ', ' '] directly but i can't be bothered
/// let candidate1 = &"  ".chars().collect::<Vec<char>>();
/// let candidate2 = &"125".chars().collect::<Vec<char>>();
/// let candidate3 = &"-57".chars().collect::<Vec<char>>();
/// let candidate4 = &"-".chars().collect::<Vec<char>>();
/// let candidate5 = &"0.78".chars().collect::<Vec<char>>();
/// 
/// // Simple matching of a pattern
/// assert_eq!(regex.r#match(candidate1), false);
/// assert_eq!(regex.r#match(candidate2), true);
/// assert_eq!(regex.r#match(candidate3), true);
/// assert_eq!(regex.r#match(candidate4), false);
/// assert_eq!(regex.r#match(candidate5), false);
/// 
/// let result1:(&[char], &[char]) = (&[], &[' ', ' ']);
/// let result2:(&[char], &[char]) = (&['1', '2', '5'], &[]);
/// let result3:(&[char], &[char]) = (&['-', '5', '7'], &[]);
/// let result4:(&[char], &[char]) = (&['-'], &[]);
/// let result5:(&[char], &[char]) = (&['0'], &['.', '7', '8']);
/// 
/// // Taking the first matching symbols and the rest
/// // This is usefull for dismantling a large set of symbols into token for instance
/// assert_eq!(regex.split_first(candidate1), result1);
/// assert_eq!(regex.split_first(candidate2), result2);
/// assert_eq!(regex.split_first(candidate3), result3);
/// assert_eq!(regex.split_first(candidate4), result4);
/// assert_eq!(regex.split_first(candidate5), result5);
/// 
/// ```
pub struct Regex<T:Symbol>{
    pattern:Vec<RegexElement<T>>
}

// Returns if a given number match a quantifier
fn match_quantifier(num:usize, quantifier:&Quantifier) -> bool{
    match quantifier {
        Quantifier::Exactly(n) => *n == num,
        Quantifier::OneOrMany => num >= 1,
        Quantifier::ZeroOrMany => true,
        Quantifier::ZeroOrOne => num == 0 || num == 1
    }
}

// Returns if a set of Symbols match a single RegexElement
// and the number of Symbols that has been read
fn match_element<T:Symbol>(candidate: Option<&[T]>, e:&RegexElement<T>) -> (bool, usize){
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
                (valid, passed) = match_element(candidate, element);

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
                        let (matched, _) = match_element(Some(&[c.clone()]), element);

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
                        (valid, passed) = match_element(candidate.get(ind..), element);
                        

                        if valid { ind += passed; }
                        else { break; }
                    }

                    if valid { occurences += 1; }


                    let (should_repeat, passed) = match_element(candidate.get(ind..), elements.get(0).unwrap());

                    if !should_repeat { break; } else if passed == 0 { break; }

                }
            }

            

            (match_quantifier(occurences, qt), ind)
        }
    }
}



impl<T:Symbol> Regex<T>{

    /// Creates a new Regex
    pub fn new() -> Self{ Regex { pattern: vec![] } }

    ///Adds an [element](RegexElement) to the regex
    pub fn then(mut self, e:RegexElement<T>) -> Self{
        self.pattern.push(e);
        self

    }


    /// Verifies if a set of [Symbols](Symbol) match the pattern of this regex
    pub fn r#match(&self, candidate:&[T]) -> bool{
        let mut valid = false;
        let mut ind = 0;

        for element in &self.pattern{
            let passed:usize;
            (valid, passed) = match_element(candidate.get(ind..), element);

            if valid { ind += passed; }
            else { break;}
        }

        valid && ind >= candidate.len()
    }

    /// Splits a set of [symbols](Symbol) into two:
    /// the first matched [symbols](Symbol)
    /// and the rest
    pub fn split_first<'a>(&self, candidate: &'a[T]) -> (&'a [T], &'a [T]){
        let mut ind = 0;

        for element in &self.pattern {
            let (valid, passed) = match_element(candidate.get(ind..), element);

            if valid { ind += passed; }
            else{ break; }
        }


        let (matched, others) = candidate.split_at(ind);

        (matched, others)

    }
    
}
