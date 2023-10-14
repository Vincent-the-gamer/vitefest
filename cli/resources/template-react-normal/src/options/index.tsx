import { ChangeEvent, useEffect, useState } from "react"
import "./options.styl"
import useChrome from "@/hooks/useChrome"
import { DEFAULT_LIMIT } from "@/global/constants"


export default function Options() {
  const chrome = useChrome()

  const [limit, setLimit] = useState<number>(DEFAULT_LIMIT)
  const [oldLimit, setOldLimit] = useState<number>(DEFAULT_LIMIT)

  const [showMessage, setShowMessage] = useState<boolean>(false)
  const [syncSuccess, setSyncSuccess] = useState<boolean>(false)


  useEffect(() => {
    chrome.storage.sync.get("limit", (result: any) => {
      if(result.limit){
        let limitNum: number = parseInt(result.limit)
        setOldLimit( limitNum )
        setLimit( limitNum )
      }
    })
  }, [])

  function changeLimit(event: ChangeEvent<HTMLInputElement>){
    let value: string = event.target.value
    let numValue = parseInt(value)
    setLimit(numValue)
  }

  function updateLimit(){
    if(isNaN(limit)){
      setSyncSuccess(false)
      messageShow()
      return
    } 
    chrome.storage.sync.set( { limit } )
    setOldLimit(limit)
    setSyncSuccess(true)
    messageShow()
  }

  function messageShow(){
    setShowMessage(true)
    setTimeout(() => {
      setShowMessage(false)
    }, 5000)
  }

  return (
    <div className="options">
        <h1>Options Page Demo</h1>
        <p>
          <h2>Old Limit: { oldLimit }</h2>
          <h2>New Limit: { limit }</h2>
          <div>
            <h3>Set Limit:</h3>
            <input type="number" value={ limit } onChange={ e => changeLimit(e) } />
            <button onClick={ updateLimit }>Sync</button>
            { 
              showMessage && (
                <p className={ syncSuccess ? "ok" : "err"}>
                  { syncSuccess ? "Sync succeeded!!" : "limit is invalid!!"}
                </p>
              )
            }
          </div>
        </p>
    </div>
  )
}
