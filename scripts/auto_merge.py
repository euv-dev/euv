#!/usr/bin/env python3
"""Merge pull requests whose required checks are green.

The gate is fail-closed. A pull request is merged only when every one of
these holds:

* it targets the configured base branch,
* it is not a draft,
* its author is trusted,
* every required check has completed successfully on the CURRENT head
  commit - a missing check is a failure, never a pass,
* the configured version guard is satisfied.

The head commit is verified again immediately before the merge, so a pull
request that moved while the checks were read is rejected instead of
merged at a commit nobody verified.

The script never trusts the pull request's own branch: it reads the
checks from the head SHA it just fetched, and it compares the version
guard against the base branch, so a pull request cannot satisfy the gate
by editing the gate itself.
"""

import argparse
import base64
import json
import os
import re
import subprocess
import sys

DEFAULT_BASE_BRANCH = "master"
TRUSTED_AUTHOR_DEFAULT = "vshengbro"


class GateError(RuntimeError):
    """A hard failure that stops the whole run."""


def run_gh(arguments, capture=True):
    """Runs one gh subcommand and returns its stdout."""
    environment = dict(os.environ)
    environment.setdefault("GH_PAGER", "cat")
    completed = subprocess.run(
        ["gh", *arguments],
        capture_output=capture,
        text=True,
        env=environment,
    )
    if completed.returncode != 0:
        raise GateError(
            "gh %s failed (%d): %s"
            % (" ".join(arguments), completed.returncode, (completed.stderr or "").strip())
        )
    return completed.stdout


def gh_json(path, paginate=False):
    """Reads one GitHub REST endpoint as JSON."""
    arguments = ["api", path]
    if paginate:
        arguments.append("--paginate")
    return json.loads(run_gh(arguments) or "null")


def check_runs(repository, commit):
    """Returns the check runs of one commit, grouped by their name.

    The value of every name is the list of that name's runs, because one
    name can belong to several workflows.
    """
    grouped = {}
    runs = gh_json("repos/%s/commits/%s/check-runs" % (repository, commit)).get(
        "check_runs", []
    )
    for run in runs:
        grouped.setdefault(run.get("name") or "", []).append(run)
    return grouped


def required_checks_green(grouped, required):
    """Reports why the required checks are not green, or None when they are.

    A required check that no workflow reported is a failure: the pull
    request is not verified, and an unverified pull request is not green.
    """
    for name in required:
        runs = grouped.get(name)
        if not runs:
            return "required check %r never ran on this commit" % name
        for run in runs:
            if run.get("status") != "completed":
                return "required check %r is still %s" % (name, run.get("status"))
            if run.get("conclusion") != "success":
                return "required check %r concluded %s" % (name, run.get("conclusion"))
    return None


def parse_version(text, path):
    """Reads one version string out of a TOML file.

    The file is read as text on purpose: the runner's Python is the one
    that has to parse it, and a plain scan keeps this script runnable on
    any interpreter the repository already uses.

    A section of None means the first version the file declares, which is
    the root package version in a manifest that opens with ``[package]``.
    """
    section = None
    pattern = re.compile(r'^\s*version\s*=\s*"([^"]+)"')
    for line in text.splitlines():
        stripped = line.strip()
        if stripped.startswith("[") and stripped.endswith("]"):
            section = stripped[1:-1].strip()
            continue
        if path and section is not None and section != path:
            continue
        match = pattern.match(line)
        if match:
            return match.group(1)
    return None


def file_at(repository, path, ref):
    """Reads one file at one ref, or None when it does not exist there."""
    try:
        payload = gh_json(
            "repos/%s/contents/%s?ref=%s" % (repository, path, ref)
        )
    except GateError:
        return None
    if not isinstance(payload, dict) or payload.get("type") != "file":
        return None
    content = payload.get("content") or ""
    return base64.b64decode(content).decode("utf-8", "replace")


def pull_request_files(repository, number):
    """Lists the paths a pull request changes."""
    files = gh_json(
        "repos/%s/pulls/%s/files?per_page=100" % (repository, number), paginate=True
    )
    if not isinstance(files, list):
        return []
    return [entry.get("filename") for entry in files if entry.get("filename")]


def touched_by_guard(files, guard):
    """The changed paths the version guard applies to."""
    triggers = guard.get("when_files_match") or []
    return [
        name for name in files if any(re.search(pattern, name) for pattern in triggers)
    ]


