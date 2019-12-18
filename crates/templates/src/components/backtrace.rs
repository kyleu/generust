use maud::{html, Markup};
use std::backtrace::Backtrace;

#[derive(Debug)]
struct Frame {
  id: i32,
  owned: bool,
  message: String,
  locs: Vec<String>
}

impl Frame {
  fn class(&self) -> &str {
    if self.owned {
      "frame owned"
    } else {
      "frame"
    }
  }

  fn from_backtrace(bt: &Backtrace) -> Vec<Self> {
    let content = format!("{:?}", bt);

    let mut curr_id = 0;
    let mut curr_message: String = "".into();
    let mut curr_locs: Vec<String> = vec![];

    content
      .lines()
      .flat_map(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        match parts[0] {
          id if id.ends_with(':') => {
            let n = id
              .get(..(id.len() - 1))
              .map(|i| i.parse::<i32>().expect("Cannot parse i32"))
              .expect("Cannot find expected key in map");
            let ret = if n == 0 {
              vec![]
            } else {
              let owned = curr_message.starts_with({{crate_name}}_core::APPNAME);
              vec![Self {
                id: curr_id,
                message: curr_message.clone(),
                owned,
                locs: curr_locs.clone()
              }]
            };
            curr_id = n;
            curr_message = line[(line.find(':').expect("Cannot find `:` in line") + 2)..].into();
            curr_locs = vec![];
            ret
          }
          _ => {
            curr_locs.push(line.trim().into());
            vec![]
          }
        }
      })
      .collect::<Vec<Self>>()
  }
}

pub(crate) fn to_html(bt: &Backtrace) -> Markup {
  let frames = Frame::from_backtrace(bt);
  html! {
    @for frame in frames {
      div.(frame.class()) {
        strong { (frame.id) }
        ": "
        span.message { (frame.message) }
        @for m in frame.locs {
          @if m.starts_with("at") {
            div.loc { (m) }
          } @else {
            div.additional { (m) }
          }
        }
      }
    }
  }
}
