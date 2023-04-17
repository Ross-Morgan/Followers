use std::cell::RefCell;
use std::fmt::Debug;

use chrono::prelude::*;
use uuid::Uuid;

type UserRef<'a> = &'a User<'a>;

#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct User<'a> {
    username: RefCell<String>,
    biography: RefCell<String>,
    uuid: Uuid,
    date_created: DateTime<Utc>,

    followers: RefCell<Vec<UserRef<'a>>>,
    following: RefCell<Vec<UserRef<'a>>>,
}

impl<'a> User<'a> {
    pub fn new<T: ToString, U: ToString>(username: T, biography: U) -> Self {
        Self {
            username: RefCell::new(username.to_string()),
            biography: RefCell::new(biography.to_string()),
            uuid: Uuid::new_v4(),
            date_created: chrono::offset::Utc::now(),
            followers: RefCell::new(vec![]),
            following: RefCell::new(vec![]),
        }
    }

    pub fn get_username(&self) -> String {
        self.username.borrow().clone()
    }
    pub fn get_biography(&self) -> String {
        self.biography.borrow().clone()
    }

    pub fn get_follower_count(&self) -> usize { self.followers.borrow().len() }
    pub fn get_following_count(&self) -> usize { self.following.borrow().len() }

    pub fn get_uuid(&self) -> &Uuid { &self.uuid }
    pub fn get_date_created(&self) -> &DateTime<Utc> { &self.date_created }
}

impl<'a> User<'a> {
    pub fn is_following(&self, user: UserRef<'a>) -> bool {
        self.following.borrow().contains(&user)
    }

    pub fn is_followed_by(&self, user: UserRef<'a>) -> bool {
        self.followers.borrow().contains(&user)
    }

    pub fn set_username<T: ToString>(&self, new_username: T) {
        let mut un = self.username.borrow_mut();

        un.clear();
        un.push_str(new_username.to_string().as_str());
    }

    pub fn set_biography<T: ToString>(&self, new_biography: T) {
        let mut bio = self.biography.borrow_mut();

        bio.clear();
        bio.push_str(new_biography.to_string().as_str());
    }

    pub fn follow(&self, user: UserRef<'a>) {
        let mut following = self.following.borrow_mut();

        match following.contains(&user) {
            true => println!("Already following {:?}", user.get_username()),
            false => following.push(user),
        };
    }

    pub fn follow_all(&self, users: Vec<UserRef<'a>>) {
        let mut following = self.following.borrow_mut();

        for user in users {
            match following.contains(&user) {
                true => println!("Already following {:?}", user.get_username()),
                false => following.push(user),
            }
        }
    }

    pub fn unfollow(&self, user: UserRef<'a>) {
        let mut following = self.following.borrow_mut();

        //TODO Match if contains

        match following.iter().position(|&u| u == user) {
            None => println!("Not following: {:?}", user),
            Some(i) => {
                following.remove(i);
            }
        };
    }

    pub fn unfollow_all(&self, users: Vec<UserRef<'a>>) {
        users
            .into_iter()
            .for_each(|u| self.unfollow(u));
    }
}

impl<'a> Debug for User<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("{{\n")?;
        f.write_fmt(format_args!("  Username: {}\n", self.get_username()))?;
        f.write_fmt(format_args!("  Biography: {}\n", self.get_biography()))?;
        f.write_fmt(format_args!("  Followers: {}\n", self.get_follower_count()))?;
        f.write_fmt(format_args!("  Following: {}\n", self.get_following_count()))?;
        f.write_str("}}\n")?;

        Ok(())
    }
}

#[macro_export]
macro_rules! follow {
    ($target:ident <= $($x:expr),+ $(,)?) => {
        let new_followers = vec![$(&$x),+];
        $target.follow_all(new_followers);
    }
}

// fn main() {
//     let a = User::new("User 1", "Bio 1");
//     let b = User::new("User 2", "Bio 2");
//     let c = User::new("User 3", "Bio 3");
//     let d = User::new("User 4", "Bio 4");
//     let e = User::new("User 5", "Bio 5");

//     a.follow_all(vec![&b, &c, &d, &e]);
//     b.follow_all(vec![&a, &c]);
//     c.follow(&e);
//     d.follow(&b);

//     dbg!(&a);
// }
