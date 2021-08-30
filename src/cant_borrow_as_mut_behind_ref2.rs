use crate::types::Bigger;
use std::collections::HashMap;

///
/// Collections as Vec/HashMap/HashSet and so on, both provided  get() and get_mut()
/// '&' is equal to 'borrow()', just a syntax sugar.
/// '&mut' is equal to 'borrow_mut()'.
/// Borrow sometimes effect same as Ref, so sometimes,  as_ref() work same as borrow(), to see the
/// difference: https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
///

#[test]
fn borrow_into() {
    let mut x1: HashMap<String, Bigger> = HashMap::new();

    x1.insert("he".to_string(), Bigger::default());
    x1.insert("he2".to_string(), Bigger::default());

    let mut x2: HashMap<String, HashMap<String, Bigger>> = HashMap::new();

    x2.insert("ss".to_string(), x1);
    // here must using get_mut() to get mutable element

    // ERROR: cannot borrow data in a `&` reference as mutable
    x2.get_mut("ss").unwrap().get_mut("he").unwrap().b1.x = 3;

    // ERROR : can't assign to field of immutable binding
    //x2.get("ss").unwrap().get("he").borrow_mut().unwrap().b1.x = 3;
    //x2.get("ss").unwrap().get("he").unwrap().borrow_mut().b1.x = 3;
}
