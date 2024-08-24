use super::profile_data::ProfileData;

#[derive(Clone, Debug, Default)]
pub struct GlobalContext {
    pub profile: ProfileData,
    pub profile_loaded: bool,
}