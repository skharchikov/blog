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
    r = p.ev(f"(()=>{{const e=document.querySelector({sel!r}); if(!e) return null; e.scrollIntoView({{block:'center'}}); const b=e.getBoundingClientRect(); return [b.x+b.width/2,b.y+b.height/2]}})()")
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

SEL_HREF = "(document.querySelector('.list li.sel a') || {}).getAttribute?.('href')"

# Enter opens the highlighted row, also after focus landed on a row or a chip
p.goto(B + "/posts")
third = p.ev("document.querySelectorAll('.list a.row')[2].getAttribute('href')")
p.ev("document.querySelector('.list a.row').focus()")
p.key("j", "KeyJ", "j", 74)
p.key("j", "KeyJ", "j", 74)
p.enter()
check("focused row, j j Enter -> the highlighted third post", path() == third, f"{path()} != {third}")
p.goto(B + "/posts")
p.key("/", "Slash", "/", 191)
p.key("j", "KeyJ", "j", 74)
p.enter()
check("/ then j Enter -> a post, not the chip's filter", path().startswith("/posts/"), path())
p.goto(B + "/projects")
second = p.ev("document.querySelectorAll('.list a.row')[1].getAttribute('href')")
p.ev("document.querySelector('.list a.row').focus()")
p.key("j", "KeyJ", "j", 74)
p.enter()
check("projects: focused row, j Enter -> the highlighted project", path() == second, f"{path()} != {second}")

# q from a post keeps the tag filter and the list selects the post just read
p.goto(B + "/posts?tag=rust")
p.key("j", "KeyJ", "j", 74)
p.enter()
read = path()
p.key("q", "KeyQ", "q", 81)
check("q from a filtered post -> back on the filtered list", path() == "/posts?tag=rust", path())
check("... with that post selected", p.ev(SEL_HREF) == read, f"{p.ev(SEL_HREF)} != {read}")
p.goto(B + read)
click(".reader .back a")
check("'all posts' link -> the unfiltered list with that post selected", path() == "/posts" and p.ev(SEL_HREF) == read, f"{path()} {p.ev(SEL_HREF)}")

# the "all" chip is only current when no tag is selected
p.goto(B + "/posts?tag=rust")
cur = p.ev("[...document.querySelectorAll('.chips [aria-current=page]')].map(a => a.textContent)")
check("filtered: only the tag chip is current", cur == ["#rust"], str(cur))
p.goto(B + "/posts")
cur = p.ev("[...document.querySelectorAll('.chips [aria-current=page]')].map(a => a.textContent)")
check("unfiltered: 'all' is current", cur == ["all"], str(cur))

# books: a first plain click selects a cover, it doesn't open Hardcover
p.goto(B + "/books")
before = len(p.popups())
click(".cv:nth-of-type(3)")
title = p.ev("document.querySelector('.book-detail h2').textContent")
want = p.ev("document.querySelector('.cv:nth-of-type(3)').getAttribute('title')")
check("books: first click selects the cover", title == want, f"{title} != {want}")
check("books: first click opens nothing", len(p.popups()) == before, f"{before} -> {len(p.popups())}")
# hjkl move focus along with the selection once a cover has keyboard focus
p.goto(B + "/books")
p.ev("document.querySelector('.cv').focus()")
p.key("l", "KeyL", "l", 76)
check("books: l moves focus with the selection", p.ev("document.activeElement.classList.contains('sel')") is True)

# shell: theme names in any case; navigating from the shell logs no warnings
p.goto(B + "/")
p.ev("document.querySelector('.input-row input').focus()")
p.type("theme Paper")
p.enter()
check("'theme Paper' sets paper", p.ev("document.documentElement.dataset.theme") == "paper")
p.ev("localStorage.removeItem('theme')")
p.ws.events.clear()
p.ev("document.querySelector('.input-row input').focus()")
p.type("cd books")
p.enter()
logs = [a.get("value", "") for e in p.ws.events if e.get("method") == "Runtime.consoleAPICalled"
        and e["params"]["type"] in ("warning", "error") for a in e["params"]["args"]]
check("cd books -> /books", path() == "/books", path())
check("cd from the shell logs no warnings", not logs, str(logs)[:200])

# the old site's dark-mode choice carries over to the new themes
p.ws.call("Emulation.setEmulatedMedia", features=[{"name": "prefers-color-scheme", "value": "light"}])
p.ev("localStorage.removeItem('theme'); localStorage.setItem('darkMode', 'true')")
p.goto(B + "/")
check("old darkMode=true on a light OS -> skh", p.ev("document.documentElement.dataset.theme") == "skh")
check("... saved as theme, old key removed", p.ev("[localStorage.getItem('theme'), localStorage.getItem('darkMode')]") == ["skh", None])
p.ev("localStorage.removeItem('theme'); localStorage.setItem('darkMode', 'false')")
p.goto(B + "/")
check("old darkMode=false on a light OS -> paper", p.ev("document.documentElement.dataset.theme") == "paper")
p.ws.call("Emulation.setEmulatedMedia", features=[])
p.ev("localStorage.clear()")

# narrow screen: the active tab is scrolled into the tab bar
p.ws.call("Emulation.setDeviceMetricsOverride", width=390, height=800, deviceScaleFactor=1, mobile=True)
p.goto(B + "/contacts")
inside = p.ev("(()=>{const t=document.querySelector('.tab[aria-current=page]').getBoundingClientRect(), b=document.querySelector('.tabs').getBoundingClientRect(); return t.left>=b.left-1 && t.right<=b.right+1})()")
check("390px: active tab is visible in the bar", inside is True)
p.ws.call("Emulation.clearDeviceMetricsOverride")

print(f"\n{len(failures)} failure(s)")
sys.exit(1 if failures else 0)
