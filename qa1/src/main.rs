
#[derive(Debug)]
struct IString {
    s: String,
    pub n: Option<Box<IString>>,
}

impl IString {
    pub fn new(s: String) -> IString {
        IString { s: s, n: None }
    }
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        IterMut { s: Some(self) }
    }
}

struct IterMut<'a> {
    s: Option<&'a mut IString>,
}

impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut String;
    // how to return ?
    // Please make a pull request or make a issue to help me fix this.
    fn next(&mut self) -> Option<Self::Item> {
        self.s.take().map(|s| {
            self.s = s.n.as_mut().map(|x| &mut **x);
            &mut s.s
        })
    }
}

fn main() {
    let mut v1 = IString::new(String::from("a"));
    v1.n = Some(Box::new(IString::new(String::from("b"))));
    let iter = v1.iter_mut();
    for v in iter {
        println!("v={:?}", v);
        v.push_str("__")
    }
    
    let iter = v1.iter_mut();
    for v in iter {
        println!("v={:?}", v);
    }
}
