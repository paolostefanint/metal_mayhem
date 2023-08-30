import {Link, useNavigate} from "solid-app-router";
import logo from "../assets/logo.png";
import {useAuthDispatch, useAuthState} from "../shared/context/auth.context";
import {createEffect} from "solid-js";

const Home = () => {
  const authDispatch = useAuthDispatch();
  const authState = useAuthState();
  const navigate = useNavigate();

  createEffect(() => {
    if (authState?.isAuthenticated) {
      navigate("/battle", {replace: true});
    }
  })

  return (
    <div class="h-full w-full flex flex-col justify-center items-center">
      <img src={logo} alt="logo" class={"h-[5em]"}/>

      <h1 class="text-2xl text-white text-center my-10">Are <span class={"text-orange-400"}>y</span>ou ready to <span class={"text-orange-400"}>s</span>pray?</h1>

      <button onClick={authDispatch?.login} class={"h-[3em] mt-3 w-full acquamarine-button"}>
        Accedi con i social
      </button>

      <div class={"text-white overflow-y-auto"}>
        <p>
           Un testo
        </p>
      </div>
    </div>
  )
}

export default Home;
