use anyhow::{Context, Result};
use std::fs::read_dir;
use std::io::prelude::Write;

/// Provides basic filesystem operations, working within a config directory
#[derive(Clone, Debug)]
pub struct FileService {
  cfg_dir: String,
  log: slog::Logger
}

impl FileService {
  pub(crate) fn new(cfg_dir: &str, logger: &slog::Logger) -> FileService {
    let ret = FileService {
      cfg_dir: cfg_dir.into(),
      log: logger.new(slog::o!("service" => "file"))
    };
    match std::fs::create_dir_all(&ret.path()) {
      Ok(_) => (),
      Err(e) => slog::warn!(logger, "Can't create config directory at [{:?}]: {}", &ret.path(), e)
    };
    ret
  }

  pub fn cfg_dir(&self) -> &String {
    &self.cfg_dir
  }

  pub(crate) fn path(&self) -> &std::path::Path {
    std::path::Path::new(&self.cfg_dir)
  }

  // Unused, rename if you need it
  pub(crate) fn _list_json(&self, path: &str) -> Result<Vec<String>> {
    let p = self.path().join(path);
    if p.is_dir() {
      let rd = read_dir(&p).with_context(|| format!("Can't read directory [{}]", p.to_string_lossy()))?;

      let mut entries = rd
        .filter_map(|entry| {
          entry.ok().and_then(|e| {
            e.path().file_name().and_then(|n| {
              n.to_str()
                .filter(|x| x.ends_with(".json"))
                .map(|x| x.trim_end_matches(".json").into())
            })
          })
        })
        .collect::<Vec<String>>();
      entries.sort();
      Ok(entries)
    } else {
      Ok(Vec::new())
    }
  }

  fn read(&self, path: &str) -> Result<String> {
    let p = self.path().join(clean_path(path));
    std::fs::read_to_string(&p).with_context(|| format!("Can't read from file [{}]", p.to_string_lossy()))
  }

  pub(crate) fn read_json<T>(&self, path: &str) -> Result<T>
  where T: serde::de::DeserializeOwned {
    let json = self.read(path)?;
    serde_json::from_str(&json).with_context(|| "Can't decode json")
  }

  pub(crate) fn write(&self, path: &str, content: &str) -> Result<()> {
    let p = self.path().join(clean_path(path));
    let mut file = std::fs::File::create(p).with_context(|| {
      format!(
        "Can't create file [{}] in directory [{}]",
        clean_path(path),
        self.path().to_string_lossy()
      )
    })?;
    file
      .write_all(&content.as_bytes())
      .with_context(|| format!("Can't write to file [{}]", clean_path(path)))?;
    Ok(())
  }

  pub fn write_json<T>(&self, o: T, path: &str) -> Result<()>
  where T: serde::Serialize {
    let content = serde_json::to_string_pretty(&o).with_context(|| "Can't encode json")?;
    self.write(path, &content)
  }

  pub(crate) fn create_dir_if_needed(&self, path: &str) -> Result<()> {
    let p = self.path().join(path);
    std::fs::create_dir_all(p).with_context(|| format!("Can't create directory at path [{}]", path))
  }

  // Unused, rename if you need it
  pub(crate) fn _remove(&self, path: &str) -> Result<()> {
    let p = self.path().join(clean_path(path));
    std::fs::remove_file(p).with_context(|| format!("Can't remove file at [{}]", path))
  }
}

fn clean_path(path: &str) -> String {
  if path.ends_with(".json") {
    path.into()
  } else {
    format!("{}.json", path)
  }
}
