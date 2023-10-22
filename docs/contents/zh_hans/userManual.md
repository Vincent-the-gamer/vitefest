---
layout: doc
title: "使用说明书"
lastUpdated: true
---

# 使用说明书

## 如何创建你的工程( Manifest V3标准的Chrome插件 + Vite引擎加持 )

你可以选择不同的方式！

### 使用命令行工具

1. 从GitHub下载命令行工具:

[https://github.com/Vincent-the-gamer/vitefest/releases](https://github.com/Vincent-the-gamer/vitefest/releases)

2. 打开终端
```shell
# 切换到你的命令行工具所在目录
cd /path/to/your/cli/

# 创建一个包含popup, options, background, content四个组成部分的工程
vitefest-cli create normal

# 创建一个仅有popup, options的简易工程
vitefest-cli create lite

# 使用--name参数来自定义工程名字，否则使用默认名称
vitefest-cli create normal --name my-project
```

接下来，选择一个框架:
1. React
2. Vue  (后续添加)
3. Svelte  (后续添加)
4. Solid  (后续添加)
5. Qwik  (后续添加)

![cli-framework](/imgs/cli-framework.png)


::: warning 警告
macOS/Linux环境下，如果提示command not found, 使用`./vitefest-cli` 代替 `vitefest-cli`即可。
你也可以配置环境变量(添加路径到PATH)来让命令行工具可以在全局环境下使用`vitefest-cli`。
:::

3. 躁起来!

打开工程，安装依赖，然后开始整活吧！

![rock1](/imgs/rock1.png)

![rock2](/imgs/rock2.png)

### 使用模板

从GitHub Release下载模板:

[https://github.com/Vincent-the-gamer/vitefest/releases](https://github.com/Vincent-the-gamer/vitefest/releases)

然后一切就已经准备就绪!

## 如何使用该模板进行Chrome插件开发？
请查看[开发指南](./devGuide.md)