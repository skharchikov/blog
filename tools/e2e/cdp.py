"""Minimal CDP client (raw websocket) to drive headless Chrome with trusted key events."""
import base64, json, os, socket, struct, sys, time, urllib.request


class WS:
    def __init__(self, url):
        assert url.startswith("ws://")
        hostport, path = url[5:].split("/", 1)
        host, port = hostport.split(":")
        self.s = socket.create_connection((host, int(port)))
        key = base64.b64encode(os.urandom(16)).decode()
        req = (f"GET /{path} HTTP/1.1\r\nHost: {hostport}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\n"
               f"Sec-WebSocket-Key: {key}\r\nSec-WebSocket-Version: 13\r\n\r\n")
        self.s.sendall(req.encode())
        buf = b""
        while b"\r\n\r\n" not in buf:
            buf += self.s.recv(4096)
        self.buf = buf.split(b"\r\n\r\n", 1)[1]
        self.id = 0
        self.events = []

    def _recv_exact(self, n):
        while len(self.buf) < n:
            chunk = self.s.recv(65536)
            if not chunk:
                raise EOFError
            self.buf += chunk
        out, self.buf = self.buf[:n], self.buf[n:]
        return out

    def send_text(self, text):
        data = text.encode()
        hdr = bytearray([0x81])
        n = len(data)
        if n < 126:
            hdr.append(0x80 | n)
        elif n < 65536:
            hdr.append(0x80 | 126); hdr += struct.pack(">H", n)
        else:
            hdr.append(0x80 | 127); hdr += struct.pack(">Q", n)
        mask = os.urandom(4)
        hdr += mask
        self.s.sendall(bytes(hdr) + bytes(b ^ mask[i % 4] for i, b in enumerate(data)))

    def recv_msg(self):
        msg = b""
        while True:
            b1, b2 = self._recv_exact(2)
            n = b2 & 0x7F
            if n == 126:
                n = struct.unpack(">H", self._recv_exact(2))[0]
            elif n == 127:
                n = struct.unpack(">Q", self._recv_exact(8))[0]
            msg += self._recv_exact(n)
            if b1 & 0x80:
                return json.loads(msg)

    def call(self, method, **params):
        self.id += 1
        my = self.id
        self.send_text(json.dumps({"id": my, "method": method, "params": params}))
        while True:
            m = self.recv_msg()
            if m.get("id") == my:
                if "error" in m:
                    raise RuntimeError(m["error"])
                return m.get("result")
            self.events.append(m)


def connect(port):
    for _ in range(50):
        try:
            tabs = json.load(urllib.request.urlopen(f"http://127.0.0.1:{port}/json"))
            page = [t for t in tabs if t["type"] == "page"][0]
            return WS(page["webSocketDebuggerUrl"])
        except Exception:
            time.sleep(0.2)
    raise SystemExit("no chrome")


class Page:
    def __init__(self, port):
        self.ws = connect(port)
        self.ws.call("Page.enable")
        self.ws.call("Runtime.enable")
        self.ws.call("Target.setDiscoverTargets", discover=True)

    def ev(self, expr):
        r = self.ws.call("Runtime.evaluate", expression=expr, returnByValue=True, awaitPromise=True)
        return r["result"].get("value")

    def goto(self, url):
        self.ws.call("Page.navigate", url=url)
        time.sleep(1.5)

    def key(self, key, code=None, text=None, vk=None, shift=False):
        p = {"key": key, "code": code or key, "modifiers": 8 if shift else 0}
        if vk:
            p["windowsVirtualKeyCode"] = vk
        if text is not None:
            self.ws.call("Input.dispatchKeyEvent", type="keyDown", text=text, **p)
        else:
            self.ws.call("Input.dispatchKeyEvent", type="rawKeyDown", **p)
        self.ws.call("Input.dispatchKeyEvent", type="keyUp", **p)
        time.sleep(0.4)

    def type(self, s):
        for ch in s:
            self.ws.call("Input.insertText", text=ch)
        time.sleep(0.2)

    def enter(self):
        self.key("Enter", "Enter", "\r", 13)

    def tab(self, shift=False):
        self.key("Tab", "Tab", None, 9, shift)

    def popups(self):
        targets = self.ws.call("Target.getTargets")["targetInfos"]
        return [t["url"] for t in targets if t["type"] == "page"]
