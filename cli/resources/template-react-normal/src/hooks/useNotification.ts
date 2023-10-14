/**
 * 需要在manifest.json中, permissions数组里面增加notifications
 */
import useChrome from "./useChrome";

interface Options {
    type: string,                                     // 类型
    title: string,                                    // 标题
    message: string                                   // 内容
    contextMessage?: string,                          // 备选内容
    iconUrl?: string,                                 // 图标url
    imageUrl?: string,                                // "image"类型的通知的图片的URL
    priority?: number,                                // 优先级，有效范围[-2,2]，默认0
    eventTime?: number                                // 通知的时间戳，单位ms
    items?: { title: string, message: string }[],     // 该数组的每个元素代表一个通知，每个通知对象都有title和message两个属性
    progress?: number,                                // 当前的进度，有效范围[0, 100]
    isClickable?: boolean,                            // 通知窗口是否响应点击事件
    requireInteraction?: boolean                      // 是否需要交互关闭通知, false即为几秒后自动滑动离场。 PS: clear函数清除通知没有动画  
}

// 通知id可以用nanoid随机生成
export default function useNotification() {
    const chrome = useChrome()

    function create(notificationId: string, options: Options, callback: Function | undefined = undefined){
       callback ? chrome.notifications.create(notificationId, options, callback) 
                : chrome.notifications.create(notificationId, options)
    }

    function clear(notificationId: string, callback: Function | undefined = undefined){ 
        callback ? chrome.notifications.clear(notificationId, callback)
                 : chrome.notifications.clear(notificationId)
    }

    return {
        create,
        clear
    }
}