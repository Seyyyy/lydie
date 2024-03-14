const fs = require("fs");
const path = require("path");

// package.jsonのパスを指定
const packageJsonPath = path.resolve(__dirname, "../", "package.json");
const distPackageJsonPath = path.resolve(__dirname, "../dist", "package.json");

// package.jsonを読み込む
const packageJson = require(packageJsonPath);

// nameとdescription以外のプロパティを削除
const newPackageJson = {
  name: packageJson.name,
  collaborators: packageJson.collaborators,
  description: packageJson.description,
  version: packageJson.version,
  license: packageJson.license,
  repository: packageJson.repository,
  homepage: packageJson.homepage,
  files: packageJson.files,
  main: packageJson.main,
  module: packageJson.module,
  exports: packageJson.exports,
};

// 新しいpackage.jsonをdistディレクトリに書き込む
fs.writeFileSync(distPackageJsonPath, JSON.stringify(newPackageJson, null, 2));
