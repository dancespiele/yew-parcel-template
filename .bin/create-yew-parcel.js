#!/usr/bin/env node
const { spawn } = require("child_process");
const fs   = require("fs");

let folderName = '.';

if (process.argv.length >= 3) {
  folderName = process.argv[2];   
  if (!fs.existsSync(folderName)) {
    fs.mkdirSync(folderName);
  }
}

const clone = spawn("git", ["clone", "https://github.com/spielrs/yew-parcel-template.git", folderName]);

clone.on("close", (code) => {
  if (code !== 0) {
    handleError("install", code);
  } else {
    console.log("Yew and parcel ready. Installing dependencies ...");
    
    const install = spawn('npm', ['install'], { cwd: folderName });
    install.on("close", (code) => {
      if (code !== 0) {
        handleError("install", code);
      } else {
        console.log(" Installed dependencies ✅ ");
        console.log(" Mucha locura! Bzzzzzzzzzzzzz")
        console.log(" If you like Yew Parcel Templa, help us supporting the project:"
        + "\nhttps://gitcoin.co/grants/1048/yew-styles\nhttps://github.com/sponsors/dancespiele\nhttps://paypal.me/dancespiele?locale.x=en_US");
      }
    });
  }
});

function handleError(type, errCode) {
    console.error()
    process.exit(errCode);
}
