import Head from "next/head";
import "emoji-mart/css/emoji-mart.css";
import { Picker } from "emoji-mart";
import { external } from "../utils/utils";

export default function IndexPage() {
  function onSelect(emoji: any) {
    console.log("Emoji selected");
    external.invoke(`COPY_EMOJI ${emoji.native}`);
  }

  return (
    <main>
      <Head>
        <title>Emoji Picker</title>
      </Head>
      <div className='picker'>
        <Picker set='apple' onSelect={onSelect} native={false} />
      </div>
      <style jsx>{`
        main {
          display: flex;
          flex-direction: column;
          align-items: center;
          justify-content: center;
          height: 100vh;
          width: 100vw;
        }
      `}</style>
    </main>
  );
}
