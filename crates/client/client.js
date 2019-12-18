function activate_tab(id, idx) {
  UIkit.tab(document.getElementById(id)).show(idx);
}

function notify(level, content) {
  UIkit.notification(content, { status: level });
}

function on_event(t, k, v) {
  window.{{crate_name}}.client.on_event(t, k, v);
}

function show_modal(id) {
  UIkit.modal(document.getElementById(id)).show();
}

function wire_onbeforeunload(socket) {
  window.addEventListener('beforeunload', (e) => {
    socket.close();
  });
}

function wire_textarea(id) {
  var el = document.getElementById(id);

  var savedValue = el.value;
  el.value = '';
  el.baseScrollHeight = el.scrollHeight;
  el.value = savedValue;

  el.oninput = function() {
    var minRows = this.getAttribute('data-min-rows')|0, rows;
    this.rows = minRows;
    rows = Math.ceil((this.scrollHeight - this.baseScrollHeight + 48) / 24);
    this.rows = minRows + rows;
  };
  el.oninput();
}

window.{{crate_name}} = {
  activate_tab: activate_tab,
  notify: notify,
  on_event: on_event,
  show_modal: show_modal,
  wire_onbeforeunload: wire_onbeforeunload,
  wire_textarea: wire_textarea
};
