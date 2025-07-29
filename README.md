
# STM32H7 Rust Project Setup Guide

This guide explains how to quickly set up a Rust project for the **STM32H7** series using a template based on the [knurling-rs app-template](https://github.com/knurling-rs/app-template).

---

## Step 1: Set up tools

1. **Install flip-link**

   ```bash
   cargo install flip-link
   ```

2. **Install probe-rs**
   Follow the installation instructions here:
   [https://probe.rs/docs/getting-started/installation/](https://probe.rs/docs/getting-started/installation/)

3. **Install cargo-generate**

   ```bash
   cargo install cargo-generate
   ```

---

## Step 2: Setup Project

### On Linux / macOS:

```bash
cargo generate \
    --git https://github.com/PyroBear5/STM32H7-rust-template \
    --branch main \
    --name my-STM32H7-project
```

### On Windows (Command Prompt):

```cmd
cargo generate ^
    --git https://github.com/PyroBear5/STM32H7-rust-template ^
    --branch main ^
    --name my-STM32H7-project
```

### On Windows (PowerShell):

```powershell
cargo generate `
    --git https://github.com/PyroBear5/STM32H7-rust-template `
    --branch main `
    --name my-STM32H7-project
```

> **Note:** Replace `my-STM32H7-project` with your project name.

---

### Optional
You can also use:
```cmd
cargo generate --git https://github.com/PyroBear5/STM32H7-rust-template
```
then it will ask you the name of the project:
```cmd
PS {path_to_your_project}> cargo generate --git https://github.com/PyroBear5/STM32H7-rust-template
 Project Name:
```
Simply type in the name of your project and press enter to generate the project.

## Step 3: Run the Project

To build and flash the code:

```bash
cargo run
```

Example output:

```
PS {path_to_your_project}\my-STM32H7-project> cargo run --bin hello
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.10s
     Running `probe-rs run --chip STM32H753ZI target\thumbv7em-none-eabihf\debug\hello`
      Erasing ✔ 100% [####################] 128.00 KiB @  70.93 KiB/s (took 2s)
  Programming ✔ 100% [####################]   8.00 KiB @  45.68 KiB/s (took 0s)
     Finished in 1.98s
Hello, world!
Firmware exited successfully
```

If multiple devices are connected, you can select one:

```
Available Probes:
0: STLink V3 -- 0483:374e:004800303234510133353533 (ST-LINK)
1: STLink V3 -- 0483:374e:002C00333234510233353533 (ST-LINK)
Selection:
```

---

## Step 4: Testing

Integration tests are located in the `tests` directory (e.g., `tests/integration.rs`).

Run them with:

```bash
cargo test
```
This will run all tests
> **Note:** If you add a new test file to `tests/`, you also need to add a new `[[test]]` section to `Cargo.toml`.

If you just want to run one test use:
```bash
cargo test --test {name_of_desired_test}
```

---

## Optional: Individualize Your Project

### Change the Chip

In `.cargo/config.toml`:

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = "probe-rs run --chip STM32H753ZI"  # Change to your chip if needed
```

### Custom Log Format

You can customize the log format:

```toml
[target.'cfg(all(target_arch = "arm", target_os = "none"))']
runner = ["probe-rs", "run", "--chip", "STM32H753ZI", "--log-format", "{L} {s}"]
```

### Adjust Compilation Target

In `.cargo/config.toml`:

```toml
[build]
# target = "thumbv6m-none-eabi"    # Cortex-M0/M0+
# target = "thumbv7m-none-eabi"    # Cortex-M3
# target = "thumbv7em-none-eabi"   # Cortex-M4/M7 (no FPU)
target = "thumbv7em-none-eabihf"   # Cortex-M4F/M7F (with FPU)
```

---

## Dependencies

In `Cargo.toml`, adjust dependencies as needed:

```toml
[dependencies]
cortex-m = { version = "0.7", features = ["critical-section-single-core"] }
cortex-m-rt = "0.7"
defmt = "1.0.1"
defmt-rtt = "1.0.0"
panic-probe = { version = "1.0.0", features = ["print-defmt"] }
cortex-m-semihosting = "0.5.0"
stm32h7xx-hal = { version = "0.16.0", features = ["stm32h753v", "rt", "defmt"] }
embedded-hal = "1.0.0"

# Older embedded-hal required by stm32h7xx HAL
embedded-hal-02 = { package = "embedded-hal", version = "0.2.6", features = ["unproven"] }
```

---

## Imports

If you modify dependencies, also update `src/lib.rs`:

```rust
use stm32h7xx_hal as _; // Change if needed

// If using nb:
// use nb;
```

---

## Linker Script

If your HAL crate requires a `memory.x` file, ensure it’s in your project root.
The `stm32h7xx-hal` crate already provides one:
[https://github.com/stm32-rs/stm32h7xx-hal/blob/master/memory.x](https://github.com/stm32-rs/stm32h7xx-hal/blob/master/memory.x)

---

You’re now ready to develop with Rust on STM32H7!

---
