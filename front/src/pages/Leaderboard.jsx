import { useEffect, useState } from 'react';
import { Link } from 'react-router-dom';

export default function Leaderboard() {
  const [leaderboardData, setLeaderboardData] = useState([]);
  const [isLocalStorageEmpty, setIsLocalStorageEmpty] = useState(false);
  const [currentPage, setCurrentPage] = useState(1);
  const [usersPerPage] = useState(15);

  useEffect(() => {
    const storageItem = localStorage.getItem("leaderboardData");
    if (!storageItem) {
      setIsLocalStorageEmpty(true);
      return;
    }
    try {
      const data = JSON.parse(storageItem) || [];
      const sortedData = data.sort((a, b) => a.score - b.score);
      setLeaderboardData(sortedData);
      setIsLocalStorageEmpty(data.length === 0);
    } catch (error) {
      console.error("Error parsing leaderboard data:", error);
      setLeaderboardData([]);
      setIsLocalStorageEmpty(true);
    }
  }, []);

  const indexOfLastUser = currentPage * usersPerPage;
  const indexOfFirstUser = indexOfLastUser - usersPerPage;
  const currentUsers = leaderboardData.slice(indexOfFirstUser, indexOfLastUser);

  const totalPages = Math.ceil(leaderboardData.length / usersPerPage);
  const maxPageNumbers = 5;
  const halfMaxPageNumbers = Math.floor(maxPageNumbers / 2);
  let startPage = Math.max(currentPage - halfMaxPageNumbers, 1);
  let endPage = Math.min(startPage + maxPageNumbers - 1, totalPages);

  if (endPage - startPage < maxPageNumbers - 1) {
    startPage = Math.max(endPage - maxPageNumbers + 1, 1);
  }

  const pageNumbers = Array.from({ length: endPage - startPage + 1 }, (_, i) => startPage + i);

  const paginate = (pageNumber) => setCurrentPage(pageNumber);

  const goToNextPage = () => {
    if (currentPage < totalPages) {
      setCurrentPage(currentPage + 1);
    }
  };

  const goToLastPage = () => {
    setCurrentPage(totalPages);
  };

  const goToPreviousPage = () => {
    if (currentPage > 1) {
      setCurrentPage(currentPage - 1);
    }
  };

  const goToFirstPage = () => {
    setCurrentPage(1);
  };

  return (
    <>
      <div className="h-screen bg-[#588157] text-[#DAD7CD] bg-center bg-cover flex flex-col justify-center items-center">
        <Link to="/" className="absolute border w-fit p-2 rounded-md bg-black/20 hover:bg-black/50 top-0 left-0 m-4 backdrop-blur-sm border-white/20">
          🏠 <span className="hidden md:inline-block">Menu</span>
        </Link>
        <h1 className="text-3xl md:text-5xl font-bold m-5">Leaderboard</h1>
        <div className="flex flex-col xl:border xl:rounded-md p-4 bg-[#476847] w-full max-w-7xl h-full md:max-h-[80%]">
          {isLocalStorageEmpty ?

            <div className="text-center mt-auto mb-auto">
              <p className="text-2xl text-red-400">No data to display</p>
              <p>Maybe you should play a game first!</p>
            </div>

            :

            <>
              <div className="overflow-auto w-full">

                <div className="flex flex-row mx-0 md:mx-20">

                  <div className="flex flex-col bor">
                    <div className="p-2 font-bold">Username</div>
                    {currentUsers.map((user, index) => (
                      <div key={index + "_username"} className="flex">
                        <div className="p-2">{user.username}</div>
                      </div>
                    ))}
                  </div>

                  <div className="flex ml-auto flex-col">
                    <div className="p-2 font-bold">Score</div>
                    {currentUsers.map((user, index) => (
                      <div key={index + "_score"} className="flex">
                        <div className="p-2">{user.score}</div>
                      </div>
                    ))}
                  </div>

                </div>

              </div>

              <div className="mt-auto flex flex-row justify-center space-x-2 border p-2 rounded-md my-2 select-none text-[#beb8a3]">
                <button
                  onClick={goToFirstPage}
                  disabled={currentPage === 1}
                  className={`${currentPage === 1 ? 'cursor-not-allowed opacity-50' : ''
                    }`}
                >
                  «
                </button>
                <button
                  onClick={goToPreviousPage}
                  disabled={currentPage === 1}
                  className={`${currentPage === 1 ? 'cursor-not-allowed opacity-50' : ''
                    }`}
                >
                  ‹
                </button>

                {pageNumbers.map((pageNumber) => (
                  <button
                    key={pageNumber}
                    onClick={() => paginate(pageNumber)}
                    className={`${currentPage === pageNumber ? 'font-bold text-[#A3B18A] border-b-2' : ''}`}
                  >
                    {pageNumber}
                  </button>
                ))}

                <button
                  onClick={goToNextPage}
                  disabled={currentPage === totalPages}
                  className={`${currentPage === totalPages ? 'cursor-not-allowed opacity-50' : ''
                    }`}
                >
                  ›
                </button>
                <button
                  onClick={goToLastPage}
                  disabled={currentPage === totalPages}
                  className={`${currentPage === totalPages ? 'cursor-not-allowed opacity-50' : ''
                    }`}
                >
                  »
                </button>
              </div>
            </>}
        </div>
      </div>
    </>
  );
}
