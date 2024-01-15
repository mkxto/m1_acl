import { Fragment, useEffect, useState } from "react";
import { useSearchParams, Link } from "react-router-dom";
import { motion } from "framer-motion"

import { GameState } from "belote_back";

export default function Game() {
  const [searchParams, setSearchParams] = useSearchParams();
  const [username, setUsername] = useState("");
  const [gameState, setGameState] = useState(undefined);

  const [error, setError] = useState(undefined);

  const [lastScore, setLastScore] = useState("Non set");
  const [score, setScore] = useState("Non set");
  const [pickedCards, setPickedCards] = useState([]);
  const [remainingTurns, setRemainingTurns] = useState(5);

  const usr = searchParams.get("username");

  if (!usr || usr.trim() === "")
    throw new Error("No username provided");

  useEffect(() => {
    setUsername(usr);

    try {
      if (username) {
        const newGameState = new GameState(username);
        newGameState.reset();
        setGameState(newGameState);
      }
    } catch (e) {
      throw new Error(e);
    }
  }, [usr, username]);

  const pick = () => {
    if (remainingTurns === 0)
      return;
    try {
      gameState.pick();
      const cardId = gameState.get_last_two_cards_id();
      console.log("Picked: ", cardId);
      setPickedCards((prevCards) => [...prevCards, cardId[0], cardId[1]]);
      setRemainingTurns(gameState.get_remaining_turns());
      setScore(gameState.get_score());
      setLastScore(gameState.get_last_score());
      if (remainingTurns === 1) 
        gameState.save_game();
    } catch (e) {
      setError(e);
    }
  }

  const renderCardImage = (cardId) => {
    const suit = cardId[0];
    const value = cardId[1] === "1" ? "10" : cardId[1];
    const imagePath = `cards/${suit}${value}.png`;

    return (
      <motion.img
        className="sm:w-1/2 md:w-28 shadow-xl"
        src={imagePath}
        alt={cardId}
        initial={{ y: -100, opacity: 0 }}
        animate={{ y: 0, opacity: 1 }}
        transition={{ duration: 0.6 }}
        whileHover={{ y: -10, transition: { duration: 0.2 } }}
      />
    );
  };

  return (
    <div className="h-screen bg-[#387438] text-[#DAD7CD] p-8 flex justify-center">
      <div className="p-4 max-w-7xl border-4 border-black/20 flex flex-col items-center h-full w-full overflow-auto">
        { remainingTurns !== 0 && (
        <Link to="/" className="self-start border w-fit p-2 rounded-md bg-red-500/20 hover:bg-red-500/50 top-0 left-0 m-4 backdrop-blur-sm border-white/20">
          ❌ <span className="hidden md:inline-block">Abort</span>
        </Link>
        )}
        <h1 className="text-3xl md:text-5xl font-bold m-5">Game</h1>
        <span>Hello, <strong>{username}</strong></span>
        <span> You may click on the deck to pick two cards. </span>
        <div className="my-2">
          <motion.img
            whileHover={remainingTurns > 0 ? { scale: 1.1 } : null}
            whileTap={remainingTurns > 0 ? { scale: 0.9 } : null}
            className={`sm:w-1/2 md:w-32 mr-auto shadow-xl ml-auto ${remainingTurns === 0 ? "cursor-not-allowed" : "cursor-pointer"}`}
            src="cards/red_back.png"
            alt="deck"
            onClick={() => pick()}
          />
        </div>
        {error && <div className="text-center text-red-400">{error}</div>}
        {pickedCards.length > 0 && (
          <div className="flex flex-wrap justify-center bg-black/20 rounded-md p-4 w-full">
            {pickedCards.map((cardId, index) => (
              <Fragment key={cardId}>
                {renderCardImage(cardId)}
                {(index + 1) % 2 === 0 && <div className="w-4" />} {/* Add spacing after every two cards */}
              </Fragment>
            ))}
          </div>
        )}

        { remainingTurns === 0 && (
          <div className="mt-4">
          <span>
            The game is now over. You can go back to the menu or check the leaderboard.
          </span>
          <div className="flex justify-center mt-4 space-x-2">
            <Link to="/" className="border w-fit p-2 rounded-md bg-black/20 hover:bg-black/50 backdrop-blur-sm border-white/20">
              🏠 Menu
            </Link>
            <Link to="/leaderboard" className="border w-fit p-2 rounded-md bg-black/20 hover:bg-black/50 backdrop-blur-sm border-white/20">
              🥇 Leaderboard
            </Link>
          </div>
          </div>
        )}

        <div className="mt-auto text-center bg-black/10 rounded-md p-4 text-[#DAD7CD] w-full">
          <div>Your score: {score}</div>
          <div>Last score: {lastScore}</div>
          <div>Remaining turns: {remainingTurns}</div>
        </div>


      </div>
    </div>
  );
}
