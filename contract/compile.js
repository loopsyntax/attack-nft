// This file does two things:
//
// 1. Compile the Rust contract using cargo (see buildCmd below). This will
//    create a wasm file in the 'build' folder.
// 2. Create a symbolic link (symlink) to the generated wasm file in the root
//    project's `out` folder, for easy use with near-cli.
//
const sh = require("shelljs");

// Figure out which directory the user called this script from, which we'll use
// later to set up the symlink.
const calledFromDir = sh.pwd().toString();

// For the duration of this script, we want to operate from within the
// Rust project's folder. Let's change into that directory.
sh.cd(__dirname);

// You can call this script with `node compile.js` or `node compile.js
// --debug`. Let's set a variable to track whether `--debug` was used.
const debug = process.argv.pop() === "--debug";

// You can call this script with `node compile.js` or `node compile.js --debug`.
// Let's set a variable to track whether `--debug` was used.
// Note: see other flags in ./cargo/config. Unfortunately, you cannot set the
// `--target option` in Cargo.toml.
const buildCmd = debug
  ? "cargo build --target wasm32-unknown-unknown"
  : "cargo build --target wasm32-unknown-unknown --release";

// Execute the build command, storing exit code for later use
const { code } = sh.exec(buildCmd);

// Assuming this is compiled from the root project directory, link the compiled
// contract to the `out` folder –
// When running commands like `near deploy`, near-cli looks for a contract at
// <CURRENT_DIRECTORY>/out/main.wasm
if (code === 0 && calledFromDir !== __dirname) {
  const linkDir = `${calledFromDir}/out`;
  const link = `${linkDir}/main.wasm`;
  // const packageName = require("fs")
  //   .readFileSync(`${__dirname}/Cargo.toml`)
  //   .toString()
  //   .match(/name = "([^"]+)"/)[1];
  const outFile = `/${__dirname}/target/wasm32-unknown-unknown/${
    debug ? "debug" : "release"
  }/*.wasm`;
  sh.mkdir("-p", linkDir);
  sh.rm("-f", link);
  //fixes #831: copy-update instead of linking .- sometimes sh.ln does not work on Windows
  sh.cp("-u", outFile, link);
}

// exit script with the same code as the build command
process.exit(code);
