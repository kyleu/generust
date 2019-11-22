use anyhow::Result;
use {{crate_name}}_core::profile::UserProfile;

pub fn load(files: &crate::files::FileService, user_id: uuid::Uuid) -> UserProfile {
  let path = format!("profile/{}", user_id);
  match files.read_json(&path) {
    Ok(p) => p,
    Err(_) => UserProfile::default()
  }
}

pub fn save(files: &crate::files::FileService, user_id: &uuid::Uuid, profile: &UserProfile) -> Result<()> {
  files.create_dir_if_needed("profile")?;
  files.write_json(profile, &format!("profile/{}", user_id))
}
