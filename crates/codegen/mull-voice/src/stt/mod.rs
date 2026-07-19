//! Palmshed Speech-to-Text: streaming `wss://api.palmshed.ai/v1/stt`.

mod streaming;
mod types;

pub use streaming::{StreamingSttEvent, StreamingSttSession};
pub use types::{SttServerEvent, SttTranscriptPartial};
