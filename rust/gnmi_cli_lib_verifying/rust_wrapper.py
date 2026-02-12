import json
import subprocess
import os
from pathlib import Path

def _binary_path(base_path: Path) -> Path:
    if os.name == "nt":
        exe_path = base_path.with_suffix(".exe")
        if exe_path.exists():
            return exe_path
    return base_path

BINARY_PATH_RELEASE = _binary_path(
    Path(__file__).parent.parent / "gnmi_cli_lib" / "target" / "release" / "gnmi-cli-convert"
)
BINARY_PATH_DEBUG = _binary_path(
    Path(__file__).parent.parent / "gnmi_cli_lib" / "target" / "debug" / "gnmi-cli-convert"
)

class RustCommandParser:
    """Wrapper to call the Rust gnmi-cli-convert binary."""

    def __init__(self):
        self.binary = self._resolve_binary()

    def _resolve_binary(self) -> str:
        if BINARY_PATH_RELEASE.exists():
            return str(BINARY_PATH_RELEASE)
        if BINARY_PATH_DEBUG.exists():
            return str(BINARY_PATH_DEBUG)

        self._attempt_build()

        if BINARY_PATH_RELEASE.exists():
            return str(BINARY_PATH_RELEASE)
        if BINARY_PATH_DEBUG.exists():
            return str(BINARY_PATH_DEBUG)

        # Fallback when build is unavailable or binary is in PATH
        return "gnmi-cli-convert"

    def _attempt_build(self) -> None:
        cargo_dir = Path(__file__).parent.parent / "gnmi_cli_lib"
        if not cargo_dir.exists():
            return

        result = subprocess.run(
            ["cargo", "build", "--release"],
            cwd=str(cargo_dir),
            capture_output=True,
            text=True,
            check=False,
        )

        if result.returncode == 0:
            return

        subprocess.run(
            ["cargo", "build"],
            cwd=str(cargo_dir),
            capture_output=True,
            text=True,
            check=False,
        )

    def parse(self, command: str, json_data: dict) -> str:
        """
        Run the rust binary to convert json_data using the command.
        """
        try:
            input_str = json.dumps(json_data)
            result = subprocess.run(
                [self.binary, "--command", command],
                input=input_str,
                encoding='utf-8', # Explicitly use utf-8 to handle table characters
                capture_output=True,
                check=False # We handle return code manually if needed
            )
            
            if result.returncode != 0:
                print(f"Rust binary Error: {result.stderr}")
                return ""
                
            # Preserve stdout formatting to match converter output.
            return result.stdout
        except Exception as e:
            print(f"Execution failed: {e}")
            return ""

    def parse_and_convert(self, command: str, json_data: dict) -> str:
        """Alias for parse to match API expected by test runner."""
        return self.parse(command, json_data)

