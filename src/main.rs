use followers::account::User;

fn main() {
    let a = User::new("Ross", "Bio 1");
    let b = User::new("Jeff", "Bio 2");
    let c = User::new("Jack", "Bio 3");
    let d = User::new("Nate", "Bio 4");
    let e = User::new("Erik", "Bio 5");

    a.follow_all(vec![&b, &c, &d, &e]);
    b.follow_all(vec![&a, &c]);
    c.follow(&e);
    d.follow(&b);

    dbg!(&a);
}
