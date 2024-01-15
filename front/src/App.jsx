import { useEffect } from 'react';
import './App.css'
import { Link } from "react-router-dom";

function App() {

  return (
    <div className="bg-[url('background.jpg')] bg-cover h-screen text-center flex flex-col justify-center">
      <h1 className="font-tinybox text-7xl text-[#344E41] pb-8">Pixel Belote</h1>
      <div className="flex flex-col items-center justify-center space-y-4 *:shadow hover:*:bg-black/20 *:rounded-lg *:text-3xl *:font-mc *:border *:py-4 *:max-w-2xl *:w-full *:backdrop-blur-3xl *:text-[#3A5A40]">
        <Link to="select-username">▶️ Play</Link>
        <Link to="leaderboard">🥇 Leaderboard</Link>
        <button onClick={() => {
            let customWindow = window.open('', '_self', '');
            customWindow.close();
        }}>🏃‍♂️ Leave</button>
      </div>
    </div>
  )
}

export default App;
