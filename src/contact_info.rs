
enum PrimaryContactInfo {
    Phone(u32),
    Email(String),
}

impl PrimaryContactInfo {
    fn print(&self) {
        match self {
            PrimaryContactInfo::Phone(number) => {
                println!("The contact's phone number is {}", number);
            },
            PrimaryContactInfo::Email(email) => {
                println!("The contacts email address is {}", email);
            }
        }
    }
}
