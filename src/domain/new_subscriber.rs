use crate::domain;

#[derive(Debug)]
pub struct NewSubscriber {
    pub email: domain::SubscriberEmail,
    pub name: domain::SubscriberName,
}
