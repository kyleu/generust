package com.{{crate_name}};

class App {
  private static native int go();

  int start_server() {
    return go();
  }
}
