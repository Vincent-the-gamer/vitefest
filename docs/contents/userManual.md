---
layout: doc
title: "User Manual"
lastUpdated: true
---

# User Manual

## How to create your project(Chrome Extension Manifest V3 + Vite)

You can choose several ways:

### Use CLI

1. Download CLI from GitHub:

[https://github.com/Vincent-the-gamer/vitefest/releases](https://github.com/Vincent-the-gamer/vitefest/releases)

2. Open Your Terminal
```shell
# enter the path to your cli
cd /path/to/your/cli/

# create a project including popup, options, background, content
vitefest-cli create normal

# create a project including popup, options.
vitefest-cli create lite

# you can customize your project name using "--name", 
# or a default name if you don't give one.
vitefest-cli create normal --name my-project
```

::: warning
For macOS/Linux, if encounter "command not found", please use `./vitefest-cli` instead of `vitefest-cli`.
You may add the path into your `PATH` environment variable to make it global, and use `vitefest-cli` anywhere.
:::

3. Let's Rock!

Open your project, install dependencies and happy hacking!

### Use Template

Download template from release:

[https://github.com/Vincent-the-gamer/vitefest/releases](https://github.com/Vincent-the-gamer/vitefest/releases)

Then open your project and you are all set!

## How to develop Chrome Extension with this template?

See [Development Guide](./devGuide.md)