const { Binary } = require("binary-install");
const os = require("os");
const { join } = require("path");
const cTable = require("console.table");

const error = (msg) => {
	console.error(msg);
	process.exit(1);
};

let { version, repository } = require("./package.json");
// temp rewrite of version to make things work
version = "0.1.0";
const name = "squeeel-cli";

const supportedPlatforms = [
	{
		TYPE: "Windows_NT",
		ARCHITECTURE: "x64",
		RUST_TARGET: "x86_64-pc-windows-msvc",
		BINARY_NAME: "squeeel-cli.exe",
	},
	{
		TYPE: "Linux",
		ARCHITECTURE: "x64",
		RUST_TARGET: "x86_64-unknown-linux-musl",
		BINARY_NAME: "squeeel-cli",
	},
	{
		TYPE: "Darwin",
		ARCHITECTURE: "x64",
		RUST_TARGET: "x86_64-apple-darwin",
		BINARY_NAME: "squeeel-cli",
	},
	{
		TYPE: "Darwin",
		ARCHITECTURE: "arm64",
		RUST_TARGET: "x86_64-apple-darwin",
		BINARY_NAME: "squeeel-cli",
	},
];

const getPlatformMetadata = () => {
	const type = os.type();
	const architecture = os.arch();

	for (const supportedPlatform of supportedPlatforms) {
		if (
			type === supportedPlatform.TYPE &&
			architecture === supportedPlatform.ARCHITECTURE
		) {
			return supportedPlatform;
		}
	}

	error(
		`Platform with type "${type}" and architecture "${architecture}" is not supported by ${name}.\nYour system must be one of the following:\n\n${cTable.getTable(
			supportedPlatforms,
		)}`,
	);
};

const getBinary = () => {
	const platformMetadata = getPlatformMetadata();
	// the url for this binary is constructed from values in `package.json`
	// https://github.com/SorenHolstHansen/squeeel/releases/download/v1.0.0/binary-install-example-v1.0.0-x86_64-apple-darwin.tar.gz
	const url = `${repository.url}/releases/download/rust_v${version}/${name}-v${version}-${platformMetadata.RUST_TARGET}.tar.gz`;
	return new Binary(platformMetadata.BINARY_NAME, url, version, {
		installDirectory: join(__dirname, "node_modules", ".bin"),
	});
};

const run = () => {
	const binary = getBinary();
	binary.run();
};

const install = () => {
	const binary = getBinary();
	binary.install();
};

module.exports = {
	install,
	run,
};
