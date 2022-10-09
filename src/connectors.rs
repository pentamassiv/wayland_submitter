use wayland_protocols::unstable::text_input::v3::client::zwp_text_input_v3::{
    ContentHint, ContentPurpose,
};
use zwp_input_method_service::{HintPurpose, IMVisibility, ReceiveSurroundingText};

#[derive(Default, Debug, Clone, Copy)]
pub struct ContentConnector {}

impl ContentConnector {
    /// Creates a new ContentConnector
    pub fn new() -> ContentConnector {
        ContentConnector {}
    }
}

/// Implements the ReceiveSurroundingText trait from the zwp_input_method_service crate to notify about changes to the surrounding text
impl ReceiveSurroundingText for ContentConnector {
    fn text_changed(&self, string_left_of_cursor: String, string_right_of_cursor: String) {
        info!(
            "text changed: {}|{}",
            string_left_of_cursor, string_right_of_cursor
        );
    }
}

/// This is a connection to send messages to the UI
/// It is used by the input_method service to notify the UI about requested changes to the visibility or content hint/purpose
#[derive(Default, Debug, Clone, Copy)]
pub struct UIConnector {}

impl UIConnector {
    /// Creates a new UIConnector
    pub fn new() -> UIConnector {
        UIConnector {}
    }
    // Send the message to the UI
    //pub fn emit(&self, msg: Msg) {
    // self.message_pipe.stream().emit(msg)
    //}
}

/// Implements the KeyboardVisibility trait from the zwp_input_method_service crate to notify the UI about requested changes to the visibility
impl IMVisibility for UIConnector {
    fn activate_im(&self) {
        info!("Requested to show the keyboard");
    }
    fn deactivate_im(&self) {
        info!("Requested to hide the keyboard");
    }
}

/// Implements the KeyboardVisibility trait from the zwp_input_method_service crate to notify the UI about requested changes to the content hint/purpose
impl HintPurpose for UIConnector {
    fn set_hint_purpose(&self, content_hint: ContentHint, content_purpose: ContentPurpose) {
        info!(
            "Requested to change to ContentHint: {:?} and  ContentPurpose: {:?}",
            content_hint, content_purpose
        );
    }
}
