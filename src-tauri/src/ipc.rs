use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IpcActionRenderPointsPayloadPoint {
    pub coords: [u32; 2],
    pub bg_color: String,
    pub fg_color: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IpcActionRenderPoints {
    pub display_action_type: String,
    pub display_action_payload: Vec<IpcActionRenderPointsPayloadPoint>
}

