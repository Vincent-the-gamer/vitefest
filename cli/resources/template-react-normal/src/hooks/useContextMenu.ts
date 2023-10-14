import useChrome from "./useChrome"

interface ContextMenuItem {
    type?: string       // 菜单类型，默认normal
    id: string,
    title: string,
    contexts: string[],
    parentId?: string,  // 二级菜单需要标注父级id
}

export default function useContextMenu(){
    const chrome = useChrome()

    // 点击事件对象
    const onClicked = chrome.contextMenus.onClicked

    function createMenu(contextMenuItem: ContextMenuItem){
        chrome.contextMenus.create(contextMenuItem)
    }

    return {
        createMenu,
        onClicked
    }
}