# m1_acl
Projet d'ACL

## Build and run

Since WASM and Rust (together) are still in development and not really stable, the build process is a bit tricky.

First, you need to go in the front folder and run `npm install` to install the dependencies.

Then in the back, run the wasm-pack command.

- 1: `wasm-pack build --release`
- 2: `npm link ..\back\pkg\` and then add the dependency in `package.json`

Sometimes you have an 404 error saying that belote_back cannot be found... Just CTRL+X the line in package.json, re-run `npm-install` and then re-add the dependency. Then re-run the link command.

By the end you should be able to run the frontend in dev mode with `npm run dev`.
