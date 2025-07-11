## How to clone or push via HTTPS
GitHub requires personal access tokens for HTTPS operations since support for password authentication was removed on August 13, 2021.

See https://docs.github.com/en/get-started/git-basics/about-remote-repositories#cloning-with-https-urls

### Using Git Credential Manager

#### Step 1
Install Git Credential Manager: https://github.com/git-ecosystem/git-credential-manager/blob/main/README.md

After installing GCM, run:
```bash
git-credential-manager configure
git config --global credential.credentialStore plaintext
```
Note: Storing credentials in plaintext is insecure. However, since we'll specify a test-only repository when creating the access key, this is acceptable for our purposes.

#### Creating an access token
- Go to https://github.com/settings/personal-access-tokens
- When creating the access token, select "Fine-grained tokens"
- Under Repository access, choose "Only select repositories" and specify a test repository
- Under Repository permissions, set Contents permission to "Read and write"
- Create the token and copy it

### Performing Git operations

Enable Git's built-in tracing to monitor fetch/push processes. Example:
```bash
# See https://stackoverflow.com/questions/6178401/how-can-i-debug-git-git-shell-related-problems

GIT_TRACE=1 GIT_TRACE_SETUP=1 GIT_TRACE_PACKET=1 git clone https://github.com/simplexforce/git-remote-helper.git
```

We'll use the HTTPS protocol for observation since SSH is built into Git and doesn't use git-remote-helper.
Use git-remote-proxy to trace git-remote-helper input/output. Example:
```bash
RUST_LOG=debug GIT_PROXY_HELPER=https git clone proxy://github.com/simplexforce/git-helper-test.git