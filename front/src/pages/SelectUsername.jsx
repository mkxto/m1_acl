import { useState } from 'react';
import { Link, useNavigate } from "react-router-dom";

export default function SelectUsername() {
  const [username, setUsername] = useState('');
  const navigate = useNavigate();

  const handleInputChange = (event) => {
    setUsername(event.target.value);
  };

  const handleFormSubmit = (event) => {
    event.preventDefault();
    navigate(`/game?username=${username}`, { replace: true });
  };

  return (
    <>
      <form onSubmit={handleFormSubmit}>
        <div className="flex flex-col justify-center items-center h-screen bg-[#387438] text-[#DAD7CD]">
          <label htmlFor="username" className="block text-sm font-medium leading-6">
            Username
          </label>
          <div className="flex items-center space-x-4">
            <div>
              <input
                id="username"
                name="username"
                type="text"
                value={username}
                onChange={handleInputChange}
                className={`block w-full rounded-md border-0 py-1.5 bg-[#69ac69] text-black shadow-sm ring-1 ring-inset placeholder:text-black/60 focus:ring-2 focus:ring-inset sm:text-sm sm:leading-6 ${username.trim() === '' ? 'border-red-500' : ''}`}
                placeholder="Enter your username"
              />
            </div>
            <button
              type="submit"
              className="px-4 py-2 bg-[#3fad53] text-white rounded-md shadow-sm hover:bg-black/30 focus:outline-none focus:ring-2 focus:ring-offset-2 disabled:bg-gray-400 disabled:cursor-not-allowed "
              disabled={username.trim() === ''}
            >
              Validate
            </button>
            <Link to="/" className="border w-fit p-2 rounded-md bg-red-500/50 hover:bg-red-500/20 top-0 left-0 m-4 backdrop-blur-sm border-white/20">
          ❌ <span className="hidden md:inline-block">Abort</span>
        </Link>
          </div>
          {username.trim() === '' && <p className="text-red-500 text-sm mt-1">Username cannot be empty</p>}
        </div>

      </form>

    </>
  );
}
