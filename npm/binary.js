const { Binary } = require("@cloudflare/binary-install");
const os = require("os");

const { version, name, repository } = require("./package.json");

const getPlatform = () => {
  const type = os.type();
  const arch = os.arch();

  if (type === "Windows_NT" && arch === "x64") {
    return "x86_64-pc-windows-msvc";
  }
  if (type === "Darwin" && arch === "x64") {
    return "x86_64-apple-darwin";
  }
  throw new Error(`Unsupported platform: ${type} ${arch}`);
};

const getBinary = () => {
  const platform = getPlatform();
  const url = `${repository.url.split(".git")[0]}/releases/download/v${version}/${name}-v${version}-${platform}.tar.gz`;
  return new Binary(url, { name });
};

const run = () => {
  const binary = getBinary();
  binary.run();
};

const install = () => {
  const binary = getBinary();
  binary.install();
};

const uninstall = () => {
  const binary = getBinary();
  binary.uninstall();
};

module.exports = {
  install,
  run,
  uninstall,
};
