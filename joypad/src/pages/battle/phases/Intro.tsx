import { Component, createSignal } from "solid-js";

interface IntroProps {
    onJoin: (selectedCharacter: number) => void;
}

const Intro: Component<IntroProps> = (props) => {
    const { onJoin } = props;
    const [selectedCharacter, setSelectedCharacter] = createSignal(0);

    return (
        <div class={"text-white"}>
<p class={"text-center text-2xl mb-3"}>Pronto ad uccidere?</p>
            <p>
                Seleziona il tuo personaggio per entrare nella stanza di attesa!
            </p>

            <div class={"grid grid-cols-3 gap-4 mt-10"} >
                <div class={"col-span-1"} onClick={() => setSelectedCharacter(1)} classList={{"selected": selectedCharacter() === 1}}>
                    <p>Character 1</p>
                    <img src={"./assets/characters/1.png"} />
                </div>
                <div class={"col-span-1"} onClick={() => setSelectedCharacter(2)} classList={{"selected": selectedCharacter() === 2}}>
                    <p>Character 2</p>
                    <img src={"./assets/characters/2.png"} />
                </div>
                <div class={"col-span-1"} onClick={() => setSelectedCharacter(3)} classList={{"selected": selectedCharacter() === 3}}>
                    <p>Character 2</p>
                    <img src={"./assets/characters/3.png"} />
                </div>
            </div>


            <div class={"mt-3"} classList={{hidden: selectedCharacter() === 0}}>
                <div class={""} classList={{hidden: selectedCharacter() !== 1}}>
                    Descrizione avatar 1
                </div>
                <div class={""} classList={{hidden: selectedCharacter() !== 2}}>
                    Descrizione avatar 2
                </div>
                <div class={""} classList={{hidden: selectedCharacter() !== 3}}>
                    Descrizione avatar 3
                </div>   
            </div>

            <button
                disabled={selectedCharacter() === 0}
                class={"h-[3em] mt-10 w-full acquamarine-button disabled:opacity-50"}
                onClick={() => onJoin(selectedCharacter())}
            >
                Join!
            </button>
        </div>
    );
};

export default Intro;
