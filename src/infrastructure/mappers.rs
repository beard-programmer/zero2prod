use crate::domain;
use crate::routes::FormData;

pub fn form_data_to_domain(form_data: FormData) -> Result<domain::NewSubscriber, String> {
    let name = domain::SubscriberName::parse(form_data.name)?;
    let email = domain::SubscriberEmail::parse(form_data.email)?;
    Ok(domain::NewSubscriber { name, email })
}
