---
name: release-checklist
description: >-
  Enforce the mandatory 3-phase release procedure for OniAnvil: verify all CI builds on GitHub Actions pass, synchronize version numbers across Cargo.toml, READMEs, and HTML files, and finally tag and publish the release. Use whenever preparing, cutting, bumping, or deploying a new release or version tag.
---

# Release Checklist & Workflow for OniAnvil

This skill provides step-by-step guidance for safely releasing a new version of OniAnvil.

**Mandatory rule: First push the version tag, wait for GitHub Actions desktop builds and release assets to succeed, and only then build & deploy the WebAssembly version and update READMEs and HTML pages without modifying or moving the tag.**

---

## Phase 1: Local Preflight & Version Tagging

1. **Local Test & Build Verification**:
   ```powershell
   cargo test --release
   cargo check --target wasm32-unknown-unknown
   cargo clippy --all-targets -- -D warnings
   ```
   All tests must pass with 0 failures, 0 warnings, and 0 errors.

2. **Bump Version in `Cargo.toml`**:
   Set `version = "X.Y.Z"`.

3. **Commit & Push Version Bump**:
   ```powershell
   git commit -am "chore(release): bump version to vX.Y.Z"
   git push origin main
   ```

4. **Create Annotated Tag & Push**:
   ```powershell
   git tag -a vX.Y.Z -m "OniAnvil vX.Y.Z: <Release Title and Summary>"
   git push origin vX.Y.Z
   ```

---

## Phase 2: GitHub Actions CI Build Verification

1. **Monitor Release Pipeline**:
   Query GitHub Actions runs for tag `vX.Y.Z` using PowerShell:
   ```powershell
   Invoke-RestMethod -Uri "https://api.github.com/repos/rbm13s/OniAnvil/actions/runs?per_page=3" |
     Select-Object -ExpandProperty workflow_runs |
     Select-Object id, status, conclusion, head_branch, name
   ```
   Wait until all matrix targets succeed:
   - WebAssembly (HTML5)
   - Linux (Debian / Ubuntu x86_64)
   - macOS Universal (Apple Silicon + Intel)
   - Windows (x86_64)
   - Publish GitHub Release & Deploy to itch.io (generates changelog, releases assets, pushes to itch.io via Butler)

2. **Verify Published Release Assets & itch.io Channels**:
   ```powershell
   (Invoke-RestMethod -Uri "https://api.github.com/repos/rbm13s/OniAnvil/releases/tags/vX.Y.Z").assets |
     Select-Object name, size
   ```
   Ensure all 5 release packages are attached:
   - `OniAnvil-X.Y.Z-windows-x86_64.zip`
   - `onianvil_X.Y.Z_amd64.deb`
   - `OniAnvil-X.Y.Z-linux-x86_64.tar.gz`
   - `OniAnvil-X.Y.Z-macos-universal.zip`
   - `OniAnvil-X.Y.Z-html5.zip`

   Verify Butler logs in the GitHub Actions `Publish GitHub Release & Deploy to itch.io` job:
   - `rbm13s/onianvil:windows`
   - `rbm13s/onianvil:linux`
   - `rbm13s/onianvil:osx`
   - `rbm13s/onianvil:html5`

> [!CAUTION]
> NEVER proceed to Phase 3 if any desktop build failed or if release assets are missing or unverified.

---

## Phase 3: WebAssembly App Rebuild & Documentation Synchronization

*Only after Phase 2 passes and all binary assets are verified on GitHub Releases:*

1. **Rebuild WebAssembly (WASM) Online App for GitHub Pages**:
   ```powershell
   .\build_wasm.ps1
   # Or directly via Trunk:
   trunk build index.html --release --dist docs/app --public-url ./
   ```
   Verify that `docs/app/` contains the updated `.wasm`, `.js`, `index.html`, and `fonts/` bundle for `vX.Y.Z`.

2. **Update Markdown Documentation**:
   - Primary `README.md` at root
   - All 36 localized files in `docs/README-*.md`
   Ensure badges, headers, and links reflect `vX.Y.Z`.

3. **Update HTML Website Pages**:
   - `docs/index.html` and all 36 localized HTML files in `docs/*.html`.
   - Update:
     - Version badge: `<span class="badge-ver">vX.Y.Z</span>`
     - Hero banner: `vX.Y.Z Available • ...` (or localized counterpart)
     - Section headings and platform download links.

4. **Commit & Push to `main`**:
   ```powershell
   git add docs/app/
   git commit -am "docs(release): sync documentation and deploy web app for vX.Y.Z"
   git push origin main
   ```
   > [!IMPORTANT]
   > **DO NOT re-tag or move the `vX.Y.Z` tag!** Documentation and web app deployments belong only in `main`. Moving a tag will trigger redundant GitHub Actions rebuilds.

5. **Verify GitHub Pages Deployment**:
   Wait for `pages build and deployment` on GitHub Actions to succeed:
   ```powershell
   Invoke-RestMethod -Uri "https://api.github.com/repos/rbm13s/OniAnvil/actions/runs?per_page=3" |
     Select-Object -ExpandProperty workflow_runs |
     Where-Object { $_.name -like '*pages*' } |
     Select-Object id, status, conclusion
   ```
