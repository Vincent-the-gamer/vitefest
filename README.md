<p align="center">
    <img src="./.github/imgs//vitefest.png" alt="vitefest" style="width: 100px;"/>
    <h1 align="center">Vitefest  [ˈvi:tɪfest]</h1>
</p>

<p align="center">Build your own Chrome Extension(Manifest V3) with modern frontend frameworks, powered by Vite.</p>
<p align="center" style="font-style: italic">Vite + Manifest = Vitefest</p>

# Installation

## Use CLI
Use `vitefest-cli`, a command line tool to create project fastly. This CLI was built in `Rust`.

Download `vitefest-cli` from release:

https://github.com/Vincent-the-gamer/vitefest/releases

Then use following command to create your template project of Chrome Extension.

```shell
# for macOS/Linux, if encounter "command not found", please use ./vitefest-cli instead of vitefest-cli

# create a project including popup, options, background, content
vitefest-cli create normal

# create a project including popup, options.
vitefest-cli create lite

# you can customize your project-name while creating.
vitefest-cli create normal --name my-project
```

## Use Template Pack
Directly download from release:

normal: `react-normal-extension-pack.zip`.

lite: `react-lite-extension-pack.zip`.

## Get from webpage
Coming soon.

# Future Plan
In version v1.0.0, `vitefest-cli` can only create `React` projects. Other framework will be added soon.

| Frameworks | Done |
|     --     | --   |
| React      |  ✅  |
| Vue        |      |
| Svelte     |      |
| Solid      |      |
| Qwik       |      |