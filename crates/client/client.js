function on_event(t, k, v) {
  window.{{crate_name}}.client.on_event(t, k, v);
};

window.{{crate_name}} = {
  on_event: on_event
};
