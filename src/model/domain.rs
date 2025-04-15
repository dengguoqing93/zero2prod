use unicode_segmentation::UnicodeSegmentation;
use validator::ValidateEmail;

pub struct SubscribeName(pub String);
impl SubscribeName {
    pub fn parse(s: String) -> Result<SubscribeName, String> {
        let is_empty = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));
        if is_empty || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid name", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<String> for SubscribeName {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

pub struct SubscribeEmail(pub String);
impl SubscribeEmail {
    pub fn parse(s: String) -> Result<SubscribeEmail, String> {
        if !ValidateEmail::validate_email(&s) {
            Err(format!("{} is not a valid email", s))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<String> for SubscribeEmail {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

pub struct NewSubscriber {
    pub email: SubscribeEmail,
    pub name: SubscribeName,
}

impl NewSubscriber {
    pub fn parse(email: String, name: String) -> Result<NewSubscriber, String> {
        if SubscribeEmail::parse(email.clone()).is_err()
            || SubscribeName::parse(name.clone()).is_err()
        {
            Err("Invalid email or name".to_string())
        } else {
            Ok(NewSubscriber {
                email: SubscribeEmail::parse(email.clone()).unwrap(),
                name: SubscribeName::parse(name.clone()).unwrap(),
            })
        }
    }
}
