import useChrome from "@/hooks/useChrome";
import useContextMenu from "@/hooks/useContextMenu";

const chrome = useChrome()
const { createMenu, onClicked } = useContextMenu()

// 切换或打开选项卡
function openTab(url: string){
    chrome.tabs.query({ url }, (tabs: any) => {
        if(tabs.length){
            chrome.tabs.update(tabs[0].id, { active: true })
        } else {
            chrome.tabs.create({ url })
        }
    })
}

// 一级菜单
createMenu({
    id: "reactCrx",
    title: "ReactCrx",
    contexts: ["page"]
})


// 二级菜单
createMenu({
    id: "options",
    title: "⚙️插件设置",
    parentId: "reactCrx",
    contexts: ["page"]
})


// 点击事件
const optionsUrl = `chrome-extension://${chrome.runtime.id}/options.html`
onClicked.addListener((menu: any) => {
    // 插件设置： 打开options页面
    switch(menu.menuItemId) {
        case "options":
            openTab(optionsUrl)
            break
        default:
            break
    }
})