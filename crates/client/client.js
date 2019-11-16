function on_event(t, k, v) {
  window.{{crate_name}}.client.on_event(t, k, v);
};

function notify(level, content) {
  UIkit.notification(content, { status: level });
};

window.{{crate_name}} = {
  on_event: on_event,
  notify: notify
};