def compare_versions(base_text, head_text, path, section, touched):
    """Compares the base and head versions and reports the failure, or None.

    The comparison is pure: it takes the two file contents, so the rule can
    be tested without a network call.
    """
    base = parse_version(base_text or "", section)
    head = parse_version(head_text or "", section)
    if head is None:
        return "%s carries no version at %s" % (path, section or "the root")
    if base is not None and head == base:
        return "%s still declares version %s after changing %s" % (
            path,
            head,
            ", ".join(sorted(touched)[:5]),
        )
    return None


def version_guard_satisfied(repository, pull, files, guard):
    """Reports why the version guard is not satisfied, or None when it is."""
    if not guard:
        return None
    path = guard.get("path") or "Cargo.toml"
    section = guard.get("section")
    touched = touched_by_guard(files, guard)
    if not touched:
        return None
    return compare_versions(
        file_at(repository, path, pull["base"]["ref"]),
        file_at(repository, path, pull["head"]["sha"]),
        path,
        section,
        touched,
    )


def decide(pull, grouped, files, repository, config):
    """Decides whether one pull request may be merged.

    Returns (allowed, reason). The reason is always printed, so a skipped
    pull request is as visible as a merged one.
    """
    base_branch = config.get("base_branch") or DEFAULT_BASE_BRANCH
    trusted = config.get("trusted_authors") or [TRUSTED_AUTHOR_DEFAULT]
    if pull.get("base", {}).get("ref") != base_branch:
        return False, "targets %s, not %s" % (pull.get("base", {}).get("ref"), base_branch)
    if pull.get("draft"):
        return False, "is a draft"
    author = (pull.get("user") or {}).get("login")
    if author not in trusted:
        return False, "author %s is not trusted" % author
    failure = required_checks_green(grouped, config.get("required_checks") or [])
    if failure:
        return False, failure
    failure = version_guard_satisfied(repository, pull, files, config.get("version_guard"))
    if failure:
        return False, failure
    return True, "every required check is green"


def merge(pull, repository, config):
    """Merges one pull request with the verified head commit."""
    arguments = [
        "pr",
        "merge",
        str(pull["number"]),
        "--repo",
        repository,
        "--match-head-commit",
        pull["head"]["sha"],
        config.get("merge_method") or "squash",
    ]
    if config.get("delete_branch", True):
        arguments.append("--delete-branch")
    run_gh(arguments)
    return "merged"


def dispatch_workflows(repository, workflows, ref):
    """Starts the workflows that a merge must run.

    A merge pushed with the workflow's own GITHUB_TOKEN does NOT trigger
    the workflows that the merge itself would trigger: GitHub suppresses
    them on purpose, to stop a workflow from recursively triggering
    itself. The release pipeline must still run after a merge, so the
    merge job starts those workflows explicitly.
    """
    started = []
    for workflow in workflows or []:
        run_gh(
            [
                "workflow",
                "run",
                workflow,
                "--repo",
                repository,
                "--ref",
                ref,
            ]
        )
        started.append(workflow)
    return started


def load_config(path):
    """Reads the gate configuration."""
    with open(path, "r", encoding="utf-8") as handle:
        config = json.load(handle)
    if not isinstance(config, dict):
        raise GateError("the configuration must be a JSON object")
    return config


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", required=True, help="owner/name")
    parser.add_argument("--config", required=True, help="path of the gate configuration")
    parser.add_argument(
        "--dry-run",
        action="store_true",
        help="decide and report without merging",
    )
    arguments = parser.parse_args()
    config = load_config(arguments.config)
    repository = arguments.repo
    merged = []
    skipped = []
    pulls = gh_json("repos/%s/pulls?state=open&per_page=50" % repository)
    for pull in pulls or []:
        number = pull.get("number")
        grouped = check_runs(repository, pull["head"]["sha"])
        files = pull_request_files(repository, number)
        allowed, reason = decide(pull, grouped, files, repository, config)
        if not allowed:
            skipped.append((number, reason))
            print("skip #%s: %s" % (number, reason), flush=True)
            continue
        if arguments.dry_run:
            print("would merge #%s: %s" % (number, reason), flush=True)
            continue
        outcome = merge(pull, repository, config)
        merged.append(number)
        print("%s #%s: %s" % (outcome, number, reason), flush=True)
        started = dispatch_workflows(
            repository,
            config.get("post_merge_dispatch"),
            config.get("base_branch") or DEFAULT_BASE_BRANCH,
        )
        for workflow in started:
            print("dispatched %s on %s" % (workflow, config.get("base_branch")), flush=True)
    print(
        "auto-merge summary: merged %d, skipped %d" % (len(merged), len(skipped)),
        flush=True,
    )
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except GateError as error:
        print("auto-merge failed: %s" % error, file=sys.stderr)
        sys.exit(1)
