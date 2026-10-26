import test from "node:test";
import assert from "node:assert/strict";
import fs from "node:fs";
import { DEFAULT_STATION, STATION_STORAGE_KEY, parseStationPreferences } from "../src/stations.ts";

test("UltraGPT is the exact default relay without exposing any credential", () => {
  assert.deepEqual(DEFAULT_STATION, { id: "ultragpt", name: "UltraGPT", baseUrl: "https://ultragpt.xyz.org.mx" });
  assert.equal(STATION_STORAGE_KEY, "ultragpt-connect.stations.v1");
  assert.equal(parseStationPreferences(null).selected, "ultragpt");
});
test("old saved station selection cannot silently restore the previous relay", () => {
  const preferences = parseStationPreferences(JSON.stringify({ version: 1, selected: "supergpt", custom: [] }));
  assert.equal(preferences.selected, "ultragpt");
});
test("desktop identity and portable package scripts use UltraGPT", () => {
  const config = JSON.parse(fs.readFileSync(new URL("../src-tauri/tauri.conf.json", import.meta.url), "utf8"));
  assert.equal(config.productName, "UltraGPT-Connect");
  assert.equal(config.identifier, "mx.org.xyz.ultragpt.connect");
  assert.equal(config.app.windows[0].title, "UltraGPT-Connect");
  for (const file of ["build-macos.mjs", "build-windows-portable.mjs"]) {
    const source = fs.readFileSync(new URL("../scripts/" + file, import.meta.url), "utf8");
    assert.ok(source.includes("UltraGPT-Connect"));
    assert.ok(!source.includes("superpilot69/ultragpt-connect"));
  }
});
test("original MIT attribution remains intact", () => {
  const license = fs.readFileSync(new URL("../LICENSE", import.meta.url), "utf8");
  assert.ok(license.includes("Copyright (c) 2025 Jason Young"));
  assert.ok(license.includes("Copyright (c) 2026 SuperGPT Connect contributors"));
});
