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

// I know a way in https://rust-unofficial.github.io/too-many-lists/second-iter-mut.html
#[derive(Debug)]
struct IterMut<'a> {
    s: &'a mut OBString, //  I know Option<&'a mut IString> is ok, but why cannot this?
}
impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut String;
    // how to return ?
    // Please make a pull request or make a issue to help me fix this.
    fn next(&mut self) -> Option<Self::Item> {
        //    fn next(&mut self) -> Option<&mut String> {  // still error
        //self.s.as_mut().map(|v| &mut (**v).s)
        None
    }
}

impl<'a> IterMut<'a> {
    // 步骤拆分 // 仅仅看是否能够返回可修改借用 // 是能够做到的
    fn test_mut(&mut self) -> Option<&mut String> {
        let r = self.s.as_mut().map(|v| {
            &mut (**v).s
        });
        return r;
    }

    // 步骤拆分 // 仅仅看是否能够做到链表后移 // 做不到
    fn test_move_next(&mut self) {
        // `old_node` does not live long enough
//        let mut old_node = self.s.take();
//        old_node.as_mut().map(|e|{
//            self.s = &mut e.n;
//        });
    }

}

fn main() {
    let mut head = IString::new(String::from("aaa"));
    let mut tail = Box::new(IString::new(String::from("bbb")));
    head.n = Some(tail);

    for v in head.iter_mut() {
        println!("v={:?}", v);
        *v = format!("11{:?}", *v);
        println!("v={:?}", v);
    }
    println!("head={:?}", head);
    let mut v2 = head.iter_mut();
    v2.test_mut().map(|e| {
        *e = format!("--{}", e);
        println!("e={:?}", e)
    });

    println!("head={:?}", head);
}
