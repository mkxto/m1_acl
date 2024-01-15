import { Link, useRouteError } from "react-router-dom";

export default function Error() {

  const error = useRouteError();
  return (
    <div className="flex flex-col justify-center items-center h-screen">
      <Link to="/" className="border w-fit p-2 rounded-md bg-black/20 hover:bg-black/50 m-4 backdrop-blur-sm border-white/20">
        🏠 <span className="hidden md:inline-block">Menu</span>
      </Link>
      <p className="text-red-500">Error:</p><p>{error.message}</p>
    </div>
  );
}
