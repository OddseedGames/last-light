import base64
import hashlib
import json
import os
from pathlib import Path
import re
from zipfile import ZipFile, ZIP_DEFLATED


root = Path(__file__).resolve().parent.parent
dist = root / "dist"
wasm = (dist / "last-light.wasm").read_bytes()
wasm_hash = hashlib.sha256(wasm).hexdigest()
wasm_url = f"last-light.wasm?v={wasm_hash[:12]}"
html, count = re.subn(
    r"load\('last-light\.wasm(?:\?v=[a-f0-9]+)?'\);",
    f"load('{wasm_url}');", (dist / "index.html").read_text(),
)
if count != 1:
    raise ValueError("Expected exactly one WASM loader")
(dist / "index.html").write_text(html)
html = html.replace('<link rel="stylesheet" href="style.css">',
                    '<style>' + (dist / "style.css").read_text() + '</style>')
html = html.replace('<script src="mq_js_bundle.js"></script>',
                    '<script>' + (dist / "mq_js_bundle.js").read_text() + '</script>')
html = html.replace(f"load('{wasm_url}');", "load('data:application/wasm;base64," + base64.b64encode(wasm).decode() + "');")
(dist / "play.html").write_text(html)
with ZipFile(dist / "OddseedGames-LastLight.zip", "w", ZIP_DEFLATED) as archive:
    archive.write(dist / "play.html", "LastLight/play.html")
    archive.writestr("LastLight/README.txt", (root / "docs/QUICKSTART.txt").read_text().encode("utf-8-sig"))
    for license in ["THIRD-PARTY-LICENSES.txt", "OFL.txt"]:
        archive.write(dist / license, "LastLight/licenses/" + license)
files = ["index.html", "style.css", "mq_js_bundle.js", "last-light.wasm",
         "play.html", "OddseedGames-LastLight.zip", "THIRD-PARTY-LICENSES.txt", "OFL.txt"]
manifest = {
    "revision": os.environ.get("GITHUB_SHA", "local"),
    "files": {name: hashlib.sha256((dist / name).read_bytes()).hexdigest() for name in files},
}
(dist / "build.json").write_text(json.dumps(manifest, indent=2) + "\n")
print(f"Готово: {dist / 'play.html'}")
print(f"Архив: {dist / 'OddseedGames-LastLight.zip'}")
