# Automatic releases

Pushes to `main` keep the release-plz PR up to date. The **Automatic release**
workflow checks it daily at 09:23 UTC and merges it when at least 14 elapsed days
have passed since the latest non-draft, non-prerelease `microtime-*` GitHub release.
GitHub may delay scheduled runs; the interval is a minimum, not an exact deadline.
A manual release resets the interval. With no published release, automation stops
and requires a first manual release to establish the interval.

The merge triggers the existing release-plz publishing workflow. Keep
`release_always = false`: ordinary pushes must not publish unmerged release PRs.
We use a merge commit to preserve release-plz's association with the prepared PR.

The scheduled reviewer follows the BCR approach of periodically checking eligible
PRs and merging the examined head SHA:
- [BCR schedule](https://github.com/bazelbuild/bazel-central-registry/blob/main/.github/workflows/review_prs.yml)
- [BCR reviewer](https://github.com/bazelbuild/continuous-integration/blob/957b0ac44549a70ccb951361b804d07d955aeb79/actions/bcr-pr-reviewer/index.js)
- [Release-plz release behavior](https://release-plz.dev/docs/usage/release)

The 14-day policy is ours; BCR's module-maintainer approval policy is not reused.
Eligibility requires exactly one non-draft, same-repository `release-plz-*` PR
into the default branch, authored by the release token account, changing only
`Cargo.toml`, `Cargo.lock`, and `CHANGELOG.md`. The latest PR CI run on its exact
head must pass Quality and both stable/beta tests. GitHub must report the PR cleanly
mergeable. The reviewer rechecks the PR and interval immediately before merging,
and pins the merge request to the examined head SHA. It shares a concurrency group
with the release PR writer. It does not submit approval reviews or bypass rules.

`RELEASE_PLZ_TOKEN` must be a PAT for the same account used by the release PR writer,
with repository Contents and Pull requests write access and Actions read access.
There is deliberately no `GITHUB_TOKEN` fallback for merging: the merge's push
must trigger the publishing workflow. Token expiration or missing access fails the
job rather than silently merging without publishing.

A merged release PR newer than the latest published GitHub release blocks further
automatic merges. If publication fails, repair/retry the original publishing run
before continuing; do not create a newer release PR to work around it. Manual
merges remain available for urgent releases and can bypass the 14-day policy.

To preview, run `python3 .github/scripts/auto_release.py` with authenticated `gh`,
or manually dispatch **Automatic release** with `dry_run` enabled (the default).
Dispatching with `dry_run` disabled enforces the same interval and eligibility as
the schedule. No workflow waits for two weeks on a runner: each daily check has
a five-minute job limit, a three-minute script step limit, and 30-second API-call
limits. Failed or ineligible attempts wait for the next daily run.

Run policy tests with:

```sh
python3 -m unittest discover -s .github/scripts -p 'test_*.py'
```
