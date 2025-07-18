use inquire::CustomType;

use crate::prelude::*;

fn ask_for_proto_email_atom_template(part: &str, default: &TemplatePart) -> Result<TemplatePart> {
    CustomType::<TemplatePart>::new(&format!("Email template for {}", part))
        .with_help_message(&TemplatePart::tutorial())
        .with_default(default.clone())
        .prompt()
        .map_err(|e| Error::EmailAtomTemplateError {
            underlying: e.to_string(),
        })
}

pub fn ask_for_template(default: &Template) -> Result<Template> {
    let subject = ask_for_proto_email_atom_template("subject", default.subject_format())?;
    let body = ask_for_proto_email_atom_template("body", default.body_format())?;
    Ok(Template::builder()
        .subject_format(subject)
        .body_format(body)
        .build())
}
