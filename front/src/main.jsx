// import React from 'react'
import ReactDOM from 'react-dom/client'
import {
  createBrowserRouter,
  RouterProvider,
} from "react-router-dom";

import App from './App.jsx'
import './index.css'
import Leaderboard from './pages/Leaderboard.jsx'; 
import Error from './pages/Error.jsx';
import SelectUsername from './pages/SelectUsername.jsx';
import Game from './pages/Game.jsx';

const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
    errorElement: <Error />,
  },
  {
    path: "/leaderboard",
    element: <Leaderboard />,
    errorElement: <Error />,
  },
  {
    path: "/select-username",
    element: <SelectUsername />,
    errorElement: <Error />,
  },
  {
    path: "/game",
    element: <Game />,
    errorElement: <Error />,
  },
]);

ReactDOM.createRoot(document.getElementById("root")).render(
  <RouterProvider router={router} />
);
