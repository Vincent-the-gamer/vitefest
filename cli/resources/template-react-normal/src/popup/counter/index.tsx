import { useState } from "react"

import "./counter.styl"

export default function Counter() {
  const [count, setCount] = useState<number>(0)

  return (
    <div className="counter">
        <button onClick={ () => setCount(count => count + 1) }>
            Count is: { count }
        </button>
    </div>
  )
}
