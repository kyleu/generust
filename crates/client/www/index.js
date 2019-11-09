import * as wasm from "../pkg/{{crate_name}}_client";

import "../client.js";

window.{{crate_name}}.client = new wasm.JsClient();
