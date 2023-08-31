import Close from "../icons/Close";
import {useUiDispatch} from "../context/ui.context";

const Instructions = () => {
  const uiDispatch = useUiDispatch();

  const close = () => {
    uiDispatch?.toggleInstruction();
  }

  return (
    <div class={"instructionModal fixed z-50 inset-0 px-5 py-10 flex flex-col text-white bg-[url('/src/assets/bg.jpg')] bg-no-repeat bg-cover"}>

      <div class={"flex justify-between mb-10 items-center"}>
        <p class={"text-2xl"}>Istruzioni di gioco</p>
        <button onClick={close}>
          <Close/>
        </button>
      </div>

      <div class={"overflow-y-auto"}>

<p>Distruggi i tuoi avversari  in una lotta senza tregua!</p>

      </div>

      <button onClick={close} class={"mt-auto h-[3em] mt-3 w-full acquamarine-button"}>
        <Close/>&nbsp; Chiudi
      </button>
    </div>
  )
}

export default Instructions;
