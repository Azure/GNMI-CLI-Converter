"""Smoke tests for the gnmi_cli_lib.cli module.

cli.py is a thin CLI wrapper — only key integration paths need testing.
"""

import json
import pytest

from gnmi_cli_lib.cli import main


class TestMain:
    """Smoke tests for the CLI entry point."""

    def test_list_commands(self, monkeypatch, capsys):
        """--list-commands prints supported commands."""
        monkeypatch.setattr("sys.argv", ["cli", "--list-commands"])
        assert main() == 0
        assert "Supported CLI Commands:" in capsys.readouterr().out

    def test_convert_with_command(self, monkeypatch, tmp_path, capsys):
        """-c converts JSON data to CLI output."""
        json_file = tmp_path / "data.json"
        json_file.write_text(json.dumps({"ingress_lossless_pool": {"Bytes": "5000"}}))
        monkeypatch.setattr("sys.argv", [
            "cli", "-i", str(json_file), "-c", "show buffer_pool watermark"
        ])
        assert main() == 0
        assert "5000" in capsys.readouterr().out

    def test_convert_json_format(self, monkeypatch, tmp_path, capsys):
        """--format json wraps result in JSON envelope."""
        json_file = tmp_path / "data.json"
        json_file.write_text(json.dumps({"ingress_lossless_pool": {"Bytes": "1234"}}))
        monkeypatch.setattr("sys.argv", [
            "cli", "-i", str(json_file),
            "-c", "show buffer_pool watermark", "--format", "json",
        ])
        assert main() == 0
        result = json.loads(capsys.readouterr().out)
        assert "1234" in result["result"]

    def test_no_command_or_key_exits_2(self, monkeypatch):
        """Missing both -c and -k triggers argparse error (exit 2)."""
        monkeypatch.setattr("sys.argv", ["cli", "-i", "dummy.json"])
        with pytest.raises(SystemExit, match="2"):
            main()

    def test_invalid_command_returns_1(self, monkeypatch, tmp_path):
        """Unknown command returns exit code 1."""
        json_file = tmp_path / "data.json"
        json_file.write_text(json.dumps({"k": "v"}))
        monkeypatch.setattr("sys.argv", [
            "cli", "-i", str(json_file), "-c", "show totally_invalid xyz"
        ])
        assert main() == 1
