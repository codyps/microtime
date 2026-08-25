# Repository TODO

## Completed in this change

- [x] Make timestamp and duration overflow behavior consistent across build profiles.
- [x] Add checked arithmetic and a `Duration::as_micros` accessor.
- [x] Replace panic-prone `SystemTime` conversions with `TryFrom` implementations.
- [x] Run CI for pushes and pull requests on stable and beta Rust.
- [x] Add release-plz configuration, release PR automation, and crates.io publishing.
- [x] Gate publication on merging a release-plz release PR.
- [x] Add automated GitHub Actions dependency updates.

## Repository setup required

- [ ] Configure a crates.io trusted publisher for `codyps/microtime`, using the
      `release-plz.yml` workflow. Do not add `CARGO_REGISTRY_TOKEN` when trusted
      publishing is enabled.
- [ ] Add a fine-grained `RELEASE_PLZ_TOKEN` repository secret with Contents and
      Pull requests read/write access. The workflow falls back to `GITHUB_TOKEN`,
      but release PRs created with that fallback will not trigger CI automatically.
- [x] Allow workflows to write repository contents and approve pull requests (verified
      in the live GitHub Actions settings on 2026-08-25).
- [ ] Protect `master` and require `Quality`, `Test (stable)`, and `Test (beta)`
      before merging.
- [ ] Remove the obsolete Bors app/integration from the GitHub repository after
      merging the staged `bors.toml` deletion.
- [ ] Review and merge the first release-plz PR, then verify its tag, GitHub release,
      and crates.io publication.
- [ ] Mark this API change as breaking in its Conventional Commit message (for
      example, `fix!:` plus a `BREAKING CHANGE:` footer) so the first automated
      release advances from `0.3.x` to `0.4.0`.

## Ongoing maintenance

- [ ] Use Conventional Commit messages so release-plz can classify changes and
      choose versions accurately.
- [ ] Review monthly Dependabot updates for pinned GitHub Actions.
