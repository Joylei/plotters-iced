use iced_native::mouse::Event;
use iced_native::Point;

/// Signature for the callback that ChartWidget can trigger when a mouse event
/// happens inside its layout. Return None if the mouse event is not being
/// handled by this callback.
///
/// # Arguments
///
/// * The type of mouse event
/// * The cursor position during the event, relative to the widget origin. Use
///   the chart coord spec to transform this point into the chart's data coordinates.
pub type MouseEventCallback<Message> = Box<dyn Fn(Event, Point) -> Option<Message>>;
