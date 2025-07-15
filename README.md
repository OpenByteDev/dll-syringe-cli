# dll-syringe CLI

[![CI](https://github.com/OpenByteDev/dll-syringe-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/OpenByteDev/dll-syringe-cli/actions/workflows/ci.yml) [![dependency status](https://deps.rs/repo/github/openbytedev/dll-syringe-cli/status.svg)](https://deps.rs/repo/github/openbytedev/dll-syringe-cli) [![MIT](https://img.shields.io/crates/l/dll-syringe-cli.svg)](https://github.com/OpenByteDev/dll-syringe-cli/blob/master/LICENSE)

Inject or eject DLLs into/from Windows processes based on [`dll-syringe`](https://github.com/OpenByteDev/dll-syringe).

## Usage

```bash
dll-syringe inject --dll <PATH> (--process <NAME> | --pid <PID>)
dll-syringe eject --dll <PATH_OR_NAME> (--process <NAME> | --pid <PID>)
```
