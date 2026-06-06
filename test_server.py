from http.server import HTTPServer, BaseHTTPRequestHandler

class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.handle_request()

    def do_POST(self):
        self.handle_request()

    def handle_request(self):
        # Print headers
        print("=== Headers ===")
        for key, value in self.headers.items():
            print(f"{key}: {value}")

        # Print body
        content_length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(content_length)
        print("\n=== Body ===")
        print(body.decode("utf-8") if body else "(empty)")
        print()

        # Default response
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.end_headers()
        self.wfile.write(b'{"status": "ok"}')

server = HTTPServer(("localhost", 8080), Handler)
print("Serving on http://localhost:8080")
server.serve_forever()