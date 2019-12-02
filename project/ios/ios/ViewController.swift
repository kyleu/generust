import UIKit
import WebKit

class ViewController: UIViewController, WKUIDelegate {
    var webView: WKWebView!

    override func loadView() {
        let webConfiguration = WKWebViewConfiguration()
        webView = WKWebView(frame: .zero, configuration: webConfiguration)
        webView.uiDelegate = self
        view = webView
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        let port = libgo()
        print("Started on port [\(port)]!")
        
        let myURL = URL(string:"http://localhost:\(port)")
        let myRequest = URLRequest(url: myURL!)
        webView.load(myRequest)
    }
}

