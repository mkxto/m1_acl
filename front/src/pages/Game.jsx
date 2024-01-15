import { useEffect, useState } from "react";
import { useSearchParams } from "react-router-dom";

import { play_whole_game } from "belote_back";

export default function Game() {
  let [searchParams, setSearchParams] = useSearchParams();
  let [username, setUsername] = useState("");
  const usr = searchParams.get("username");
  console.log(usr);
  if (!usr || usr.trim() === "")
    throw new Error("No username provided");
  useEffect(() => {
    setUsername(usr);
    console.log("Played game: ", play_whole_game(usr));
  }, [usr]);
  return (
    <div>
      <h1>{username}</h1>
    </div>
  );
}
