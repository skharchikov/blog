"""Serve dist/ like Cloudflare Pages would for this SPA: real files first,
otherwise index.html. /__harness.html is the screenshot harness.

    python3 tools/screenshots/serve.py <dist-dir> <port>
"""
import http.server
import os
import sys

DIST = os.path.abspath(sys.argv[1])
HARNESS = os.path.join(os.path.dirname(os.path.abspath(__file__)), "harness.html")


class Handler(http.server.SimpleHTTPRequestHandler):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, directory=DIST, **kwargs)

    def translate_path(self, path):
        clean = path.split("?", 1)[0].split("#", 1)[0]
        if clean == "/__harness.html":
            return HARNESS
        real = super().translate_path(path)
        if os.path.isfile(real) or os.path.isfile(os.path.join(real, "index.html")):
            return real
        return os.path.join(DIST, "index.html")

    def log_message(self, *args):
        pass


http.server.ThreadingHTTPServer(("127.0.0.1", int(sys.argv[2])), Handler).serve_forever()
