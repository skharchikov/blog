"""Trusted-input regression checks (real key presses and clicks via the
Chrome DevTools protocol). Needs a running site and headless Chrome:

    PATH="$HOME/.cargo/bin:$PATH" trunk serve            # or serve.py on dist/
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" --headless=new \\
        --remote-debugging-port=9333 --window-size=1280,800 about:blank &
    python3 tools/e2e/keys.py [base-url]                  # default http://127.0.0.1:8080
"""
import os
import sys
import time

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from cdp import Page  # noqa: E402

B = sys.argv[1] if len(sys.argv) > 1 else "http://127.0.0.1:8080"
p = Page(9333)
failures = []


def check(name, ok, detail=""):
    print(("PASS " if ok else "FAIL ") + name + (f"  [{detail}]" if detail else ""))
    if not ok:
        failures.append(name)


def path():
    return p.ev("location.pathname + location.search")


def click(sel):
    r = p.ev(f"(()=>{{const e=document.querySelector({sel!r}); if(!e) return null; const b=e.getBoundingClientRect(); return [b.x+b.width/2,b.y+b.height/2]}})()")
    for t in ("mousePressed", "mouseReleased"):
        p.ws.call("Input.dispatchMouseEvent", type=t, x=r[0], y=r[1], button="left", clickCount=1)
    time.sleep(0.8)


def digit(n):
    p.key(str(n), f"Digit{n}", str(n), 48 + n)


# home shell: digits switch tabs on an empty prompt; Tab can leave the prompt
p.goto(B + "/")
p.ev("document.querySelector('.input-row input').focus()")
digit(2)
check("empty prompt + 2 -> /posts", path() == "/posts", path())
p.goto(B + "/")
p.ev("document.querySelector('.input-row input').focus()")
p.tab()
check("Tab leaves the shell input", p.ev("document.activeElement.tagName") != "INPUT", p.ev("document.activeElement.tagName"))
p.goto(B + "/")
p.ev("document.querySelector('.input-row input').focus()")
p.type("th")
p.tab()
check("Tab completes 'th' -> 'theme'", p.ev("document.querySelector('.input-row input').value") == "theme")
shadow = p.ev("getComputedStyle(document.querySelector('.input-row')).boxShadow")
check("focused prompt draws no second caret-like bar", shadow == "none", shadow)

# Enter on a focused control does its native thing
p.goto(B + "/posts")
p.ev("document.querySelector('.chips a[href=\"/posts?tag=rust\"]').focus()")
p.enter()
check("focused #rust chip + Enter -> filter", path() == "/posts?tag=rust", path())
p.goto(B + "/posts")
p.ev("document.querySelector('a.tab[href=\"/books\"]').focus()")
p.enter()
check("focused tab link + Enter -> that tab", path() == "/books", path())

# list: j/k/Enter, q keeps the place; one click opens a row
p.goto(B + "/posts")
p.key("j", "KeyJ", "j", 74)
p.key("j", "KeyJ", "j", 74)
p.enter()
opened = path()
check("j j Enter -> third post", opened.startswith("/posts/") and opened != "/posts/", opened)
p.key("q", "KeyQ", "q", 81)
sel = p.ev("document.querySelectorAll('.list li').length && [...document.querySelectorAll('.list li')].findIndex(li => li.classList.contains('sel'))")
check("q -> back on /posts with row 3 still selected", path() == "/posts" and sel == 2, f"{path()} sel={sel}")
p.goto(B + "/posts")
click(".list li:nth-child(2) a")
check("single click on a row opens it", path().startswith("/posts/"), path())

# projects: the list doesn't move when the preview changes
p.goto(B + "/projects")
top0 = p.ev("document.querySelector('.list').getBoundingClientRect().top")
for _ in range(3):
    p.key("j", "KeyJ", "j", 74)
top1 = p.ev("document.querySelector('.list').getBoundingClientRect().top")
check("projects list stays put while the preview changes", abs(top0 - top1) < 1, f"{top0} -> {top1}")

# books: the selected cover stays visible; covers show a focus ring
p.goto(B + "/books")
for _ in range(8):
    p.key("j", "KeyJ", "j", 74)
vis = p.ev("(()=>{const c=document.querySelector('.cv.sel').getBoundingClientRect(); const pane=document.querySelector('.wall').closest('.in').getBoundingClientRect(); return c.top>=pane.top-2 && c.bottom<=pane.bottom+2})()")
check("8 x j keeps the selected cover in view", vis is True)
p.ev("document.querySelector('.cv').focus()")
ring = p.ev("getComputedStyle(document.querySelector('.cv')).outlineColor")
check("focused cover has a visible ring", ring not in ("rgba(0, 0, 0, 0)", "transparent"), ring)

# book grid keys move the selection, also on a non-Latin keyboard layout
p.goto(B + "/books")
T = "document.querySelector('.book-detail h2').textContent"
t0 = p.ev(T)
p.key("l", "KeyL", "l", 76)
t1 = p.ev(T)
check("books: l moves right", t1 != t0, f"{t0} -> {t1}")
p.key("о", "KeyJ", "о", 74)  # the J key on a Russian layout types "о"
t2 = p.ev(T)
check("books: j moves down on a Russian layout", t2 != t1, f"{t1} -> {t2}")

# contact: one address, a mailto link, no copy button
p.goto(B + "/contacts")
emails = p.ev("[...document.querySelectorAll('.email a')].map(a => a.textContent)")
check("contact shows only me@skh.rs", emails == ["me@skh.rs"], str(emails))
href = p.ev("document.querySelector('.email a').getAttribute('href')")
check("email links to mailto:me@skh.rs", href == "mailto:me@skh.rs", href)
check("no copy button", p.ev("document.querySelectorAll('.copy').length") == 0)

print(f"\n{len(failures)} failure(s)")
sys.exit(1 if failures else 0)
