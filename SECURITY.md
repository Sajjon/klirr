# Release Verification and Security

This document explains the security measures implemented in klirr's release process to ensure the integrity and authenticity of released binaries.

## Overview

Klirr implements multiple layers of security for its releases:

1. **GPG-signed commits and tags** - All release commits and tags are cryptographically signed
2. **Build attestations** - Each binary includes provenance information about its build process
3. **Multi-platform builds** - Binaries are built on native runners to avoid cross-compilation security issues
4. **Dependency verification** - Build process uses locked dependency versions

## Verification Methods

### GitHub Verification

GitHub automatically verifies releases that include:

-   ✅ Signed commits/tags from the release workflow
-   ✅ Build attestations generated during the build process
-   ✅ Provenance information linking binaries to their source code

When these conditions are met, GitHub will display a "Verified" badge on the release.

### Manual Verification

You can manually verify the integrity of klirr releases:

#### 1. Verify GPG Signatures

```bash
# Download the release tag and verify its signature
git tag -v v0.1.22
```

#### 2. Verify Build Attestations

GitHub provides build attestations for each binary. You can verify these using the GitHub CLI:

```bash
# Install GitHub CLI if you haven't already
# Download the binary you want to verify
curl -L -o klirr-aarch64-apple-darwin "https://github.com/Sajjon/klirr/releases/download/v0.1.22/klirr-aarch64-apple-darwin"

# Verify the attestation
gh attestation verify klirr-aarch64-apple-darwin --repo Sajjon/klirr
```

#### 3. Verify SHA256 Checksums

Each Homebrew formula includes SHA256 checksums that you can verify:

```bash
# Calculate the checksum of your downloaded binary
shasum -a 256 klirr-aarch64-apple-darwin

# Compare with the checksum in the Homebrew formula
curl -s https://raw.githubusercontent.com/Sajjon/homebrew-klirr/main/Formula/klirr.rb | grep sha256
```

## Build Process Security

### Signed Releases

All release tags and commits are signed using SSH signing in the GitHub Actions workflow:

-   **Signing method**: SSH signing with ephemeral keys
-   **Identity**: github-actions[bot]
-   **Verification**: GitHub automatically verifies the signatures

### Build Attestations

Each binary includes a build attestation that contains:

-   **Source repository**: The exact commit that produced the binary
-   **Build environment**: Details about the runner and tools used
-   **Build process**: The workflow that created the binary
-   **Timestamp**: When the build occurred

### Supply Chain Security

-   **Dependency locking**: All Rust dependencies are locked to specific versions
-   **Native builds**: No cross-compilation - each target builds on its native platform
-   **Isolated builds**: Each target builds in a fresh environment
-   **Minimal permissions**: Workflows use minimal required permissions

## Security Best Practices

When using klirr binaries:

1. **Download from official sources**: Only download from GitHub Releases or official Homebrew tap
2. **Verify signatures**: Check that releases show as "Verified" on GitHub
3. **Check checksums**: Verify SHA256 checksums when available
4. **Use attestations**: Use `gh attestation verify` for additional security
5. **Stay updated**: Keep klirr updated to the latest version

## Reporting Security Issues

If you discover a security vulnerability in klirr or its release process, please:

1. **Do not** create a public issue
2. Contact the maintainer directly via email or GitHub private message
3. Provide details about the vulnerability and potential impact
4. Allow time for the issue to be addressed before public disclosure

## Technical Details

The release process implements the following security standards:

-   **SLSA Build Level 2**: Through GitHub's hosted build environment and attestations
-   **Supply Chain Levels for Software Artifacts (SLSA)**: Provenance generation and verification
-   **Sigstore**: Compatible signing and verification methods
-   **OpenSSF Best Practices**: Following security guidelines for open source projects

This multi-layered approach ensures that users can trust the binaries they download and that any tampering or compromise would be easily detectable.
