
# Git Remote Helper
A simple Git remote helper implementation.

What is a Git remote helper? See [Git remote helpers documentation](https://git-scm.com/docs/gitremote-helpers)

## TODO
 - The in-memory mock Remote implementation isn't very practical as it doesn't persist state between uses. We should implement a solution that uses another local repository as the Remote.
 - Implement pktline and pack parsing, or leverage gitoxide.
 - Implement commands like push, fetch, connect, and stateless-connect.
 - More documents.
 - Refactor and optimize the design to make it simpler.
 - Enhance observability to better assist in understanding Git's underlying processes and make debugging easier.
 - More considerations for security and performance.
 - Choose a better crate name.

## Getting Start
### Build and install git-remote-gaia to bin path
```bash
./install.sh
```

### Create a new git repository for test
```bash
mkdir test-repo
cd test-repo

git init .

# Add some files and commit.
echo hello > README.md
git add .
git commit -m "hello test"

# Add a remote with `demo://` prefix.
git remote add origin demo://example.com/hello/test-repo
```

### Try

```bash
RUST_LOG=debug git push origin main
# or
RUST_LOG=debug git fetch origin main
```

Modify the code in [git-remote-demo/src/main.rs](git-remote-demo/src/main.rs) and test your changes. For example, you can add or remove refs and capabilities.

### Tips

Use `cd -` to quickly switch between directories.
For example:
```bash
# Assuming the current directory is git-remote-helper,
# modify some code and recompile:
./install.sh

# Go to the test repository
cd ../test-repo

# Test the git-remote-helper with git push
RUST_LOG=debug git push origin main

# Switch back to git-remote-helper
cd -
# Modify code and recompile again
./install.sh

# Switch back to test-repo for another test
cd -
RUST_LOG=debug git push origin main
```