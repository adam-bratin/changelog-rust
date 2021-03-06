# Changelog-Rust

This tool is written in rust to be light weight and portable cli for the management and generation of chagelogs for software realease. The idea is to create a way for developers to maintain an up to date changelog with minimal overhead and great integration into CI/CD (agnostic tool that can be used in any CI/CD solution). The idea is for developers to create change files for every story/issue/task/bug/feature (depending of planning tooling the name differs). This prevents uglier solutions of messy git conflict when all developers change a single changelog file. Allows for nice auto generation of changelog file based on predefined template

## Installation

### For Rust Users

run cmd: `cargo install changelog-rust `

### Use installation scripts

#### Linux or OSX

`wget -O - https://raw.githubusercontent.com/adam-bratin/changelog-rust/main/install/unix.sh | sh` </br>
or </br>
`curl -s http://example.com/script.sh | sh`

#### Windows

`powershell -Command {iwr -useb https://raw.githubusercontent.com/adam-bratin/changelog-rust/main/install/windows.ps1 | iex}`

### Manual Installation

Go to the downloads page for the latest release at: https://github.com/adam-bratin/changelog-rs/releases/latest
Download correct version of OS. </br>
**NOTE** You may neeed to give the executable execute permissions

## Usage

```bash
changelog

USAGE:
    changelog-rust [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <config>        path to config file [default: ./.changelogrc]

SUBCOMMANDS:
    generate    generate chagefile for PR
    help        Prints this message or the help of the given subcommand(s)
    init        initialize repo with setup for changelog cli
    merge       merges all the change files into an changelog from template
```

### Init

The init command is used to setup a repo to use the cli tool

- It creates the the folder where the change files
- It creates a default template file for the changelog
- It generate a .changelogrc file with settings for the cli (for more info see [Config File](#config-file))

```bash
USAGE:
    changelog-rust init [OPTIONS] --appName <app-name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --appName <app-name>    name of app
    -i <input>                  path where change files will be located [default: ./changes/]
    -t <template>               path to changelog template file to be generated [default: ./CHANGELOG.md.hbs]
```

### Generate

The generate command is used to create a change file for json spec see [Changefile schmea](#changefile-schema)

If the output directory does not exist it is automatically create for you.

It is possible to create a change file non interactive by passing the -t and -d flags see below for more info.

```bash
USAGE:
    changelog-rust generate [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d <description>        optional description for change (for automated generation)
    -t, --type <kind>       optional change type (for automated generation)
    -o <output>             path to output change file [default: ./changes/]
```

### Merge

The merge command is uesed to generate a changelog by parsing all the change files and applying them to the teplate file. For more info on how to customize the teplate file see [Template File](#template-file)

```bash
USAGE:
    changelog-rust merge [FLAGS] [OPTIONS]

FLAGS:
    -d, --delete     whether to delete change files after changelog is created
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i <input>           path to input change files [default: ./changes/]
    -o <output>          path to output changelog [default: ./CHANGELOG.md]
    -t <template>        path to changelog template file [default: ./CHANGELOG.md.hbs]
    -v <version>         version of current release [env: VERSION=]  [default: 0.0.1]
```

### Changefile schema

```js
{
  "date": "05/03/2021", // DD/MM/YYY
  "author": "John Smith john.smith@gmail.com", // <git User.name> <git User.email>
  "label": "Feature", // of type Section for more info see [Config File](#config-file)
  "description": "- Added new feature for release" // automatically add bullet to beginning of description
}
```

### Config File

schema:

```js
{
  "name": "changelog-rust", // this is the name of your application
  "extra_commit_args": ["--no-verify"], // this is optional if you need to skip git hooks
  "sections": [] // this is the list of change types that is used to generate sections in changelog
}
```

### Template File

Below is the default changelog template:

```markdown
# Release Notes {{name}} Version {{versionNoV}}

{{date}}

## Changes

### Features

{{#Feature}}
{{description}} - by {{author}} on {{date}}
{{/Feature}}

### Bugfixes

{{#BugFix}}
{{description}} - by {{author}} on {{date}}
{{/BugFix}}

### Other changes

{{#Other}}
{{description}} - by {{author}} on {{date}}
{{/Other}}
```

the data accessible to the template is: (This is based on default from init command)

```js
{
  "date": "05/03/2021", // date string in this format
  "versionNoV": "1.0.0", // the version string passed in without a v at front
  "version": "v1.0.0", // version string passed in with v at front
  "name": "changelog-rust", // application name from .changelogrc
  // There is an entry for each Section from the .changelogrc
  "Feature": [
      {
        "date": "03/03/2021",
        "author": "John Smith john.smith@gmail.com",
        "label": "Feature",
        "description": "- Added new feature for release"
    }
  ],
  "BugFix": [],
  "Other": []
}
```

The entry for each sections is the json object of the change file you can access
any property on the change file based on handle bars syntax as shown in example above

The output with the above template and data is:

```markdown
# Release Notes changelog-rust Version 1.0.0

05/03/2021

## Changes

### Features

- Added new feature for release by John Smith john.smith@gmail.com on 03/03/2021

### Bugfixes

### Other changes
```
