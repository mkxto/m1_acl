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
    navigate(`/game?username=${username}`);
  };

  return (
    <>
      <form onSubmit={handleFormSubmit}>
        <div className="flex flex-col justify-center items-center h-screen">
          <label htmlFor="username" className="block text-sm font-medium leading-6 text-gray-900">
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
                className={`block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6 ${username.trim() === '' ? 'border-red-500' : ''}`}
                placeholder="Enter your username"
              />
            </div>
            <button
              type="submit"
              className="px-4 py-2 bg-indigo-600 text-white rounded-md shadow-sm hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:bg-gray-400 disabled:cursor-not-allowed "
              disabled={username.trim() === ''}
            >
              Send
            </button>
          </div>
          {username.trim() === '' && <p className="text-red-500 text-sm mt-1">Username cannot be empty</p>}

          <p className="mt-3 text-sm leading-6 text-gray-600">Enter the name of your choice.</p>
          <Link to="/" className="border w-fit p-2 rounded-md bg-red-400 hover:bg-black/50 m-4 backdrop-blur-sm border-white/20">
            Abort
          </Link>
        </div>

      </form>

    </>
  );
}
