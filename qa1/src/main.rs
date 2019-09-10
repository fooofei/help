#[derive(Debug)]
struct IString {
    s: String,
    n: OBString,
}
type OBString = Option<Box<IString>>;

impl IString {
    pub fn new(s: String) -> IString {
        IString { s: s, n: None }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut { s: &mut self.n }
    }
}

struct IterMut<'a> {
    s: &'a mut OBString,
}
impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut String;
    // how to return ?
    // Please make a pull request or make a issue to help me fix this.
    fn next(&mut self) -> Option<Self::Item> {
        self.s.as_mut().map(|v| &mut (**v).s)
    }
}

fn main() {
    let mut v1 = Some(Box::new(IString::new(String::from("a"))));
    let iter = v1.iter_mut();
    for v in iter {
        println!("v={:?}", v);
    }
}
