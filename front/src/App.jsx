import './App.css'

import { greet } from "back"

function App() {

  return (
    <div className="bg-[url('./background.png')] bg-cover h-screen text-center flex flex-col justify-center">
      <h1 className="font-tinybox text-7xl text-amber-400 pb-8">Pixel Belote</h1>
      <div className="flex flex-col items-center justify-center space-y-4 *:shadow hover:*:bg-black/20 *:rounded-lg *:text-3xl *:font-mc *:border *:py-4 *:max-w-2xl *:w-full *:backdrop-blur-sm">
        <button onClick={() => {greet("alex")}}>Play</button>
        <button>Rules</button>
        <button>Settings</button>
      </div>
    </div>

  )
}

export default App;
