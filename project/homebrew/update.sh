version=v0.0.1

curl -s -L "https://github.com/{{project-name}}/{{project-name}}/releases/download/${version}/{{project-name}}-${version}-x86_64-apple-darwin.tar.gz" > osx.gz
shasum -a 256 osx.gz
rm osx.gz

curl -s -L "https://github.com/{{project-name}}/{{project-name}}/releases/download/${version}/{{project-name}}-${version}-x86_64-unknown-linux-gnu.tar.gz" > linux.gz
shasum -a 256 linux.gz
rm linux.gz
