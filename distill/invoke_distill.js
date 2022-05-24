#!/usr/bin/env node

const spawn = require('child_process').spawn;
const path = require('path');
const os = require('os');
const fs = require('fs');

const binaryPaths = {
  "Linux": path.join(__dirname, "dist/ubuntu/distill"),
  "Windows_NT": path.join(__dirname, "dist/windows/distill.exe"),
  "Darwin": path.join(__dirname, "dist/osx/distill"),
};

// Collecting command line arguments
const [_interpreter, _scriptName, ...args] = process.argv;

let binaryPath = binaryPaths[os.type()];

if (!fs.existsSync(binaryPath)){
  throw new Error('Unsupported operating system!');
} 

const childProcess = spawn(binaryPath, args);
childProcess.stdout.on('data', (data) => console.log(data.toString()));
childProcess.stderr.on('data', (data) => console.log(data.toString()));