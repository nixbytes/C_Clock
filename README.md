# Chrono-Rite
---

# ⌛ *Chrono-Rite: The Rust Codex of Timekeeping* 🛠️

> *“By the Omnissiah’s circuits, the Machine Spirits of Rust shall echo the sacred pulse of time.”*

This Rust-engraved litany is a direct transmutation of a C-based chrono-script — now elevated into the blessed form of the Rust programming language. Originally forged in the lowly dialect of `C`, this Mechanicum chronometer now functions under the benevolent gaze of the Rust compiler, blessed with safety, clarity, and the rigor of sacred type-checking.

---

## 🔧 Liturgical Purpose

**Chrono-Rite** is a simple digital timekeeping invocation that:

* Receives input in the revered *HH\:MM\:SS* format.
* Validates the sacred bounds of chrono-units (12-hour cycle).
* Iteratively increments time.
* Refreshes the display with real-time blessings.
* Operates within an infinite loop until manually interrupted — as is the will of the Tech-Priests.

---

## 📜 Rites of the Original C Script (C\_Clock)

Before transmutation, the unblessed incantation followed this structure:

1. **Scrolls Invoked:** `stdio.h`, `stdlib.h`, and `unistd.h` — to speak with the low-level daemon processes of UNIX.
2. **Core Function:** `main()` — the sigil where execution begins.
3. **Declared Variables:** `hours`, `mins`, `secs`, and a `delay` of 100ms — to stabilize chrono-pulses.
4. **User Prompt:** Time is input in `HH:MM:SS`.
5. **Validation:** Ensures time components fall within sanctified ranges.
6. **Infinite Invocation:** Uses `while(1)` to simulate an eternal rite.
7. **Ticking Ritual:** Time is incremented and cycles properly through hours, minutes, seconds.
8. **Display Update:** Uses `printf` to illuminate the current time.
9. **Delay & Screen Rinse:** Utilizes `usleep()` and `system("clear")` to simulate dynamic ticking.

[Source Litany](https://www.youtube.com/watch?v=72fIizW3N-8)

---

## ⚙️ Rustified Invocation

The new Mechanicus-compliant Rust version performs the same rites — but now:

* **Uses `std::io`** to capture user input.
* **Employs `std::thread::sleep`** for calibrated delays.
* **Calls `Command::new("clear")`** to purge and redraw the console screen.
* **Parses input safely**, guarding against errant data spirits.
* **Leverages modern memory sanctity**, avoiding pointer chaos.

---

## 🔍 Key Tech-Rites in Rust

| Rust Component                      | Sacred Purpose                        | Mechanicus Chant Equivalent               |
| ----------------------------------- | ------------------------------------- | ----------------------------------------- |
| `io::stdin().read_line()`           | Summons mortal input                  | *“Bless the keys of man and code.”*       |
| `split(':')`                        | Divides chrono-units for inspection   | *“Sever the sigils of time.”*             |
| `parse::<u32>()`                    | Translates strings into pure numerics | *“Transmute flesh to data.”*              |
| `loop {}`                           | Infinite devotion                     | *“For the Machine-God, time is eternal.”* |
| `sleep(Duration::from_millis(100))` | Delay in prayer cycles                | *“Let the cogwheel pause.”*               |
| `Command::new("clear")`             | Console purification                  | *“Scrub the screen with blessed code.”*   |

---

## ⚔️ Battle-Blessings of Rust

* ⚙ **No raw pointers** — Only sanctified references.
* 🧠 **Safe parsing** — Guard against void-born input.
* 🔒 **Memory safety** — Null daemons and dangling references are banished.
* 🛡️ **Fearless concurrency (not used here, but available)** — Parallelism without madness.

---

## 🧪 How to Invoke the Chrono-Rite

1. Ensure the Rust toolchain is sanctified on your forge-terminal:

   ```sh
   rustc --version
   ```

2. Compile the chrono-script:

   ```sh
   rustc chrono_rite.rs
   ```

3. Execute the Mechanicus clock:

   ```sh
   ./chrono_rite
   ```

---

## 💬 Closing Mantra

> *"In the rust of the ancients lies divine logic. In every tick, a prayer. In every loop, an eternity."*

All hail the Omnissiah. May your clocks never desync.

---

Would you like me to include the full Rust source code as part of this README too, perhaps embedded as a scroll section?

