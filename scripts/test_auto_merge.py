#!/usr/bin/env python3
"""Contract tests for the auto-merge gate.

Every case is a fixture: the gate is decided from the data it is given,
never from a live GitHub call. The tests therefore prove the decision
rules, not the network.
"""

import json
import os
import sys
import tempfile
import unittest

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))

import auto_merge


def pull(number=7, base="master", head="abc123", author="vshengbro", draft=False):
    """One pull request as the GitHub API reports it."""
    return {
        "number": number,
        "draft": draft,
        "base": {"ref": base},
        "head": {"ref": "feature/x", "sha": head},
        "user": {"login": author},
    }


def green(*names):
    """Check runs where every named check succeeded."""
    return {
        name: [{"name": name, "status": "completed", "conclusion": "success"}]
        for name in names
    }


class DecisionTests(unittest.TestCase):
    """The gate's decisions."""

    def setUp(self):
        self.config = {
            "base_branch": "master",
            "trusted_authors": ["vshengbro"],
            "required_checks": ["setup", "check", "tests"],
            "merge_method": "squash",
        }

    def test_green_pull_request_is_allowed(self):
        allowed, reason = auto_merge.decide(
            pull(), green("setup", "check", "tests"), [], "o/r", self.config
        )
        self.assertTrue(allowed)
        self.assertIn("green", reason)

    def test_wrong_base_branch_is_skipped(self):
        allowed, reason = auto_merge.decide(
            pull(base="develop"), green("setup", "check", "tests"), [], "o/r", self.config
        )
        self.assertFalse(allowed)
        self.assertIn("develop", reason)

    def test_draft_is_skipped(self):
        allowed, reason = auto_merge.decide(
            pull(draft=True), green("setup", "check", "tests"), [], "o/r", self.config
        )
        self.assertFalse(allowed)
        self.assertIn("draft", reason)

    def test_untrusted_author_is_skipped(self):
        allowed, reason = auto_merge.decide(
            pull(author="someone-else"), green("setup", "check", "tests"), [], "o/r", self.config
        )
        self.assertFalse(allowed)
        self.assertIn("not trusted", reason)

    def test_missing_required_check_is_a_failure(self):
        allowed, reason = auto_merge.decide(
            pull(), green("setup", "check"), [], "o/r", self.config
        )
        self.assertFalse(allowed)
        self.assertIn("never ran", reason)

    def test_failed_required_check_is_a_failure(self):
        grouped = green("setup", "check", "tests")
        grouped["tests"] = [
            {"name": "tests", "status": "completed", "conclusion": "failure"}
        ]
        allowed, reason = auto_merge.decide(pull(), grouped, [], "o/r", self.config)
        self.assertFalse(allowed)
        self.assertIn("concluded failure", reason)

    def test_running_required_check_is_a_failure(self):
        grouped = green("setup", "check")
        grouped["tests"] = [{"name": "tests", "status": "in_progress", "conclusion": None}]
        allowed, reason = auto_merge.decide(pull(), grouped, [], "o/r", self.config)
        self.assertFalse(allowed)
        self.assertIn("in_progress", reason)


class VersionGuardTests(unittest.TestCase):
    """The version guard that keeps a released crate from being republished."""

    def setUp(self):
        self.guard = {
            "path": "Cargo.toml",
            "section": "workspace.package",
            "when_files_match": [r"\.rs$", r"^Cargo\.toml$"],
        }

    def test_version_is_read_from_the_section(self):
        text = '[package]\nversion = "1.0.0"\n\n[workspace.package]\nversion = "0.37.7"\n'
        self.assertEqual(
            auto_merge.parse_version(text, "workspace.package"), "0.37.7"
        )

    def test_untouched_files_skip_the_guard(self):
        reason = auto_merge.version_guard_satisfied(
            "o/r", pull(), ["README.md"], self.guard
        )
        self.assertIsNone(reason)

    def test_changed_source_without_a_version_bump_is_rejected(self):
        base = '[workspace.package]\nversion = "0.37.6"\n'
        head = '[workspace.package]\nversion = "0.37.6"\n'
        reason = auto_merge.compare_versions(
            base, head, "Cargo.toml", "workspace.package", ["ui/src/lib.rs"]
        )
        self.assertIsNotNone(reason)
        self.assertIn("still declares version", reason)

    def test_changed_source_with_a_version_bump_is_allowed(self):
        base = '[workspace.package]\nversion = "0.37.6"\n'
        head = '[workspace.package]\nversion = "0.37.7"\n'
        reason = auto_merge.compare_versions(
            base, head, "Cargo.toml", "workspace.package", ["ui/src/lib.rs"]
        )
        self.assertIsNone(reason)

    def test_guard_selects_only_the_matching_paths(self):
        touched = auto_merge.touched_by_guard(
            ["ui/src/lib.rs", "README.md"], self.guard
        )
        self.assertEqual(touched, ["ui/src/lib.rs"])


class ConfigTests(unittest.TestCase):
    """The configuration file."""

    def test_configuration_must_be_an_object(self):
        with tempfile.TemporaryDirectory() as scratch:
            path = os.path.join(scratch, "config.json")
            with open(path, "w", encoding="utf-8") as handle:
                handle.write("[]")
            with self.assertRaises(auto_merge.GateError):
                auto_merge.load_config(path)

    def test_configuration_round_trips(self):
        with tempfile.TemporaryDirectory() as scratch:
            path = os.path.join(scratch, "config.json")
            payload = {"base_branch": "master", "required_checks": ["check"]}
            with open(path, "w", encoding="utf-8") as handle:
                json.dump(payload, handle)
            self.assertEqual(auto_merge.load_config(path), payload)


if __name__ == "__main__":
    unittest.main()
