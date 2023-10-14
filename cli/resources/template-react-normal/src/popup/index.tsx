import "./popup.styl"
import { useEffect, useState } from "react"
import { nanoid } from "nanoid"

import Counter from "@/popup/counter"

import { DEFAULT_LIMIT } from "@/global/constants"

import ViteLogo from "@/assets/images/vite.png"
import ReactLogo from "@/assets/images/react.png"
import ManifestLogo from "@/assets/images/manifest.png"

import useNotification from "@/hooks/useNotification"
import useChrome from "@/hooks/useChrome"
import useBadge from "@/hooks/useBadge"

export default function Popup() {
  const [amount, setAmount] = useState<number>(0)
  const [total, setTotal] = useState<number>(0)
  const [limit, setLimit] = useState<number>(DEFAULT_LIMIT)

  const chrome = useChrome()
  const { create } = useNotification()

  useEffect(() => {
    chrome.storage.sync.get("total", (result: any) => {
      if(result.total){
        setTotal(
          parseInt(result.total)
        )
      } else {
        clearTotal()
      }
    })

    chrome.storage.sync.get("limit", (result: any) => {
      if(result.limit){
        setLimit(result.limit)
      } else {
        chrome.storage.sync.set( { limit: DEFAULT_LIMIT } )
      }
    })
  },[])

  function add(){
    chrome.storage.sync.get("total", (result: any) => {
      let newTotal: number = 0;
      if (result.total) {
        newTotal += parseInt(result.total)
      }

      if(amount) {
        if(newTotal + amount > limit){
          // 测试通知
          let id = nanoid()
          create(id, {
            type: "basic",
            iconUrl: "images/ashley.png",
            title: "Limit Exceeded!",
            message: "You have exceeded the limit!",
            requireInteraction: false
          })
        } else {
          newTotal += amount
          chrome.storage.sync.set( { total: newTotal } )      
          setTotal(newTotal)
        }
      }
    })
  }

  function clearTotal(){
    chrome.storage.sync.set( { total: 0 } )
    setTotal(0)
  }


  /**
   * badge
   */
  const { showBadge, hideBadge } = useBadge("PS5", "deeppink")


  return (
    <div className="popup">
        <div className="img-area">
            <a href="https://cn.vitejs.dev/" title="https://cn.vitejs.dev/"
               target="_blank">
              <img src={ ViteLogo } id="vite" alt="vite"/>
            </a>
            <a href="https://react.docschina.org/" title="https://react.docschina.org/"
               target="_blank">
              <img src={ ReactLogo } id="react" alt="react"/>
            </a>
            <a href="https://doc.yilijishu.info/chrome/mv3-overview.html" title="https://doc.yilijishu.info/chrome/mv3-overview.html"
               target="_blank">
              <img src={ ManifestLogo } id="manifest" alt="manifest"/>
            </a>
        </div>
        <h1>Vite + React + Manifest V3</h1>
        <h3>Develop chrome extension in React.</h3>
        <p>
            <Counter/>
        </p>
        <br/>
        <div className="test">
            <h2 className="title">Chrome storage test</h2>
            <h3>Total: { total }</h3>
            <h3>Limit: { limit }</h3>
            <br/>
            <h3>Input number: </h3>
            <input type="number" value={ amount }
                   onChange={ e => setAmount(parseInt(e.target.value)) }/>
            <p>
              <button onClick={ add }>Add</button>
              <button onClick={ clearTotal }>Clear Total</button>
            </p>
            <br/>
            <h2 className="title">Badge test</h2>
            <p>
              <button onClick={ showBadge }>Show Badge</button>
              <button onClick={ hideBadge }>Hide Badge</button>
            </p>
        </div>

        <p className="ps">
          <h3>Set limit in options page. Open options page from right click menu.</h3>
          <h3 id="author">
            <a href="https://github.com/Vincent-the-gamer"
               target="_blank" 
               title="Go to my GitHub">@2023-present Vincent-the-gamer</a>
          </h3>
        </p>
    </div>
  )
}
