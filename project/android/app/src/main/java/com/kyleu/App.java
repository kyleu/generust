package com.{{crate_name}};

public class App {
  private static native int go();

  public int start_server() {
    return go();
  }
}
