import fs from "fs"
import path from "path"

// 拷贝目录
const copyDirectory = (srcDir, destDir) => {
    // 判断目标目录是否存在，不存在则创建
    if (!fs.existsSync(destDir)) {
        fs.mkdirSync(destDir)
    }

    fs.readdirSync(srcDir).forEach((file) => {
        const srcPath = path.join(srcDir, file)
        const destPath = path.join(destDir, file)

        if (fs.lstatSync(srcPath).isDirectory()) {
            // 递归复制子目录
            copyDirectory(srcPath, destPath)
        } else {
            // 复制文件
            fs.copyFileSync(srcPath, destPath)
        }
    })
}

// 删除单个文件
const deleteFile = filePath => {
    fs.unlink(filePath, err => {
        if(err) throw err
    })
}


// 删除目录及文件
const deleteDirectory = (dir) => {
    if(fs.existsSync(dir)) {
        fs.readdirSync(dir).forEach((file) => {
            const curPath = path.join(dir, file)
            if (fs.lstatSync(curPath).isDirectory()) {
                // 递归删除子目录
                deleteDirectory(curPath)
            } else {
                // 删除文件
                fs.unlinkSync(curPath)
            }
        })
        // 删除空目录
        fs.rmdirSync(dir)
    }
}


fs.copyFile('crx-background/background.js', 'public/background.js', (err) => {
    if (err) throw err
});
fs.copyFile('crx-content/content.js', 'public/content.js', (err) => {
    if (err) throw err
});
deleteDirectory("crx-background")
deleteDirectory("crx-content")