import * as wasm from "../pkg/{{crate_name}}_client";

import "../client.js";

window.{{project-name}}.client = new wasm.JsClient();
