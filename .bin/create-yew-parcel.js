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
        console.log(" If you like Yew Parcel Template, help us supporting the project:"
        + "\nBAT rewards in case that you use Brave Browser in https://github.com/dancespiele" 
        + "\nUsing this link https://www.binance.com/en/register?ref=DB8EPXF0 to create an account in Binance (get 10% fee back for every trading)");
      }
    });
  }
});

function handleError(type, errCode) {
    console.error()
    process.exit(errCode);
}
