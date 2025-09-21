# requires -Version 5.0

# Check if bindgen is installed
if (-not (Get-Command bindgen -ErrorAction SilentlyContinue)) {
    Write-Host "bindgen not found. Install it using 'cargo install bindgen-cli'"
    exit 1
}

# Set default header path if not already set
if (-not $env:CLAY_HEADER_PATH) {
    $env:CLAY_HEADER_PATH = "clay.h"
}

$COMMON_BINDGEN_FLAGS = @(
    "--use-core"
    "--no-layout-tests"
    "--blocklist-file", ".*stdlib.*"
    "--blocklist-file", ".*pthread.*"
    "--blocklist-file", ".*glibc.*"
    "--blocklist-file", ".*pthread_rwlock.*",
)

# Generate normal bindings
bindgen $env:CLAY_HEADER_PATH -o src/bindings/bindings.rs $COMMON_BINDGEN_FLAGS
Write-Host "Generated src/bindings/bindings.rs"

# Generate debug bindings
bindgen $env:CLAY_HEADER_PATH -o src/bindings/bindings_debug.rs $COMMON_BINDGEN_FLAGS -- -DCLAY_DEBUG
Write-Host "Generated src/bindings/bindings_debug.rs"
