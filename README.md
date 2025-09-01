# Server Project

A Rust server application configured for Ubuntu Linux deployment.

## Development Setup

### Local Development (Windows)
```bash
# Build for Windows (development)
cargo build

# Run locally
cargo run
```

### Building for Linux

#### Option 1: GitHub Actions (Recommended)
Push to GitHub and the workflow will automatically build Linux binaries.

#### Option 2: Build on Ubuntu Server
```bash
# Copy source to Ubuntu server
scp -r . user@server:/path/to/project/

# SSH to server and build
ssh user@server
cd /path/to/project
chmod +x build-linux.sh
./build-linux.sh
```

#### Option 3: Docker (if available)
```bash
# Install Docker Desktop first, then:
cross build --target x86_64-unknown-linux-gnu --release
```

## Configuration

- **Target**: x86_64-unknown-linux-gnu (Ubuntu Linux)
- **Edition**: 2021
- **Profile**: Release builds are optimized for production

## Files

- `src/main.rs` - Main application code
- `.cargo/config.toml` - Cargo configuration
- `build-linux.sh` - Linux build script
- `.github/workflows/build.yml` - CI/CD pipeline