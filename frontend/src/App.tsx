import { useState } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import Home from './actions/views/homepage/Home.tsx'

function App() {
  const [count, setCount] = useState(0)

  return (
    <div class="dark">
      <Home/>
    </div>
  )
}

export default App
