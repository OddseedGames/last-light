from pathlib import Path
import unittest

from playwright.sync_api import expect, sync_playwright


ROOT = Path(__file__).resolve().parents[2]


class OfflineGameTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.playwright = sync_playwright().start()
        cls.browser = cls.playwright.chromium.launch(
            args=["--use-angle=swiftshader", "--enable-unsafe-swiftshader"]
        )

    @classmethod
    def tearDownClass(cls):
        cls.browser.close()
        cls.playwright.stop()

    def setUp(self):
        self.context = self.browser.new_context(
            viewport={"width": 1440, "height": 1100}, offline=True
        )
        self.context.tracing.start(screenshots=True, snapshots=True, sources=True)
        self.page = self.context.new_page()
        self.errors = []
        self.external_requests = []
        self.page.on("pageerror", lambda error: self.errors.append(str(error)))
        self.page.on("console", lambda message: self.errors.append(message.text)
                     if message.type == "error" else None)
        self.page.on("request", lambda request: self.external_requests.append(request.url)
                     if request.url.startswith(("http://", "https://")) else None)
        self.page.goto((ROOT / "dist/play.html").as_uri())
        self.status = self.page.locator("#game-status")
        expect(self.status).to_have_attribute("data-phase", "0", timeout=30_000)
        expect(self.page.locator("#loading")).to_be_hidden()
        expect(self.page.locator("#load-error")).to_be_hidden()
        self.page.locator("canvas").focus()

    def tearDown(self):
        diagnostics = ROOT / "test-results" / self.id().rsplit(".", 1)[-1]
        diagnostics.mkdir(parents=True, exist_ok=True)
        try:
            self.page.screenshot(path=str(diagnostics / "screen.png"), full_page=True)
            self.context.tracing.stop(path=str(diagnostics / "trace.zip"))
            self.assertEqual(self.errors, [])
            self.assertEqual(self.external_requests, [])
        finally:
            self.context.close()

    def state(self):
        return self.status.evaluate(
            "element => Object.fromEntries(Object.entries(element.dataset)"
            ".map(([key, value]) => [key, Number(value)]))"
        )

    def wait_for(self, condition):
        self.page.wait_for_function(
            "() => { const s = Object.fromEntries(Object.entries("
            "document.querySelector('#game-status').dataset)"
            ".map(([key, value]) => [key, Number(value)])); return " + condition + "; }",
            timeout=20_000,
        )

    def start(self):
        self.page.keyboard.press("Enter")
        expect(self.status).to_have_attribute("data-phase", "1")

    def aim(self, x, y):
        box = self.page.locator("canvas").bounding_box()
        self.page.mouse.move(box["x"] + x / 1280 * box["width"],
                             box["y"] + y / 880 * box["height"])

    def test_movement_and_light_rotation(self):
        self.start()
        start = self.state()
        self.page.keyboard.down("d")
        self.page.keyboard.down("w")
        try:
            self.wait_for(f"s.x > {start['x'] + 35} && s.y < {start['y'] - 35}")
        finally:
            self.page.keyboard.up("d")
            self.page.keyboard.up("w")
        moved = self.state()
        self.aim(moved["x"] + 28 + 180, moved["y"] + 110)
        self.wait_for("Math.abs(s.angle) < 0.05")
        self.page.keyboard.down("q")
        try:
            self.wait_for("s.angle < -0.8")
        finally:
            self.page.keyboard.up("q")
        self.page.keyboard.press("p")
        expect(self.status).to_have_attribute("data-phase", "2")
        angle = self.state()["angle"]
        self.page.wait_for_timeout(250)
        self.assertEqual(self.state()["angle"], angle)
        self.page.keyboard.press("Enter")
        self.aim(moved["x"] + 28 + 185, moved["y"] + 110 + 2)
        self.wait_for("Math.abs(s.angle) < 0.05")

    def test_shooting_and_reload(self):
        self.start()
        self.page.keyboard.down("Space")
        try:
            self.wait_for("s.fired === 6 && s.ammo === 0 && s.reload > 0")
        finally:
            self.page.keyboard.up("Space")
        self.wait_for("s.ammo === 6 && s.reload === 0")
        self.page.keyboard.press("Space", delay=70)
        self.wait_for("s.fired === 7 && s.ammo === 5")
        self.page.keyboard.press("r")
        self.wait_for("s.reload > 0")
        self.wait_for("s.reload === 0 && s.ammo === 6")

    def test_pause_blur_and_restart(self):
        self.start()
        self.wait_for("s.elapsed > 0.2")
        self.page.keyboard.press("p")
        expect(self.status).to_have_attribute("data-phase", "2")
        paused = self.state()
        self.page.wait_for_timeout(350)
        self.assertEqual(self.state(), paused)
        self.page.keyboard.press("Enter")
        expect(self.status).to_have_attribute("data-phase", "1")
        self.page.evaluate("window.dispatchEvent(new Event('blur'))")
        expect(self.status).to_have_attribute("data-phase", "2")
        self.page.keyboard.press("n")
        expect(self.status).to_have_attribute("data-phase", "1")
        state = self.state()
        self.assertEqual(state["health"], 3)
        self.assertEqual(state["ammo"], 6)
        self.assertEqual(state["kills"], 0)
        self.assertLess(state["elapsed"], 0.5)

    def test_start_button_does_not_fire(self):
        self.aim(288, 562)
        self.page.mouse.down()
        try:
            expect(self.status).to_have_attribute("data-phase", "1")
            self.page.wait_for_timeout(250)
            self.assertEqual(self.state()["fired"], 0)
        finally:
            self.page.mouse.up()
        self.page.wait_for_timeout(70)
        self.aim(850, 440)
        self.page.mouse.down()
        try:
            self.wait_for("s.fired === 1")
        finally:
            self.page.mouse.up()


if __name__ == "__main__":
    unittest.main()
