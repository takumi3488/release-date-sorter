use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UserVolume {
    pub user_id: Uuid,
    pub volume_id: Uuid,
    pub checked: bool,
}
