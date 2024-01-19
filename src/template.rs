use askama::*;

#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate {}

#[derive(Template)]
#[template(path = "modal.html")]
pub struct ModalTemplate {}
