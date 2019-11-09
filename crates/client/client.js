function on_event(t, k, v) {
  window.{{project-name}}.client.on_event(t, k, v);
};

window.{{project-name}} = {
  on_event: on_event
};
