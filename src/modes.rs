use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define a struct to represent the shared properties of the modes.
#[derive(Serialize, Deserialize, Debug)]
pub struct ModeConfig {
    pub options_sets: Vec<String>,
    // Include any other shared properties here.
}

// Define the variants of the Mode enum.
#[derive(Serialize, Deserialize, Debug)]
pub enum Mode {
    Creative(ModeConfig),
    Balanced(ModeConfig),
    Precise(ModeConfig),
    // Add any other mode-specific structs here, if needed.
}

// Define functions or constructors for creating instances of each mode.
impl Mode {
    pub fn new_creative() -> Self {
        Mode::Creative(ModeConfig {
            options_sets: vec![
                "nlu_direct_response_filter".to_string(),
                "deepleo".to_string(),
                "disable_emoji_spoken_text".to_string(),
                "responsible_ai_policy_235".to_string(),
                "enablemm".to_string(),
                "dv3sugg".to_string(),
                "autosave".to_string(),
                "iyxapbing".to_string(),
                "iycapbing".to_string(),
                "h3imaginative".to_string(),
                "clgalileo".to_string(),
                "gencontentv3".to_string(),
                "intmvallowlist".to_string(),
                "logprobsc".to_string(),
                "paywallv2".to_string(),
                "2tlocretbn".to_string(),
                "eredirecturl".to_string()
            ]
        })
    }

    pub fn new_precise() -> Self {
        Mode::Precise(ModeConfig {
            options_sets: vec![
                "nlu_direct_response_filter".to_string(),
                "deepleo".to_string(),
                "disable_emoji_spoken_text".to_string(),
                "responsible_ai_policy_235".to_string(),
                "enablemm".to_string(),
                "dv3sugg".to_string(),
                "autosave".to_string(),
                "iyxapbing".to_string(),
                "iycapbing".to_string(),
                "h3precise".to_string(),
                "intmvallowlist".to_string(),
                "logprobsc".to_string(),
                "paywallv2".to_string(),
                "2tlocretbn".to_string(),
                "eredirecturl".to_string(),
                "clgalileo".to_string(),
                "gencontentv3".to_string()
            ]
        })
    }

    pub fn new_balanced() -> Self {
        Mode::Balanced(ModeConfig {
            options_sets: vec![
                "nlu_direct_response_filter".to_string(),
                "deepleo".to_string(),
                "disable_emoji_spoken_text".to_string(),
                "responsible_ai_policy_235".to_string(),
                "enablemm".to_string(),
                "dv3sugg".to_string(),
                "autosave".to_string(),
                "iyxapbing".to_string(),
                "iycapbing".to_string(),
                "galileo".to_string(),
                "intmvallowlist".to_string(),
                "logprobsc".to_string(),
                "paywallv2".to_string(),
                "2tlocretbn".to_string(),
                "eredirecturl".to_string(),
                "saharagenconv5".to_string(),
                "glfluxv15".to_string()
              ],
        })
    }
}