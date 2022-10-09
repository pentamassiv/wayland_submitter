// Imports from other crates
use wayland_client::{protocol::wl_seat::WlSeat, Display, EventQueue, GlobalManager};
use wayland_protocols::misc::zwp_input_method_v2::client::zwp_input_method_manager_v2::ZwpInputMethodManagerV2;
use zwp_virtual_keyboard::virtual_keyboard_unstable_v1::zwp_virtual_keyboard_manager_v1::ZwpVirtualKeyboardManagerV1;

// The type declarations are not necessary but make the code easier to read
type VirtualKeyboardMgr = wayland_client::Main<ZwpVirtualKeyboardManagerV1>;
type InputMethodMgr = wayland_client::Main<ZwpInputMethodManagerV2>;

/// Get the 'GlobalManager' and the 'EventQueue'
fn get_wl_global_mgr(display: Display) -> (EventQueue, GlobalManager) {
    // Create the event queue
    let mut event_queue = display.create_event_queue();
    // Attach the display
    let attached_display = display.attach(event_queue.token());

    let global_mgr = GlobalManager::new(&attached_display);

    // sync_roundtrip is a special kind of dispatching for the event queue.
    // Rather than just blocking once waiting for replies, it'll block
    // in a loop until the server has signalled that it has processed and
    // replied accordingly to all requests previously sent by the client.
    //
    // In this case, this allows to be sure that after this call returns,
    // the full list of globals was received.
    event_queue
        .sync_roundtrip(
            // No global state is used
            &mut (),
            // The only object that can receive events is the WlRegistry, and the
            // GlobalManager already takes care of assigning it to a callback, so
            // no orphan events can be received at this point
            |_, _, _| unreachable!(),
        )
        .unwrap();
    (event_queue, global_mgr)
}

/// Tries to get the manager for the protocols input_method and virtual_keyboard
/// It returns 'None' if the compositor does not undestand a protocol
fn try_get_mgrs(
    global_mgr: &GlobalManager,
) -> (Option<VirtualKeyboardMgr>, Option<InputMethodMgr>) {
    let mut virtual_keyboard_option = None;
    let mut input_method_mgr_option = None;
    if let Ok(vk_mgr) = global_mgr.instantiate_exact::<ZwpVirtualKeyboardManagerV1>(1) {
        virtual_keyboard_option = Some(vk_mgr);
    } else {
        warn!("Your wayland compositor does not understand the wp_virtual_keyboard protocol. Entering any keycode will fail");
    }
    if let Ok(im_mgr) = global_mgr.instantiate_exact::<ZwpInputMethodManagerV2>(1) {
        input_method_mgr_option = Some(im_mgr);
    } else {
        warn!("Your wayland compositor does not understand the wp_virtual_keyboard protocol. Only keycodes can be entered");
    }
    (virtual_keyboard_option, input_method_mgr_option)
}

/// Initializes the wayland connection and returns the wayland objects needed to submit text and keycodes
pub fn init_wayland() -> (
    EventQueue,
    WlSeat,
    Option<VirtualKeyboardMgr>,
    Option<InputMethodMgr>,
) {
    // Get wayland display and WlSeat
    let display = Display::connect_to_env()
        .or_else(|_| Display::connect_to_name("wayland-0"))
        .unwrap();
    // Get the event queue and the GlobalManager
    let (event_queue, global_mgr) = get_wl_global_mgr(display);
    let seat = global_mgr.instantiate_exact::<WlSeat>(7).unwrap();
    let seat: WlSeat = WlSeat::from(seat.as_ref().clone());
    // Try to get the manager for the input_method and virtual_keyboard protocol
    let (vk_mgr, im_mgr) = try_get_mgrs(&global_mgr);
    info!("Wayland connection and objects initialized");
    (event_queue, seat, vk_mgr, im_mgr)
}
// */
