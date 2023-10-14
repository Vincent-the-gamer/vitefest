/**
 * Badge
 */
import useChrome from "./useChrome"

type BackgroundColor = string | Array<number>

export default function useBadge(text: string, backgroundColor: BackgroundColor){
    const chrome = useChrome()

    function showBadge(){
        chrome.action.setBadgeText({
            text,
        })
        chrome.action.setBadgeBackgroundColor({
            color: backgroundColor
        })
    }
    
    function hideBadge() {
        chrome.action.setBadgeText({
            text: ""
        }),
        chrome.action.setBadgeBackgroundColor({
            color: [0, 0, 0, 0]
        })
    }

    return {
        showBadge,
        hideBadge
    }
}