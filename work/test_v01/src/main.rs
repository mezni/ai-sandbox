use fake::faker::name::raw::*;
use fake::locales::*;
use fake::{Dummy, Fake, Faker};
use std::fmt;
#[derive(Debug)]
enum AccountStatus {
    Active,
    Suspended,
    Terminated,
}

impl fmt::Display for AccountStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccountStatus::Active => write!(f, "Active"),
            AccountStatus::Suspended => write!(f, "Suspended"),
            AccountStatus::Terminated => write!(f, "Terminated"),
        }
    }
}
#[derive(Debug)]
struct Subscriber {
    subscriber_id: u32,
    name: String,
    account_status: AccountStatus,
}

impl fmt::Display for Subscriber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Subscriber ID: {}\nName: {}\nAccount Status: {}",
            self.subscriber_id, self.name, self.account_status
        )
    }
}

fn main() -> fmt::Result {
    let mut subscribers: Vec<Subscriber> = Vec::new();

    for i in 1..11 {
        let name: String = Name(EN).fake();
        let subscriber = Subscriber {
            subscriber_id: i,
            name: name,
            account_status: AccountStatus::Active,
        };
        subscribers.push(subscriber);
    }

    for subscriber in subscribers {
        println!("{:?}", subscriber);
    }
    Ok(())
    //    let name: String = Name(EN).fake();
    //    println!("name {:?}", name);
}
