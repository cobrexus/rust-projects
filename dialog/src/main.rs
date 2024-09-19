use rfd::{MessageButtons, MessageDialog, MessageLevel};

fn main() {
    MessageDialog::new()
        .set_title("title")
        .set_buttons(MessageButtons::Ok)
        .set_description("description")
        .set_level(MessageLevel::Warning)
        .show();
}
