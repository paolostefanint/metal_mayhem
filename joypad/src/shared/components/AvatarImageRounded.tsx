import {createSignal, Show} from "solid-js";
import {PlayerDetail} from "../../models/user";

interface Props {
  player: PlayerDetail
}

let avatarImageMap = [ 
    "https://cdn.sofifa.com/players/158/023/21_60.png",
    "https://cdn.sofifa.com/players/158/023/21_60.png",
    "https://cdn.sofifa.com/players/158/023/21_60.png",
];

const AvatarImageRounded = (props: Props) => {
  const { player } = props;
  const [ imgError, setImgError ] = createSignal(false);
  const commonClasses = "w-[50px] h-[50px] mr rounded-full border-2 border-black";


  return (
    <>
        <img src={avatarImageMap[parseInt(player.avatar)]} alt=""
             class={commonClasses}
             onError={() => setImgError(true)}
        />
    </>
  )
}

export default AvatarImageRounded;
