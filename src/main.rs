use followers::account::User;

fn main() {
    let a = User::new("User 1", "Bio 1");
    let b = User::new("User 2", "Bio 2");
    let c = User::new("User 3", "Bio 3");
    let d = User::new("User 4", "Bio 4");
    let e = User::new("User 5", "Bio 5");

    a.follow_all(vec![&b, &c, &d, &e]);
    b.follow_all(vec![&a, &c]);
    c.follow(&e);
    d.follow(&b);

    dbg!(&a);
}
