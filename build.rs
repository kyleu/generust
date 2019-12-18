fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    let path = "project/windows/app.ico";
    res.set_icon(path);
    res.compile().expect("Can't compile Windows icon");
  }
}
