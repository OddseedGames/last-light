import base64
import hashlib
import json
from pathlib import Path
import re
from zipfile import ZipFile


dist = Path(__file__).resolve().parent.parent / "dist"
manifest = json.loads((dist / "build.json").read_text())
for name, expected_hash in manifest["files"].items():
    actual_hash = hashlib.sha256((dist / name).read_bytes()).hexdigest()
    if actual_hash != expected_hash:
        raise ValueError(f"Hash mismatch: {name}")

html = (dist / "play.html").read_text()
encoded = re.findall(r"load\('data:application/wasm;base64,([A-Za-z0-9+/=]+)'\);", html)
if len(encoded) != 1:
    raise ValueError("Standalone HTML must contain exactly one embedded WASM")
wasm = (dist / "last-light.wasm").read_bytes()
if wasm[:4] != b"\x00asm" or base64.b64decode(encoded[0], validate=True) != wasm:
    raise ValueError("Embedded WASM does not match built WASM")
for external in ['href="style.css"', 'src="mq_js_bundle.js"', "load('last-light.wasm"]:
    if external in html:
        raise ValueError(f"Standalone HTML depends on an external file: {external}")

with ZipFile(dist / "OddseedGames-LastLight.zip") as archive:
    if archive.testzip() is not None:
        raise ValueError("Archive is corrupted")
    expected = {"LastLight/play.html", "LastLight/README.txt",
                "LastLight/licenses/THIRD-PARTY-LICENSES.txt", "LastLight/licenses/OFL.txt"}
    if set(archive.namelist()) != expected or archive.read("LastLight/play.html").decode() != html:
        raise ValueError("Archive contents do not match the standalone game")
print("WASM, автономный HTML, ZIP и контрольные суммы проверены.")
