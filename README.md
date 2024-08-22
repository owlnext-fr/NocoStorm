# NocoStorm

![GitHub Release](https://img.shields.io/github/v/release/owlnext-fr/nocostorm?style=for-the-badge&logo=github&label=Current%20release)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/owlnext-fr/nocostorm/.github%2Fworkflows%2Fmain.yml?style=for-the-badge&logo=githubactions)

⚡ Lightning fast, multi-threaded tool to push CSV data into NocoDB tables

This CLI tool is designed to upload CSV files data into NocoDB tables. It is a circumvention of the current limitation of the NocoDB API that only allows to upload CSV files with a maximum of 5MB.

This internally uses NocoDB API to upload data, so you need to have a NocoDB server running and an API token to use this tool.

Default options may be suitable for most use cases, but you can configure the tool to fit your needs with the available options.

> ⚠️ This tool is quite ressource intensive and may slow down your NocoDB server if you upload a large amount of data. Use it with caution.

## Features

- 🪶 Lightweight (less than 6MB)
- ⚡ Fast & multi-threaded
- 🔧 Configurable
- ✅ Easy to use

## Requirements

- **OS:** Linux (Sorry Windows and MacOS users, you can still use the source code to run the tool)
- **A NocoDB server** running or an instance of NocoDB Cloud.
- **An API token** to access the NocoDB API.
- **A CSV file** to upload.

## Installation

Check the [releases](https://github.com/owlnext-fr/nocostorm/releases) page to download the latest version of NocoStorm.

Unzip/Utar the downloaded file and run the executable.

## Usage

```man
nocostorm [OPTIONS] <FILE>
```

The CLI only take one argument, which is the path to the CSV file to upload, and will ask for other required information interactively.

If you want to use this CLI in other tools or scripts, you can use the available [options](#options) to configure the tool.

## Example

Basic example:

```bash
nocostorm ./my_big_data.csv \
    --nocodb-base-url=https://my.noco.instance.fr \
    --nocodb-api-token=aaa \
    --nocodb-table-id=bbb \
```

Complete example:

```bash
nocostorm ./my_big_data.csv \
    --nocodb-base-url=https://my.noco.instance.fr \
    --nocodb-api-token=aaa \
    --nocodb-table-id=bbb \
    --parallel-jobs=4 \
    --chunk-size=1000 \
    --csv-separator=',' \
    --csv-quote-separator='"' \
    --use-windows-foramt=false \
    --vv
```

## Options

| Option                        | Description                                                    | Default                                     |
| ----------------------------- | -------------------------------------------------------------- | ------------------------------------------- |
| `-b`, `--nocodb-base-url`     | The base URL of the NocoDB server                              | Will be asked interactively if not provided |
| `-k`, `--nocodb-api-token`    | The API token to access the NocoDB API                         | Will be asked interactively if not provided |
| `-t`, `--nocodb-table-id`     | The ID of the table to upload data to                          | Will be asked interactively if not provided |
| `-j`, `--parallel-jobs`       | The number of parallel jobs to run                             | `4`                                         |
| `-c`, `--chunk-size`          | The number of rows to upload per chunk                         | `1000`                                      |
| `-s`, `--csv-separator`       | The separator used in the CSV file                             | `,`                                         |
| `-u`, `--csv-quote-separator` | The quote separator used in the CSV file                       | `"`                                         |
| `-w`, `--use-windows-foramt`  | Use Windows encoding for the CSV file (useful for excel files) | `false`                                     |

## Contributing

If you want to contribute to this project, please read the [CONTRIBUTING.md](CONTRIBUTING.md) file.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
